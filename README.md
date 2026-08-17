# puntosGsigma — Módulo de Fidelización y Puntos POS

Aplicación de escritorio local, ligera y auto-actualizable para la gestión y
fidelización de puntos de clientes, integrada de forma segura (solo lectura sobre
las tablas nativas) con la base de datos **MySQL 5.5** del POS **GsigmaPOS / Itheke**.

## Stack

- **Shell de escritorio:** Tauri v2 (Rust)
- **Capa de datos:** Rust + `sqlx` (MySQL, `mysql_native_password`), `tokio`
- **Frontend:** SvelteKit compilado a SPA estática (`@sveltejs/adapter-static`)
- **Auto-actualización:** `tauri-plugin-updater` desde GitHub Releases
- **CI/CD:** GitHub Actions → instalador `.msi`/`.nsis` firmado + `latest.json`

## Modelo de datos y seguridad

El módulo **nunca** altera el esquema del POS. Solo escribe en tablas auxiliares
propias dentro de la base `pv`:

| Ámbito | Tablas | Acceso |
| --- | --- | --- |
| Clientes / ventas nativas POS | `adm.trc`, `pv.venta` | **SELECT ONLY** |
| Módulo de puntos (propias) | `pv.puntos_config`, `pv.puntos_saldo`, `pv.puntos_historial` | Lectura/Escritura |

No se usan `ALTER TABLE`, `DROP TABLE` ni `TRIGGER` sobre tablas nativas.

### Motor de puntos (query-based ledger)

Los puntos se calculan en tiempo real al consultar al cliente, sin triggers:

```
Puntos por factura = FLOOR(VENVAL / monto_por_punto)   -- por cada venta elegible
Puntos brutos      = SUM(puntos por factura)           -- facturas no anuladas
Saldo disponible   = Puntos brutos − Puntos redimidos  -- redimidos: pv.puntos_historial
```

Facturas anuladas (`VENEST = 'A'`), por debajo de `min_compra_puntos` o anteriores
a `fecha_inicio_puntos` no otorgan puntos. Si una factura se anula en el POS, la
consulta la descuenta automáticamente en la siguiente lectura.

## Desarrollo

```bash
npm install
npm run tauri dev      # app de escritorio en modo desarrollo
npm run build          # compila la SPA estática a build/
```

Parámetros de conexión por defecto (sobreescribibles con variables de entorno o
desde la pantalla de Configuración de la app): `DB_HOST`, `DB_PORT`, `DB_USER`,
`DB_PASS`, `DB_NAME` (por defecto `pv`).

## Prueba de integración con Docker (MySQL 5.5 real)

El directorio `docker/` levanta un MySQL 5.5 idéntico al de producción y carga la
copia real de la base de datos más datos de venta sintéticos para validar el motor
de puntos de extremo a extremo.

```bash
cd docker
docker compose up -d           # carga la copia (tarda varios minutos)
docker compose logs -f db      # observa el avance

# Ejecuta la prueba de integración contra el contenedor:
cd ../src-tauri
DB_HOST=127.0.0.1 DB_PORT=3316 DB_USER=root DB_PASS=test \
  cargo test --test points_engine -- --ignored --nocapture

cd ../docker && docker compose down -v   # limpia el entorno
```

La prueba valida: conexión, creación de tablas auxiliares, búsqueda de clientes,
cálculo de puntos por factura y redención transaccional.

## Publicación y auto-actualización

El workflow `.github/workflows/release.yml` compila en `windows-latest` y publica
un Release al empujar un tag `v*`.

Para que la auto-actualización funcione, configura en el repositorio
(**Settings → Secrets and variables → Actions**) los secretos:

- `TAURI_SIGNING_PRIVATE_KEY` — contenido de la llave privada de firma.
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — contraseña de la llave (vacía si no tiene).

La llave pública correspondiente ya está en `src-tauri/tauri.conf.json`
(`plugins.updater.pubkey`). Genera un par nuevo con:

```bash
npx tauri signer generate -w miclave.key
```

Luego crea y publica una versión:

```bash
git tag v0.1.0 && git push origin v0.1.0
```
