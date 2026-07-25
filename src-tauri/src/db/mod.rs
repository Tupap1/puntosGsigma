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
        Err("No hay conexión activa a la base de datos MySQL 5.5.".to_string())
    }
}

pub async fn create_connection_pool(config: &DbConfig) -> Result<MySqlPool, String> {
    // 1. Resolve host: Convert "localhost" to IPv4 "127.0.0.1" for Windows MySQL 5.5 compatibility
    let host_str = config.host.trim();
    let target_host = if host_str.eq_ignore_ascii_case("localhost") || host_str.is_empty() {
        "127.0.0.1"
    } else {
        host_str
    };

    let target_db = if config.database.trim().is_empty() {
        "pv"
    } else {
        config.database.trim()
    };

    // 2. Try connecting directly to target database (e.g. 'pv')
    let base_options = MySqlConnectOptions::new()
        .host(target_host)
        .port(config.port)
        .username(config.user.trim())
        .password(&config.password)
        .ssl_mode(MySqlSslMode::Disabled);

    let target_options = base_options.clone().database(target_db);

    let pool_result = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(target_options)
        .await;

    match pool_result {
        Ok(pool) => Ok(pool),
        Err(err_err) => {
            let err_msg = err_err.to_string();
            
            // 3. If failure was due to Unknown database, connect without DB first and create 'pv'
            if err_msg.contains("Unknown database") || err_msg.contains("1049") {
                let root_pool = MySqlPoolOptions::new()
                    .max_connections(2)
                    .acquire_timeout(std::time::Duration::from_secs(5))
                    .connect_with(base_options)
                    .await
                    .map_err(|e| format!("Error al conectar con MySQL 5.5 en {}:{}: {}", target_host, config.port, e))?;

                let create_db_query = format!("CREATE DATABASE IF NOT EXISTS `{}` DEFAULT CHARACTER SET utf8;", target_db);
                sqlx::query(&create_db_query)
                    .execute(&root_pool)
                    .await
                    .map_err(|e| format!("Error creando base de datos '{}': {}", target_db, e))?;

                root_pool.close().await;

                // Reconnect to newly created database
                let final_options = MySqlConnectOptions::new()
                    .host(target_host)
                    .port(config.port)
                    .username(config.user.trim())
                    .password(&config.password)
                    .database(target_db)
                    .ssl_mode(MySqlSslMode::Disabled);

                MySqlPoolOptions::new()
                    .max_connections(10)
                    .acquire_timeout(std::time::Duration::from_secs(5))
                    .connect_with(final_options)
                    .await
                    .map_err(|e| format!("Error conectando a BD creada '{}': {}", target_db, e))
            } else {
                Err(format!("Error de conexión MySQL ({}:{}): {}", target_host, config.port, err_msg))
            }
        }
    }
}

pub async fn init_db_tables(pool: &MySqlPool) -> Result<(), String> {
    // 1. Table pv.puntos_config
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

    // 2. Table pv.puntos_saldo
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

    // 3. Table pv.puntos_historial
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

        let compra_100k = 100000.0;
        let puntos = (compra_100k / config.monto_por_punto).floor();
        assert_eq!(puntos, 100.0);

        let valor_cop = puntos * config.valor_punto_cop;
        assert_eq!(valor_cop, 5000.0);
    }
}
