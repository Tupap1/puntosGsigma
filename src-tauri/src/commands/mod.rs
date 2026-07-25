use tauri::{AppHandle, State, Manager};
use sqlx::Row;
use crate::db::{AppState, create_connection_pool, init_db_tables};
use crate::models::{Customer, DbConfig, LoyaltyConfig, PointSummary, PointTransaction};

fn get_config_file_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    let config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Error al obtener directorio de configuración: {}", e))?;
    std::fs::create_dir_all(&config_dir).ok();
    Ok(config_dir.join("db_config.json"))
}

#[tauri::command]
pub async fn check_db_connection(
    config: Option<DbConfig>,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let cfg = if let Some(c) = config {
        c
    } else {
        let guard = state.db_config.read().await;
        guard.clone()
    };

    let pool = create_connection_pool(&cfg).await?;
    
    // Execute ping query
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| format!("Error en prueba SELECT 1: {}", e))?;

    // Initialize tables if needed
    init_db_tables(&pool).await?;

    // Update active pool & config in state
    {
        let mut pool_guard = state.pool.write().await;
        *pool_guard = Some(pool);
    }
    {
        let mut cfg_guard = state.db_config.write().await;
        *cfg_guard = cfg;
    }

    Ok(true)
}

#[tauri::command]
pub async fn search_customers(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Customer>, String> {
    let pool = state.get_pool().await?;
    let search_term = format!("%{}%", query.trim());

    // Native POS table: READ ONLY (SELECT ONLY)
    let rows = sqlx::query(
        r#"
        SELECT TRCID, TRCNUMDOC, TRCNOM, TRCAPE, TRCTEL1, trcema1
        FROM trc
        WHERE TRCNUMDOC LIKE ? OR TRCNOM LIKE ? OR TRCAPE LIKE ? OR TRCTEL1 LIKE ?
        LIMIT 50
        "#
    )
    .bind(&search_term)
    .bind(&search_term)
    .bind(&search_term)
    .bind(&search_term)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Error buscando clientes en tabla trc: {}", e))?;

    let mut customers = Vec::new();
    for row in rows {
        customers.push(Customer {
            trcid: row.try_get("TRCID").unwrap_or_default(),
            trcnumdoc: row.try_get("TRCNUMDOC").unwrap_or_default(),
            trcnom: row.try_get("TRCNOM").unwrap_or_default(),
            trcape: row.try_get("TRCAPE").unwrap_or_default(),
            trctel1: row.try_get("TRCTEL1").unwrap_or_default(),
            trcema1: row.try_get("trcema1").unwrap_or_default(),
        });
    }

    Ok(customers)
}

#[tauri::command]
pub async fn get_customer_points_summary(
    trcid: String,
    state: State<'_, AppState>,
) -> Result<PointSummary, String> {
    let pool = state.get_pool().await?;

    // 1. Fetch current loyalty config
    let config = get_loyalty_config(state.clone()).await?;

    // 2. Fetch gross sales for the customer from native table 'venta' (SELECT ONLY)
    // Supports matching VENCLI with TRCID or TRCNUMDOC
    let total_ventas: f64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(VENVAL), 0.0)
        FROM venta
        WHERE (VENCLI = ? OR VENCLI = (SELECT TRCNUMDOC FROM trc WHERE TRCID = ? LIMIT 1))
          AND (VENEST IS NULL OR VENEST != 'A')
          AND VENVAL >= ?
          AND VENFEC >= ?
        "#
    )
    .bind(&trcid)
    .bind(&trcid)
    .bind(config.min_compra_puntos)
    .bind(&config.fecha_inicio_puntos)
    .fetch_one(&pool)
    .await
    .unwrap_or(0.0);

    // Calculate gross accumulated points: FLOOR(total_ventas / monto_por_punto)
    let monto_por_punto = if config.monto_por_punto > 0.0 { config.monto_por_punto } else { 1000.0 };
    let puntos_acumulados_brutos = (total_ventas / monto_por_punto).floor();

    // 3. Fetch total redeemed points from auxiliary table 'puntos_historial'
    let puntos_redimidos: f64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(puntos), 0.0)
        FROM puntos_historial
        WHERE trcid = ? AND tipo = 'REDENCION'
        "#
    )
    .bind(&trcid)
    .fetch_one(&pool)
    .await
    .unwrap_or(0.0);

    let saldo_disponible = (puntos_acumulados_brutos - puntos_redimidos).max(0.0);
    let valor_equivalente_cop = saldo_disponible * config.valor_punto_cop;

    Ok(PointSummary {
        trcid,
        puntos_acumulados_brutos,
        puntos_redimidos,
        saldo_disponible,
        total_ventas_cop: total_ventas,
        valor_equivalente_cop,
    })
}

