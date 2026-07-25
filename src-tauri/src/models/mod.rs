use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Customer {
    pub trcid: String,
    pub trcnumdoc: String,
    pub trcnom: String,
    pub trcape: String,
    pub trctel1: String,
    pub trcema1: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointSummary {
    pub trcid: String,
    pub puntos_acumulados_brutos: f64,
    pub puntos_redimidos: f64,
    pub saldo_disponible: f64,
    pub total_ventas_cop: f64,
    pub valor_equivalente_cop: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct PointTransaction {
    pub id: Option<i64>,
    pub trcid: String,
    pub tipo: String,
    pub puntos: f64,
    pub monto_cop: f64,
    pub concepto: String,
    pub referencia_doc: Option<String>,
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LoyaltyConfig {
    pub monto_por_punto: f64,
    pub valor_punto_cop: f64,
    pub min_compra_puntos: f64,
    pub fecha_inicio_puntos: String,
}

impl Default for LoyaltyConfig {
    fn default() -> Self {
        Self {
            monto_por_punto: 1000.0,
            valor_punto_cop: 50.0,
            min_compra_puntos: 0.0,
            fecha_inicio_puntos: "2000-01-01".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "".to_string(),
            database: "pv".to_string(),
        }
    }
}
