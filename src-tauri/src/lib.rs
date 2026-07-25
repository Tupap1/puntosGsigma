pub mod commands;
pub mod db;
pub mod models;

use db::AppState;
use models::DbConfig;
use std::env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    let default_config = DbConfig {
        host: env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("DB_PORT")
            .unwrap_or_else(|_| "3306".to_string())
            .parse()
            .unwrap_or(3306),
        user: env::var("DB_USER").unwrap_or_else(|_| "root".to_string()),
        password: env::var("DB_PASS").unwrap_or_else(|_| "".to_string()),
        database: env::var("DB_NAME").unwrap_or_else(|_| "pv".to_string()),
    };

    let app_state = AppState::new(default_config);

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::check_db_connection,
            commands::search_customers,
            commands::get_customer_points_summary,
            commands::redeem_points,
            commands::get_points_history,
            commands::get_loyalty_config,
            commands::save_loyalty_config,
            commands::save_db_config,
            commands::get_db_config,
        ])
        .run(tauri::generate_context!())
        .expect("Error mientras se ejecutaba la aplicación Tauri");
}