#[tauri::command]
pub async fn redeem_points(
    trcid: String,
    puntos_a_redimir: f64,
    concepto: String,
    referencia_doc: Option<String>,
    state: State<'_, AppState>,
) -> Result<PointTransaction, String> {
    if puntos_a_redimir <= 0.0 {
        return Err("La cantidad de puntos a redimir debe ser mayor a 0.".to_string());
    }

    let pool = state.get_pool().await?;

    // Begin SQL Transaction with ROLLBACK safety guarantee
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Error al iniciar transacción SQL: {}", e))?;

    // 1. Verify current available balance inside transaction
    let summary = get_customer_points_summary(trcid.clone(), state.clone()).await?;
    if summary.saldo_disponible < puntos_a_redimir {
        let _ = tx.rollback().await;
        return Err(format!(
            "Saldo insuficiente. Saldo disponible: {} pts, Solicitado: {} pts",
            summary.saldo_disponible, puntos_a_redimir
        ));
    }

    // 2. Fetch conversion rate
    let config = get_loyalty_config(state.clone()).await?;
    let monto_cop = puntos_a_redimir * config.valor_punto_cop;

    // 3. Insert transaction into auxiliary table 'puntos_historial'
    let insert_res = sqlx::query(
        r#"
        INSERT INTO puntos_historial (trcid, tipo, puntos, monto_cop, concepto, referencia_doc, fecha)
        VALUES (?, 'REDENCION', ?, ?, ?, ?, NOW())
        "#
    )
    .bind(&trcid)
    .bind(puntos_a_redimir)
    .bind(monto_cop)
    .bind(&concepto)
    .bind(&referencia_doc)
    .execute(&mut *tx)
    .await;

    let result = match insert_res {
        Ok(res) => res,
        Err(e) => {
            let _ = tx.rollback().await;
            return Err(format!("Error guardando historial de canje: {}", e));
        }
    };

    let new_id = result.last_insert_id() as i64;

    // 4. Update consolidated balance table 'puntos_saldo'
    let nuevo_redimidos = summary.puntos_redimidos + puntos_a_redimir;
    let nuevo_saldo = (summary.puntos_acumulados_brutos - nuevo_redimidos).max(0.0);

    let upsert_res = sqlx::query(
        r#"
        INSERT INTO puntos_saldo (trcid, puntos_acumulados, puntos_redimidos, saldo_actual, ultima_actualizacion)
        VALUES (?, ?, ?, ?, NOW())
        ON DUPLICATE KEY UPDATE
            puntos_acumulados = VALUES(puntos_acumulados),
            puntos_redimidos = VALUES(puntos_redimidos),
            saldo_actual = VALUES(saldo_actual),
            ultima_actualizacion = NOW()
        "#
    )
    .bind(&trcid)
    .bind(summary.puntos_acumulados_brutos)
    .bind(nuevo_redimidos)
    .bind(nuevo_saldo)
    .execute(&mut *tx)
    .await;

    if let Err(e) = upsert_res {
        let _ = tx.rollback().await;
        return Err(format!("Error actualizando saldo consolidado: {}", e));
    }

    // 5. Commit SQL Transaction
    tx.commit()
        .await
        .map_err(|e| format!("Error finalizando transacción: {}", e))?;

    // Query formatted date of inserted transaction
    let fecha_str: String = sqlx::query_scalar(
        "SELECT DATE_FORMAT(fecha, '%Y-%m-%d %H:%i:%s') FROM puntos_historial WHERE id = ?"
    )
    .bind(new_id)
    .fetch_one(&pool)
    .await
    .unwrap_or_else(|_| "Ahora".to_string());

    Ok(PointTransaction {
        id: Some(new_id),
        trcid,
        tipo: "REDENCION".to_string(),
        puntos: puntos_a_redimir,
        monto_cop,
        concepto,
        referencia_doc,
        fecha: fecha_str,
    })
}

