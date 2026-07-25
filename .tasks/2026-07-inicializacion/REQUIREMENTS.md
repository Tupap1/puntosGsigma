# Requerimientos: Inicialización y Arquitectura Base del Sistema de Puntos POS

## 1. Contexto y Objetivos
Desarrollar una aplicación de escritorio local ultra ligera, modular y auto-actualizable para la gestión y fidelización de puntos de clientes, integrada de forma segura con la base de datos MySQL 5.5 en producción del sistema POS GsigmaPOS/Itheke.

## 2. Stack Tecnológico Obligatorio
- **Desktop Shell**: Tauri v2 (Rust)
- **Backend / Database Layer**: Rust con `sqlx` (driver asíncrono compatible con MySQL 5.5 / `mysql_native_password`), `tokio`, `tauri-plugin-updater`
- **Frontend**: SvelteKit compilado a SPA estática (`@sveltejs/adapter-static`)
- **Estilos**: Vanilla CSS con sistema de variables (Dark Mode `#0d1425`, Acento verde `#10b981`, fuentes Inter/Outfit)
- **CI/CD & Releases**: GitHub Actions (`.github/workflows/release.yml`) para generación de installer `.msi` de Windows firmado/empaquetado para auto-actualizaciones.

## 3. Protocolo de Seguridad y Base de Datos (MySQL 5.5 POS)
Basado en el análisis verificado de la copia de base de datos (`Copia21 20260709 1809.sql`):
- **Tablas POS Nativas de Clientes / Facturación (Lectura Estricta - `SELECT ONLY`)**:
  - `trc`: Consulta de terceros/clientes por `TRCNUMDOC` (Cédula/NIT), `TRCNOM`, `TRCAPE`, `TRCTEL1`, `trcema1`.
  - `trcfac`: Estructura nativa de acumulado mensual (`TRCID`, `TRFANO`, `TRFMES`, `TRFVAL`, `TRFPUN`).
  - `compra` / `dcmpr` / `venta` / `dvent`: Historial de facturas y líneas de venta.
  - `forpa`: Tabla de formas de pago (Forma `'99'`: `REDENCION DE PUNTOS`, `FORAPOCOPE = 'RDP'`).
- **Tablas Auxiliares Dedicadas al Módulo (Creación y Lectura/Escritura Aislada)**:
  - `pv.puntos_config`: Configuración de reglas (COP por punto `monto_por_punto`, equivalencia en descuento COP `valor_punto_cop`, compra mínima `min_compra_puntos`).
  - `pv.puntos_saldo`: Consolidado rápido de saldo acumulado.
  - `pv.puntos_historial`: Registro auditable de movimientos (acumulaciones, canjes, ajustes, vencimientos).
- **Prohibiciones Estrictas**:
  - NO realizar `ALTER TABLE`, `DROP TABLE` ni `CREATE TRIGGER` en tablas nativas del POS.

## 4. Requerimientos Funcionales Base
1. **Módulo de Conexión y Estado**:
   - Configuración de parámetros de conexión local MySQL 5.5.
   - Badge visual en tiempo real con indicador del estado de la base de datos.
2. **Buscador Instantáneo de Clientes**:
   - Búsqueda en tiempo real por Cédula / NIT / Nombre / Teléfono desde `trc`.
3. **Motor Ledger de Puntos en Tiempo Real**:
   - Cálculo dinámico del acumulado bruto por ventas del cliente y deducción de canjes anteriores.
4. **Módulo de Canje y Redención**:
   - Interfaz modal/slide-over para procesar canje de puntos con validación de saldo.
   - Registro de transacción en `pv.puntos_historial` y sincronización opcional con `trcfac`.
5. **Auto-actualización de la Aplicación**:
   - Integración con `tauri-plugin-updater` para verificación y descarga de actualizaciones en segundo plano desde GitHub Releases.
