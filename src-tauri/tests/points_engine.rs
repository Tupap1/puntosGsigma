//! Prueba de integración del motor de puntos contra un MySQL 5.5 real.
//!
//! Ejecuta las MISMAS consultas SQL que usan los comandos IPC de la aplicación
//! contra la copia de producción cargada en el contenedor Docker (`docker/`),
//! validando de extremo a extremo los objetivos del módulo:
//!   - Conexión a MySQL 5.5 y creación de tablas auxiliares aisladas (pv.puntos_*)
//!   - Búsqueda de clientes sobre adm.trc (SELECT ONLY)
//!   - Cálculo de puntos por factura sobre pv.venta
//!   - Redención transaccional en pv.puntos_historial y saldo resultante
//!
//! Está marcada `#[ignore]` para no depender de infraestructura en `cargo test`
//! normal. Para correrla con el contenedor arriba:
//!
//!   DB_HOST=127.0.0.1 DB_PORT=3316 DB_USER=root DB_PASS=test \
//!     cargo test -p puntos-gsigma --test points_engine -- --ignored --nocapture

use puntos_gsigma::db::{create_connection_pool, init_db_tables};
use puntos_gsigma::models::DbConfig;
use sqlx::Row;

fn config_from_env() -> DbConfig {
    DbConfig {
        host: std::env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: std::env::var("DB_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(3316),
        user: std::env::var("DB_USER").unwrap_or_else(|_| "root".to_string()),
        password: std::env::var("DB_PASS").unwrap_or_else(|_| "test".to_string()),
        database: std::env::var("DB_NAME").unwrap_or_else(|_| "pv".to_string()),
    }
}

// TRCID del cliente sembrado en docker/seed-test-data.sql
const TEST_TRCID: &str = "000000000000001";

#[tokio::test]
#[ignore = "requiere el contenedor MySQL 5.5 de docker/ activo"]
async fn full_points_flow() {
    let pool = create_connection_pool(&config_from_env())
        .await
        .expect("no se pudo conectar a MySQL 5.5");

    init_db_tables(&pool)
        .await
        .expect("no se pudieron crear las tablas auxiliares pv.puntos_*");

    // 1) Búsqueda de cliente sobre adm.trc (misma consulta que search_customers)
    let term = "%40019552%";
    let rows = sqlx::query(
        r#"SELECT TRCID, TRCNUMDOC, TRCNOM FROM adm.trc
           WHERE (TRCNUMDOC LIKE ? OR TRCNOM LIKE ? OR TRCAPE LIKE ? OR TRCTEL1 LIKE ?)
             AND TRCNUMDOC IS NOT NULL AND TRCNUMDOC != '' LIMIT 50"#,
    )
    .bind(term).bind(term).bind(term).bind(term)
    .fetch_all(&pool).await.expect("fallo la búsqueda en adm.trc");
    assert!(!rows.is_empty(), "la búsqueda debía encontrar al cliente de prueba");
    let nom: String = rows[0].try_get("TRCNOM").unwrap_or_default();
    println!("[search] encontrado: {}", nom);

    // 2) Cálculo de puntos por factura (misma consulta que get_customer_points_summary)
    let monto_por_punto = 1000.0_f64;
    let row = sqlx::query(
        r#"SELECT COALESCE(SUM(VENVAL),0.0) AS total_ventas,
                  COALESCE(SUM(FLOOR(VENVAL / ?)),0.0) AS puntos_brutos
           FROM pv.venta
           WHERE (VENCLI = ? OR VENCLI = (SELECT TRCNUMDOC FROM adm.trc WHERE TRCID = ? LIMIT 1))
             AND (VENEST IS NULL OR VENEST != 'A')
             AND VENVAL >= ? AND VENFEC >= ?"#,
    )
    .bind(monto_por_punto)
    .bind(TEST_TRCID).bind(TEST_TRCID)
    .bind(0.0_f64).bind("2000-01-01")
    .fetch_one(&pool).await.expect("fallo el cálculo de puntos");

    let total_ventas: f64 = row.try_get("total_ventas").unwrap_or(0.0);
    let puntos_brutos: f64 = row.try_get("puntos_brutos").unwrap_or(0.0);
    println!("[points] total_ventas={} puntos_brutos={}", total_ventas, puntos_brutos);

    // Factura anulada (99.000) excluida => total 1.500 + 25.000 = 26.500
    assert_eq!(total_ventas, 26500.0, "el total de ventas elegibles no coincide");
    // FLOOR(1500/1000)=1  +  FLOOR(25000/1000)=25  = 26 puntos
    assert_eq!(puntos_brutos, 26.0, "los puntos por factura no coinciden");

    // 3) Redención transaccional en pv.puntos_historial (idempotente: limpia al final)
    sqlx::query("DELETE FROM pv.puntos_historial WHERE trcid = ? AND concepto = 'TEST_INTEGRACION'")
        .bind(TEST_TRCID).execute(&pool).await.ok();

    sqlx::query(
        r#"INSERT INTO pv.puntos_historial (trcid, tipo, puntos, monto_cop, concepto, referencia_doc, fecha)
           VALUES (?, 'REDENCION', 10, 500, 'TEST_INTEGRACION', 'T-001', NOW())"#,
    )
    .bind(TEST_TRCID).execute(&pool).await.expect("fallo insertar la redención");

    let redimidos: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(puntos),0.0) FROM pv.puntos_historial WHERE trcid = ? AND tipo = 'REDENCION'",
    )
    .bind(TEST_TRCID).fetch_one(&pool).await.expect("fallo leer redimidos");

    let saldo_disponible = (puntos_brutos - redimidos).max(0.0);
    println!("[redeem] redimidos={} saldo_disponible={}", redimidos, saldo_disponible);
    assert_eq!(saldo_disponible, 16.0, "el saldo tras redimir 10 pts debía ser 16");

    // Limpieza
    sqlx::query("DELETE FROM pv.puntos_historial WHERE trcid = ? AND concepto = 'TEST_INTEGRACION'")
        .bind(TEST_TRCID).execute(&pool).await.ok();

    println!("[OK] flujo completo de puntos validado contra MySQL 5.5");
}