#[tauri::command]
pub async fn get_points_history(
    trcid: String,
    state: State<'_, AppState>,
) -> Result<Vec<PointTransaction>, String> {
    let pool = state.get_pool().await?;

    let rows = sqlx::query(
        r#"
        SELECT id, trcid, tipo, puntos, monto_cop, concepto, referencia_doc,
               DATE_FORMAT(fecha, '%Y-%m-%d %H:%i:%s') as fecha_fmt
        FROM puntos_historial
        WHERE trcid = ?
        ORDER BY id DESC
        LIMIT 100
        "#
    )
    .bind(&trcid)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Error consultando extracto de historial: {}", e))?;

    let mut history = Vec::new();
    for row in rows {
        history.push(PointTransaction {
            id: row.try_get("id").ok(),
            trcid: row.try_get("trcid").unwrap_or_default(),
            tipo: row.try_get("tipo").unwrap_or_default(),
            puntos: row.try_get("puntos").unwrap_or(0.0),
            monto_cop: row.try_get("monto_cop").unwrap_or(0.0),
            concepto: row.try_get("concepto").unwrap_or_default(),
            referencia_doc: row.try_get("referencia_doc").ok(),
            fecha: row.try_get("fecha_fmt").unwrap_or_default(),
        });
    }

    Ok(history)
}

#[tauri::command]
pub async fn get_loyalty_config(
    state: State<'_, AppState>,
) -> Result<LoyaltyConfig, String> {
    let pool = state.get_pool().await?;

    let row = sqlx::query(
        r#"
        SELECT monto_por_punto, valor_punto_cop, min_compra_puntos,
               DATE_FORMAT(fecha_inicio_puntos, '%Y-%m-%d') as fecha_fmt
        FROM puntos_config
        WHERE id = 1
        "#
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| format!("Error leyendo reglas de puntos: {}", e))?;

    if let Some(r) = row {
        Ok(LoyaltyConfig {
            monto_por_punto: r.try_get("monto_por_punto").unwrap_or(1000.0),
            valor_punto_cop: r.try_get("valor_punto_cop").unwrap_or(50.0),
            min_compra_puntos: r.try_get("min_compra_puntos").unwrap_or(0.0),
            fecha_inicio_puntos: r.try_get("fecha_fmt").unwrap_or_else(|_| "2000-01-01".to_string()),
        })
    } else {
        Ok(LoyaltyConfig::default())
    }
}

#[tauri::command]
pub async fn save_loyalty_config(
    config: LoyaltyConfig,
    state: State<'_, AppState>,
) -> Result<LoyaltyConfig, String> {
    let pool = state.get_pool().await?;

    sqlx::query(
        r#"
        INSERT INTO puntos_config (id, monto_por_punto, valor_punto_cop, min_compra_puntos, fecha_inicio_puntos, updated_at)
        VALUES (1, ?, ?, ?, ?, NOW())
        ON DUPLICATE KEY UPDATE
            monto_por_punto = VALUES(monto_por_punto),
            valor_punto_cop = VALUES(valor_punto_cop),
            min_compra_puntos = VALUES(min_compra_puntos),
            fecha_inicio_puntos = VALUES(fecha_inicio_puntos),
            updated_at = NOW()
        "#
    )
    .bind(config.monto_por_punto)
    .bind(config.valor_punto_cop)
    .bind(config.min_compra_puntos)
    .bind(&config.fecha_inicio_puntos)
    .execute(&pool)
    .await
    .map_err(|e| format!("Error guardando reglas de puntos: {}", e))?;

    Ok(config)
}

#[tauri::command]
pub async fn save_db_config(
    config: DbConfig,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let file_path = get_config_file_path(&app_handle)?;
    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Error serializando configuración DB: {}", e))?;
    std::fs::write(&file_path, content)
        .map_err(|e| format!("Error escribiendo archivo db_config.json: {}", e))?;

    // Try applying config
    check_db_connection(Some(config), state).await
}

#[tauri::command]
pub async fn get_db_config(
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<DbConfig, String> {
    let file_path = get_config_file_path(&app_handle)?;
    if file_path.exists() {
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Error leyendo db_config.json: {}", e))?;
        if let Ok(config) = serde_json::from_str::<DbConfig>(&content) {
            let mut guard = state.db_config.write().await;
            *guard = config.clone();
            return Ok(config);
        }
    }

    let guard = state.db_config.read().await;
    Ok(guard.clone())
}
