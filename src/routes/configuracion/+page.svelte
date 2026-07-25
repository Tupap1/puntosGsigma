<script lang="ts">
  import { onMount } from 'svelte';
  import { loyaltyConfigStore, dbConfigStore, isDbConnected, addToast } from '$lib/stores/appStore';
  import { getLoyaltyConfig, saveLoyaltyConfig, getDbConfig, saveDbConfig, checkDbConnection, type DbConfig } from '$lib/api';
  import { Settings, Save, Database, Server, Key, Shield, CheckCircle2, AlertTriangle, Loader2 } from 'lucide-svelte';

  let isSavingLoyalty = false;
  let isSavingDb = false;
  let isTestingDb = false;
  let dbTestMessage = '';
  let dbTestSuccess: boolean | null = null;

  let localLoyalty = { ...$loyaltyConfigStore };
  let localDb: DbConfig = { 
    host: $dbConfigStore.host || '127.0.0.1',
    port: $dbConfigStore.port || 3306,
    user: $dbConfigStore.user || 'root',
    password: $dbConfigStore.password || '',
    database: 'pv' 
  };

  onMount(async () => {
    try {
      const cfg = await getLoyaltyConfig();
      if (cfg) {
        loyaltyConfigStore.set(cfg);
        localLoyalty = { ...cfg };
      }
      const dbCfg = await getDbConfig();
      if (dbCfg) {
        dbConfigStore.set({ ...dbCfg, database: 'pv' });
        localDb = { ...dbCfg, database: 'pv' };
      }
    } catch (e) {
      console.warn('Error al cargar configuración:', e);
    }
  });

  async function handleSaveLoyalty() {
    isSavingLoyalty = true;
    try {
      await saveLoyaltyConfig(localLoyalty);
      loyaltyConfigStore.set(localLoyalty);
      addToast('Parámetros de fidelización guardados correctamente.', 'success');
    } catch (err: any) {
      addToast('Error al guardar configuración de fidelización.', 'error');
    } finally {
      isSavingLoyalty = false;
    }
  }

  async function handleTestDb() {
    isTestingDb = true;
    dbTestMessage = 'Conectando al servidor MySQL 5.5...';
    dbTestSuccess = null;

    // Standardize database to 'pv' and clean whitespace
    localDb.database = 'pv';
    localDb.host = localDb.host.trim();
    localDb.user = localDb.user.trim();
    localDb.password = localDb.password || '';

    try {
      // 1. Write config file
      await saveDbConfig(localDb);
      dbConfigStore.set(localDb);

      // 2. Test live connection with updated credentials
      const connected = await checkDbConnection(localDb);
      dbTestSuccess = connected;
      isDbConnected.set(connected);

      if (connected) {
        dbTestMessage = '¡Conexión verificada con éxito a la base de datos POS!';
        addToast('Conexión con MySQL 5.5 exitosa', 'success');
      } else {
        dbTestMessage = 'No se pudo conectar. Verifica que el servidor MySQL 5.5 esté corriendo en el puerto indicado y que el usuario y la contraseña sean correctos.';
        addToast('Falló la conexión a la base de datos', 'error');
      }
    } catch (err: any) {
      dbTestSuccess = false;
      const errMsg = typeof err === 'string' ? err : err?.message || JSON.stringify(err);
      dbTestMessage = 'Error de conexión: ' + errMsg;
      addToast('Error al conectar con MySQL: ' + errMsg, 'error');
    } finally {
      isTestingDb = false;
    }
  }

  async function handleSaveDb() {
    isSavingDb = true;
    await handleTestDb();
    isSavingDb = false;
  }
</script>

