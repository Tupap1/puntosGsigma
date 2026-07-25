# Plan de Implementación: Estructura Base y Arquitectura

## 1. Arquitectura de Proyectos

```
puntosGsigma/
├── .github/
│   └── workflows/
│       └── release.yml          # GitHub Actions para CI/CD y auto-updater MSI
├── .tasks/
│   └── 2026-07-inicializacion/ # Documentación de requerimientos y tareas
├── src/                        # SvelteKit SPA Frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── api/            # Wrapper IPC para comandos Tauri Rust
│   │   │   ├── components/     # Componentes UI (Buscador, Cards, Slide-over, Badges)
│   │   │   └── stores/         # Svelte stores (cliente, puntos, conexion)
│   │   ├── styles/             # Variables CSS, utilidades y tema oscuro (#0d1425, #10b981)
│   │   ├── app.html
│   │   └── routes/             # Páginas SvelteKit SPA (+page.svelte, +layout.svelte)
│   ├── svelte.config.js        # Configurado con @sveltejs/adapter-static
│   ├── vite.config.js
│   └── package.json
└── src-tauri/                  # Tauri v2 Rust Application
    ├── Cargo.toml              # Tauri v2, sqlx (mysql), tokio, serde, tauri-plugin-updater
    ├── tauri.conf.json         # Configuración Tauri v2, bundle, updater
    ├── capabilities/           # Tauri v2 capabilities (default.json)
    └── src/
        ├── main.rs
        ├── lib.rs
        ├── db/                 # Pool MySQL, migraciones de tablas auxiliares, queries
        ├── commands/           # Comandos IPC para SvelteKit
        └── models/             # Estructuras de datos Rust
```

## 2. Metodología de Cálculo y Vinculación de Compras sin Triggers

### A. Configuración Dinámica de Parámetros (`pv.puntos_config`)
Los parámetros de conversión se almacenan en la tabla auxiliar `pv.puntos_config` y se pueden modificar libremente desde la interfaz:
- `monto_por_punto`: Monto en COP necesario para ganar 1 punto (Ej: `$1.000 COP` = 1 punto).
- `valor_punto_cop`: Equivalencia en pesos de 1 punto para canjes (Ej: 1 punto = `$50 COP` en descuento).
- `min_compra_puntos`: Monto mínimo de factura para otorgar puntos (Ej: `$10.000 COP`).
- `fecha_inicio_puntos`: Fecha a partir de la cual las facturas empiezan a sumar puntos.

### B. Vinculación de Facturas sin Modificar el POS (Motor Query-Based Ledger)
Dado que **no se permite modificar `pv.venta` ni usar `TRIGGERS`**, la vinculación se realiza en la capa de aplicación de Rust (`sqlx`) de dos formas complementarias:

1. **Cálculo Dinámico al Consultar el Cliente (Tiempo Real)**:
   - Al buscar a un cliente en el sistema, Rust ejecuta una consulta `SELECT` sobre `pv.venta` filtrando por el ID o Cédula del cliente y facturas activas (`anulado = 0` o `estado = 'VALIDA'`).
   - Se calcula el acumulado bruto multiplicando cada factura elegible por la regla de conversión:
     $$\text{Puntos Ganados} = \text{FLOOR}\left(\frac{\text{Total Factura}}{\text{monto\_por\_punto}}\right)$$
   - Se resta el total de puntos que el cliente ha redimido previamente (registrados en la tabla auxiliar `pv.puntos_historial`).
   - **Resultado**: $\text{Saldo Disponible} = \text{Puntos Ganados Brutos} - \text{Puntos Redimidos}$.

2. **Ventajas Clave de esta Metodología**:
   - **Cero Riesgo POS**: No altera las tablas de facturación activa.
   - **Actualización Instantánea**: En cuanto la caja del POS guarda la venta en `pv.venta`, el sistema de puntos la lee inmediatamente al consultar al cliente.
   - **Gestión Automática de Anulaciones**: Si en el POS se anula una factura, la consulta `SELECT` automáticamente la descuenta sin requerir lógica compleja de reversión en triggers.
   - **Auditoría Transaccional (`pv.puntos_historial`)**: Solo las redenciones/canjes o ajustes manuales escriben registros en `pv.puntos_historial`.

---

## 3. Fase de Ejecución

1. **Estructura SvelteKit Frontend**:
   - Inicializar `package.json` con dependencias SvelteKit, `@sveltejs/adapter-static`, `@tauri-apps/api`, `@tauri-apps/plugin-updater`, `lucide-svelte`.
   - Configurar `svelte.config.js` en modo Single Page Application (SPA).
   - Crear Sistema de Estilos Vanilla CSS con variables de color (`#0d1425`, `#10b981`, fuentes Inter/Outfit).

2. **Estructura Tauri v2 (Rust Backend)**:
   - Configurar `Cargo.toml` con `sqlx` (con soporte MySQL 5.5 / async Tokio), `tauri` (v2), `tauri-plugin-updater`, `serde`.
   - Crear módulo `db/` para la gestión segura de pool de conexiones y script de inicialización de tablas auxiliares (`pv.puntos_config`, `pv.puntos_saldo`, `pv.puntos_historial`).
   - Crear handlers de comandos Tauri IPC en `commands/`:
     - `check_db_connection`
     - `search_customers`
     - `get_customer_points`
     - `redeem_points`
     - `get_points_history`

3. **Workflow GitHub Actions (`release.yml`)**:
   - Compilación automatizada en runner de Windows (`windows-latest`).
   - Generación de instalador `.msi` y bundle de actualización de Tauri v2.
   - Publicación automática de Release en GitHub con archivos binarios y `latest.json` para auto-update.

4. **Verificación & Integración**:
   - Verificar la compilación local (`cargo check`, `npm run build`).
   - Validar cumplimiento del protocolo de seguridad en BD (SELECT ONLY en tablas nativas POS).

