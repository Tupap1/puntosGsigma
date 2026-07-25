# Lista de Tareas Desglosadas: Inicialización y Arquitectura Base

## Tareas Principales (SD-01 a SD-07)

- [x] **SD-00**: Análisis del esquema real en copia BD `Copia21 20260709 1809.sql` y validación de campos `trc`, `trcfac.TRFPUN`, `forpa`, `redpund`.
- [ ] **SD-01: Configuración de Proyecto SvelteKit SPA (Frontend)**
  - [ ] Crear `package.json` con dependencias SvelteKit, `@sveltejs/adapter-static`, `@tauri-apps/api`, `@tauri-apps/plugin-updater`, `lucide-svelte`.
  - [ ] Configurar `svelte.config.js` y `vite.config.js` para generación de SPA estática en `build/`.
  - [ ] Crear `src/app.html` y estructura de rutas `src/routes/`.
- [ ] **SD-02: Sistema de Diseño CSS Vanilla & UI Components**
  - [ ] Implementar `src/lib/styles/theme.css` (Dark mode `#0d1425`, acento `#10b981`, fuentes Inter/Outfit, tablas densas, botones, badges pulsantes, toast notifications).
  - [ ] Crear componente `Navbar.svelte` (Badge de estado BD, botón de configuración de conexión, logo).
  - [ ] Crear componente `CustomerSearch.svelte` (Buscador instantáneo por Cédula/NIT/Nombre con debounce).
  - [ ] Crear componente `PointsSummaryCard.svelte` (Tarjeta financiera de saldo acumulado, redimido, disponible y equivalencia en COP).
  - [ ] Crear componente `RedeemModal.svelte` (Slide-over drawer para procesar canjes de puntos con validación de saldo).
  - [ ] Crear componente `HistoryTable.svelte` (Tabla densa con historial de movimientos y facturas).
  - [ ] Crear componente `DbConfigModal.svelte` (Modal de credenciales MySQL).
- [ ] **SD-03: Configuración de Proyecto Tauri v2 (Rust Backend)**
  - [ ] Crear `src-tauri/Cargo.toml` con `tauri` (v2), `sqlx` (features `mysql`, `runtime-tokio-rustls`), `tokio`, `serde`, `tauri-plugin-updater`, `dotenvy`.
  - [ ] Crear `src-tauri/tauri.conf.json` con configuración de ventana, bundle, updater e identificador `com.gsigma.puntos`.
  - [ ] Crear `src-tauri/capabilities/default.json` con permisos de Tauri v2.
  - [ ] Crear `src-tauri/build.rs` y punto de entrada `src-tauri/src/main.rs` & `src-tauri/src/lib.rs`.
- [ ] **SD-04: Capa de Base de Datos `sqlx` & Tablas Auxiliares**
  - [ ] Crear `src-tauri/src/db/mod.rs`: Pool de conexiones MySQL 5.5 asíncrono y script de auto-creación de tablas auxiliares (`pv.puntos_config`, `pv.puntos_saldo`, `pv.puntos_historial`).
  - [ ] Crear `src-tauri/src/models/mod.rs`: Structs Rust para `Customer` (mapeado de `trc`), `PointSummary`, `PointTransaction`, `LoyaltyConfig`, `DbCredentials`.
- [ ] **SD-05: Comandos IPC Tauri Rust (`src-tauri/src/commands/mod.rs`)**
  - [ ] Command `check_db_connection`: Validar ping a la BD MySQL.
  - [ ] Command `search_customers`: Búsqueda sobre `trc` (`TRCNUMDOC`, `TRCNOM`, `TRCAPE`).
  - [ ] Command `get_customer_points_summary`: Cálculo en tiempo real de puntos acumulados por ventas y deducción de canjes.
  - [ ] Command `redeem_points`: Registro de canje en `pv.puntos_historial` y `pv.puntos_saldo`.
  - [ ] Command `get_points_history`: Consulta de extracto de movimientos.
  - [ ] Command `get_loyalty_config` / `save_loyalty_config`: Gestión de regla de conversión ($1.000 COP = 1 pt).
- [ ] **SD-06: CI/CD & Pipeline de Auto-actualización**
  - [ ] Configurar `.github/workflows/release.yml` para compilación en Windows `windows-latest`, empaquetado `.msi` y generación de `latest.json` para auto-update.
  - [ ] Configurar `.gitignore` y `README.md`.
- [ ] **SD-07: Verificación y Pruebas de Compilación**
  - [ ] Ejecutar `npm run build` en el frontend y validar salida SPA estática.
  - [ ] Ejecutar `cargo check` en `src-tauri/` y verificar ausencia de errores en Rust.