<div class="config-page">
  <header class="page-header">
    <div>
      <h1 class="page-title">Configuración del Sistema</h1>
      <p class="page-subtitle">Ajuste las reglas financieras de conversión de puntos y la conexión local a MySQL 5.5 POS.</p>
    </div>
  </header>

  <div class="config-grid">
    <!-- 1. Reglas de Fidelización COP -->
    <div class="card config-card">
      <div class="card-header">
        <Settings size={20} color="#059669" />
        <h2 class="card-title">Reglas de Conversión & Puntos COP</h2>
      </div>

      <form on:submit|preventDefault={handleSaveLoyalty} class="form-body">
        <div class="form-group">
          <label for="montoPunto" class="form-label">Monto en COP por cada 1 Punto Ganado ($)</label>
          <input 
            id="montoPunto"
            type="number"
            min="1"
            bind:value={localLoyalty.monto_por_punto}
            class="form-input font-mono"
            required
          />
          <span class="form-hint">Ejemplo: $1.000 COP acumulados en ventas Otorgan 1 Punto.</span>
        </div>

        <div class="form-group">
          <label for="valorPunto" class="form-label">Valor en Descuento COP por cada Punto Redimido ($)</label>
          <input 
            id="valorPunto"
            type="number"
            min="1"
            bind:value={localLoyalty.valor_punto_cop}
            class="form-input font-mono"
            required
          />
          <span class="form-hint">Ejemplo: 1 Punto equivale a $50 COP de descuento en la factura.</span>
        </div>

        <div class="form-group">
          <label for="minCompra" class="form-label">Compra Mínima Elegible en Factura ($)</label>
          <input 
            id="minCompra"
            type="number"
            min="0"
            bind:value={localLoyalty.min_compra_puntos}
            class="form-input font-mono"
            required
          />
          <span class="form-hint">Facturas menores a este valor no acumulan puntos.</span>
        </div>

        <div class="form-actions mt-2">
          <button type="submit" disabled={isSavingLoyalty} class="btn btn-primary w-full">
            {#if isSavingLoyalty}
              <Loader2 size={16} class="animate-spin" />
              <span>Guardando...</span>
            {:else}
              <Save size={16} />
              <span>Guardar Parámetros COP</span>
            {/if}
          </button>
        </div>
      </form>
    </div>

    <!-- 2. Conexión MySQL 5.5 POS -->
    <div class="card config-card">
      <div class="card-header">
        <Database size={20} color="#2563eb" />
        <h2 class="card-title">Conexión Base de Datos Local MySQL 5.5</h2>
      </div>

      <form on:submit|preventDefault={handleSaveDb} class="form-body">
        <div class="form-row">
          <div class="form-group flex-2">
            <label for="dbHost" class="form-label">Servidor / Host MySQL</label>
            <div class="input-with-icon">
              <span class="input-icon"><Server size={15} color="#64748b" /></span>
              <input 
                id="dbHost"
                type="text" 
                bind:value={localDb.host}
                placeholder="127.0.0.1"
                class="form-input font-mono icon-padding"
                required
              />
            </div>
          </div>

          <div class="form-group flex-1">
            <label for="dbPort" class="form-label">Puerto</label>
            <input 
              id="dbPort"
              type="number" 
              bind:value={localDb.port}
              placeholder="3306"
              class="form-input font-mono"
              required
            />
          </div>
        </div>

        <div class="form-row">
          <div class="form-group flex-1">
            <label for="dbUser" class="form-label">Usuario MySQL</label>
            <input 
              id="dbUser"
              type="text" 
              bind:value={localDb.user}
              placeholder="root"
              class="form-input font-mono"
              required
            />
          </div>

          <div class="form-group flex-1">
            <label for="dbPass" class="form-label">Contraseña</label>
            <div class="input-with-icon">
              <span class="input-icon"><Key size={15} color="#64748b" /></span>
              <input 
                id="dbPass"
                type="password" 
                bind:value={localDb.password}
                placeholder="••••••••"
                class="form-input font-mono icon-padding"
              />
            </div>
          </div>
        </div>

        <!-- Status Banner -->
        {#if dbTestMessage}
          <div class="test-banner {dbTestSuccess ? 'success' : dbTestSuccess === false ? 'error' : 'info'}">
            {#if dbTestSuccess}
              <CheckCircle2 size={16} color="#059669" />
            {:else if dbTestSuccess === false}
              <AlertTriangle size={16} color="#dc2626" />
            {:else}
              <Loader2 size={16} class="animate-spin text-blue" />
            {/if}
            <span>{dbTestMessage}</span>
          </div>
        {/if}

        <div class="btn-group mt-2">
          <button type="button" on:click={handleTestDb} disabled={isTestingDb} class="btn btn-secondary flex-1">
            {#if isTestingDb}
              <Loader2 size={15} class="animate-spin" />
              <span>Probando...</span>
            {:else}
              <Shield size={15} />
              <span>Probar Conexión</span>
            {/if}
          </button>

          <button type="submit" disabled={isSavingDb || isTestingDb} class="btn btn-primary flex-1">
            <span>Guardar & Conectar</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</div>

<style>
  .config-page {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    padding-bottom: 16px;
    border-bottom: 1px solid #e2e8f0;
  }

  .page-title {
    font-size: 22px;
    font-weight: 800;
    color: #0f172a;
  }

  .page-subtitle {
    font-size: 13px;
    color: #64748b;
    margin-top: 4px;
  }

  .config-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 24px;
  }

  @media (max-width: 960px) {
    .config-grid {
      grid-template-columns: 1fr;
    }
  }

  .config-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 14px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.05);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding-bottom: 14px;
    border-bottom: 1px solid #e2e8f0;
  }

  .card-title {
    font-size: 16px;
    font-weight: 700;
    color: #0f172a;
  }

  .form-body {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .flex-1 { flex: 1; }
  .flex-2 { flex: 2; }
  .w-full { width: 100%; }

  .form-label {
    font-size: 11.5px;
    font-weight: 700;
    color: #475569;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .input-with-icon {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .input-icon {
    position: absolute;
    left: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    z-index: 2;
  }

  .form-input {
    width: 100%;
    height: 42px;
    padding: 10px 14px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 14px;
    color: #0f172a;
    outline: none;
    transition: all 150ms ease;
  }

  .form-input.icon-padding {
    padding-left: 36px;
  }

  .form-input:focus {
    border-color: #059669;
    background: #ffffff;
    box-shadow: 0 0 0 3px rgba(5, 150, 105, 0.15);
  }

  .form-hint {
    font-size: 11px;
    color: #64748b;
  }

  .test-banner {
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .test-banner.success { background: #ecfdf5; border: 1px solid #a7f3d0; color: #065f46; }
  .test-banner.error { background: #fef2f2; border: 1px solid #fecaca; color: #991b1b; }
  .test-banner.info { background: #eff6ff; border: 1px solid #bfdbfe; color: #1e40af; }

  .btn-group {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .mt-2 { margin-top: 8px; }
</style>
