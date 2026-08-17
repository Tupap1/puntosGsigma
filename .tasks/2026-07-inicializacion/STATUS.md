# Estado del Proyecto: Listo para Producción

- **Fecha**: 2026-08-17
- **Estado**: Módulos SD-01 a SD-07 implementados y verificados. Frontend SPA,
  backend Rust/sqlx, comandos IPC, tablas auxiliares y pipeline de release completos.
- **Verificación**:
  - `npm run build` → SPA estática generada en `build/`. ✅
  - `cargo build` / `cargo test --lib` → compilación limpia. ✅
  - **Prueba de integración con Docker (MySQL 5.5 + copia real de producción)**: la
    búsqueda de clientes sobre `adm.trc`, el cálculo de puntos por factura sobre
    `pv.venta` (26 pts para el cliente de prueba, factura anulada excluida) y la
    redención transaccional se validaron de extremo a extremo. ✅
- **Correcciones para producción aplicadas**:
  - Motor de puntos calculado **por factura** (`SUM(FLOOR(VENVAL/monto))`) según el plan.
  - Firma de auto-actualización: `pubkey` y `createUpdaterArtifacts` configurados,
    endpoint corregido a `Tupap1/puntosGsigma`, `release.yml` firma con secretos.
  - Eliminada credencial hardcodeada en el test; ahora es un test de integración
    dirigido por variables de entorno y marcado `#[ignore]`.
- **Pendiente antes de publicar**: cargar los secretos `TAURI_SIGNING_PRIVATE_KEY`
  y `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` en GitHub (ver README).
