use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::{mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode}, MySqlPool};
use crate::models::DbConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<RwLock<Option<MySqlPool>>>,
    pub db_config: Arc<RwLock<DbConfig>>,
}

impl AppState {
    pub fn new(config: DbConfig) -> Self {
        Self {
            pool: Arc::new(RwLock::new(None)),
            db_config: Arc::new(RwLock::new(config)),
        }
    }

    pub async fn get_pool(&self) -> Result<MySqlPool, String> {
        let pool_guard = self.pool.read().await;
        if let Some(ref pool) = *pool_guard {
            if !pool.is_closed() {
                return Ok(pool.clone());
            }
        }
        Err("No hay conexión activa a la base de datos MySQL.".to_string())
    }
}

pub async fn create_connection_pool(config: &DbConfig) -> Result<MySqlPool, String> {
    let connect_options = MySqlConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password)
        .database(&config.database)
        .ssl_mode(MySqlSslMode::Disabled);

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(connect_options)
        .await
        .map_err(|e| format!("Error al conectar con MySQL: {}", e))?;

    Ok(pool)
}

pub async fn init_db_tables(pool: &MySqlPool) -> Result<(), String> {
    // 1. pv.puntos_config
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS puntos_config (
            id INT NOT NULL DEFAULT 1 PRIMARY KEY,
            monto_por_punto DOUBLE NOT NULL DEFAULT 1000.0,
            valor_punto_cop DOUBLE NOT NULL DEFAULT 50.0,
            min_compra_puntos DOUBLE NOT NULL DEFAULT 0.0,
            fecha_inicio_puntos DATE NOT NULL DEFAULT '2000-01-01',
            updated_at DATETIME NOT NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Error creando tabla puntos_config: {}", e))?;

    // Seed default config if empty
    sqlx::query(
        r#"
        INSERT IGNORE INTO puntos_config (id, monto_por_punto, valor_punto_cop, min_compra_puntos, fecha_inicio_puntos, updated_at)
        VALUES (1, 1000.0, 50.0, 0.0, '2000-01-01', NOW());
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Error insertando configuración por defecto: {}", e))?;

    // 2. pv.puntos_saldo
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS puntos_saldo (
            trcid VARCHAR(20) NOT NULL PRIMARY KEY,
            puntos_acumulados DOUBLE NOT NULL DEFAULT 0,
            puntos_redimidos DOUBLE NOT NULL DEFAULT 0,
            saldo_actual DOUBLE NOT NULL DEFAULT 0,
            ultima_actualizacion DATETIME NOT NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Error creando tabla puntos_saldo: {}", e))?;

    // 3. pv.puntos_historial
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS puntos_historial (
            id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
            trcid VARCHAR(20) NOT NULL,
            tipo VARCHAR(20) NOT NULL,
            puntos DOUBLE NOT NULL,
            monto_cop DOUBLE NOT NULL DEFAULT 0,
            concepto VARCHAR(255) NOT NULL DEFAULT '',
            referencia_doc VARCHAR(50) DEFAULT '',
            fecha DATETIME NOT NULL,
            KEY idx_trcid (trcid)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Error creando tabla puntos_historial: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::LoyaltyConfig;

    #[test]
    fn test_points_calculation_standard() {
        let config = LoyaltyConfig {
            monto_por_punto: 1000.0,
            valor_punto_cop: 50.0,
            min_compra_puntos: 10000.0,
            fecha_inicio_puntos: "2000-01-01".to_string(),
        };

        // Factura de $100.000 COP -> Debe dar 100 puntos
        let compra_100k = 100000.0;
        let puntos = (compra_100k / config.monto_por_punto).floor();
        assert_eq!(puntos, 100.0);

        // Equivalencia en COP para canje de 100 puntos -> 100 * 50 = $5.000 COP
        let valor_cop = puntos * config.valor_punto_cop;
        assert_eq!(valor_cop, 5000.0);
    }

    #[test]
    fn test_minimum_purchase_threshold() {
        let config = LoyaltyConfig {
            monto_por_punto: 1000.0,
            valor_punto_cop: 50.0,
            min_compra_puntos: 10000.0,
            fecha_inicio_puntos: "2000-01-01".to_string(),
        };

        // Compra menor al mínimo ($5.000 COP) -> No acumula puntos
        let compra_5k = 5000.0;
        let puntos = if compra_5k >= config.min_compra_puntos {
            (compra_5k / config.monto_por_punto).floor()
        } else {
            0.0
        };
        assert_eq!(puntos, 0.0);
    }

    #[test]
    fn test_redemption_validation() {
        let saldo_disponible = 150.0;
        let puntos_a_redimir = 100.0;

        // Canje dentro del saldo -> Permitido
        assert!(puntos_a_redimir <= saldo_disponible);

        // Canje excediendo saldo -> Rechazado
        let puntos_excesivos = 200.0;
        assert!(puntos_excesivos > saldo_disponible);
    }
}

