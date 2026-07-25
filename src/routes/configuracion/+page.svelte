<script lang="ts">
  import { onMount } from 'svelte';
  import { loyaltyConfigStore, dbConfigStore, isDbConnected, addToast } from '$lib/stores/appStore';
  import { getLoyaltyConfig, saveLoyaltyConfig, getDbConfig, saveDbConfig, checkDbConnection } from '$lib/api';
  import { Settings, Save, Database, Server, Key, Shield, CheckCircle2, AlertTriangle, Loader2 } from 'lucide-svelte';

  let isSavingLoyalty = false;
  let isSavingDb = false;
  let isTestingDb = false;
  let dbTestMessage = '';
  let dbTestSuccess: boolean | null = null;

  let localLoyalty = { ...$loyaltyConfigStore };
  let localDb = { ...$dbConfigStore };

  onMount(async () => {
    try {
      const cfg = await getLoyaltyConfig();
      if (cfg) {
        loyaltyConfigStore.set(cfg);
        localLoyalty = { ...cfg };
      }
      const dbCfg = await getDbConfig();
      if (dbCfg) {
        dbConfigStore.set(dbCfg);
        localDb = { ...dbCfg };
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
    try {
      await saveDbConfig(localDb);
      dbConfigStore.set(localDb);

      const connected = await checkDbConnection();
      dbTestSuccess = connected;
      isDbConnected.set(connected);

      if (connected) {
        dbTestMessage = '¡Conexión verificada con éxito a la base de datos POS!';
        addToast('Conexión con MySQL exitosa', 'success');
      } else {
        dbTestMessage = 'No se pudo conectar. Verifica que el servidor MySQL 5.5 esté corriendo y las credenciales sean válidas.';
        addToast('Falló la conexión a la base de datos', 'error');
      }
    } catch (err: any) {
      dbTestSuccess = false;
      dbTestMessage = 'Error de conexión: ' + (err?.message || 'Servidor inalcanzable');
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
        <Settings size={20} class="icon-emerald" />
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

        <button type="submit" disabled={isSavingLoyalty} class="btn btn-primary mt-4">
          {#if isSavingLoyalty}
            <Loader2 size={16} class="animate-spin" />
            <span>Guardando...</span>
          {:else}
            <Save size={16} />
            <span>Guardar Parámetros COP</span>
          {/if}
        </button>
      </form>
    </div>

    <!-- 2. Conexión MySQL 5.5 POS -->
    <div class="card config-card">
      <div class="card-header">
        <Database size={20} class="icon-blue" />
        <h2 class="card-title">Conexión Base de Datos Local MySQL 5.5</h2>
      </div>

      <form on:submit|preventDefault={handleSaveDb} class="form-body">
        <div class="form-row">
          <div class="form-group flex-2">
            <label for="dbHost" class="form-label">Servidor / Host MySQL</label>
            <div class="input-icon-wrapper">
              <Server size={15} class="icon-input" />
              <input 
                id="dbHost"
                type="text" 
                bind:value={localDb.host}
                placeholder="127.0.0.1"
                class="form-input pl-input font-mono"
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
            <div class="input-icon-wrapper">
              <Key size={15} class="icon-input" />
              <input 
                id="dbPass"
                type="password" 
                bind:value={localDb.password}
                placeholder="••••••••"
                class="form-input pl-input font-mono"
              />
            </div>
          </div>
        </div>

        <div class="form-group">
          <label for="dbName" class="form-label">Nombre de Base de Datos</label>
          <input 
            id="dbName"
            type="text" 
            bind:value={localDb.database}
            placeholder="pv"
            class="form-input font-mono"
            required
          />
        </div>

        <!-- Banner status -->
        {#if dbTestMessage}
          <div class="test-banner {dbTestSuccess ? 'success' : dbTestSuccess === false ? 'error' : 'info'}">
            {#if dbTestSuccess}
              <CheckCircle2 size={16} />
            {:else if dbTestSuccess === false}
              <AlertTriangle size={16} />
            {:else}
              <Loader2 size={16} class="animate-spin" />
            {/if}
            <span>{dbTestMessage}</span>
          </div>
        {/if}

        <div class="btn-group mt-4">
          <button type="button" on:click={handleTestDb} disabled={isTestingDb} class="btn btn-secondary">
            {#if isTestingDb}
              <Loader2 size={15} class="animate-spin" />
              <span>Probando...</span>
            {:else}
              <Shield size={15} />
              <span>Probar Conexión</span>
            {/if}
          </button>

          <button type="submit" disabled={isSavingDb || isTestingDb} class="btn btn-primary">
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

  @media (max-width: 900px) {
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

  .icon-emerald { color: #059669; }
  .icon-blue { color: #2563eb; }

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

  .form-label {
    font-size: 12px;
    font-weight: 700;
    color: #475569;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .form-input {
    width: 100%;
    padding: 10px 12px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 14px;
    color: #0f172a;
    outline: none;
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

  .input-icon-wrapper {
    position: relative;
    width: 100%;
  }

  .icon-input {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: #94a3b8;
  }

  .pl-input {
    padding-left: 36px;
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
    gap: 12px;
    justify-content: flex-end;
  }

  .mt-4 { margin-top: 16px; }
</style>
