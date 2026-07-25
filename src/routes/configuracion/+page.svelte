<script lang="ts">
  import { onMount } from 'svelte';
  import { isDbConnected, dbConfigStore, loyaltyConfigStore, addToast } from '$lib/stores/appStore';
  import { saveDbConfig, checkDbConnection, getLoyaltyConfig, saveLoyaltyConfig } from '$lib/api';
  import { Settings, Database, Save, RefreshCw, Award, CheckCircle2, AlertTriangle } from 'lucide-svelte';
  import type { DbConfig, LoyaltyConfig } from '$lib/api';

  let localDbConfig: DbConfig = { ...$dbConfigStore };
  let localLoyaltyConfig: LoyaltyConfig = { ...$loyaltyConfigStore };
  let isTestingConnection = false;

  onMount(async () => {
    try {
      const cfg = await getLoyaltyConfig();
      if (cfg) {
        loyaltyConfigStore.set(cfg);
        localLoyaltyConfig = { ...cfg };
      }
    } catch (e) {
      console.warn('Error al cargar parámetros:', e);
    }
  });

  async function handleSaveDb() {
    isTestingConnection = true;
    try {
      await saveDbConfig(localDbConfig);
      dbConfigStore.set(localDbConfig);

      const connected = await checkDbConnection();
      isDbConnected.set(connected);

      if (connected) {
        addToast('Conexión con MySQL 5.5 verificada y guardada con éxito.', 'success');
      } else {
        addToast('Configuración guardada pero no se pudo establecer conexión con MySQL.', 'error');
      }
    } catch (err: any) {
      addToast(`Error al guardar credenciales: ${err?.message || err}`, 'error');
      isDbConnected.set(false);
    } finally {
      isTestingConnection = false;
    }
  }

  async function handleSaveLoyalty() {
    try {
      await saveLoyaltyConfig(localLoyaltyConfig);
      loyaltyConfigStore.set(localLoyaltyConfig);
      addToast('Parámetros de conversión y fidelización guardados con éxito.', 'success');
    } catch (err: any) {
      addToast(`Error al guardar parámetros: ${err?.message || err}`, 'error');
    }
  }

  function formatCOP(val: number) {
    return new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(val);
  }
</script>

<div class="config-view flex flex-col gap-6">
  <!-- Page Header -->
  <header class="flex items-center justify-between pb-4 border-b border-slate-800">
    <div>
      <div class="flex items-center gap-2">
        <Settings class="text-emerald-400" size={24} />
        <h1 class="text-2xl font-extrabold text-slate-100">Configuración & Parámetros</h1>
      </div>
      <p class="text-xs text-slate-400 mt-1">Gestión de reglas de conversión de puntos y credenciales de la base de datos MySQL 5.5 del POS.</p>
    </div>
  </header>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Panel 1: Parámetros de Fidelización -->
    <div class="card flex flex-col gap-5">
      <div class="flex items-center gap-3 pb-3 border-b border-slate-800">
        <div class="p-2 rounded-lg bg-emerald-500/10 text-emerald-400">
          <Award size={20} />
        </div>
        <div>
          <h3 class="text-base font-bold text-slate-100">Parámetros de Puntos & Canjes</h3>
          <p class="text-xs text-slate-400">Define las equivalencias financieras del programa de puntos.</p>
        </div>
      </div>

      <form on:submit|preventDefault={handleSaveLoyalty} class="flex flex-col gap-4">
        <div class="form-group">
          <label class="form-label" for="monto_por_punto">Monto COP necesario para ganar 1 Punto</label>
          <input
            id="monto_por_punto"
            type="number"
            step="100"
            min="100"
            bind:value={localLoyaltyConfig.monto_por_punto}
            class="form-input"
            required
          />
          <span class="text-[11px] text-slate-500">Ejemplo: Por cada {formatCOP(localLoyaltyConfig.monto_por_punto || 1000)} comprados, el cliente gana 1 punto.</span>
        </div>

        <div class="form-group">
          <label class="form-label" for="valor_punto_cop">Valor COP equivalente de 1 Punto al Redimir</label>
          <input
            id="valor_punto_cop"
            type="number"
            step="5"
            min="1"
            bind:value={localLoyaltyConfig.valor_punto_cop}
            class="form-input"
            required
          />
          <span class="text-[11px] text-slate-500">Ejemplo: 100 puntos redimidos equivalen a {formatCOP((localLoyaltyConfig.valor_punto_cop || 50) * 100)} de descuento.</span>
        </div>

        <div class="form-group">
          <label class="form-label" for="min_compra">Monto Mínimo de Factura para Acumular Puntos</label>
          <input
            id="min_compra"
            type="number"
            step="1000"
            min="0"
            bind:value={localLoyaltyConfig.min_compra_puntos}
            class="form-input"
            required
          />
          <span class="text-[11px] text-slate-500">Facturas por debajo de este monto no otorgarán puntos.</span>
        </div>

        <button type="submit" class="btn btn-primary mt-2">
          <Save size={16} />
          Guardar Parámetros de Puntos
        </button>
      </form>
    </div>

    <!-- Panel 2: Conexión MySQL 5.5 -->
    <div class="card flex flex-col gap-5">
      <div class="flex items-center justify-between pb-3 border-b border-slate-800">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-blue-500/10 text-blue-400">
            <Database size={20} />
          </div>
          <div>
            <h3 class="text-base font-bold text-slate-100">Base de Datos MySQL 5.5 POS</h3>
            <p class="text-xs text-slate-400">Credenciales del servidor local GsigmaPOS / Itheke.</p>
          </div>
        </div>

        <div class="badge {$isDbConnected ? 'badge-success' : 'badge-danger'}">
          <span class="dot-pulse {$isDbConnected ? 'dot-pulse-green' : 'dot-pulse-red'}"></span>
          {$isDbConnected ? 'Conectado' : 'Desconectado'}
        </div>
      </div>

      <form on:submit|preventDefault={handleSaveDb} class="flex flex-col gap-4">
        <div class="grid grid-cols-3 gap-3">
          <div class="form-group col-span-2">
            <label class="form-label" for="db_host">Host / Servidor IP</label>
            <input id="db_host" type="text" bind:value={localDbConfig.host} class="form-input" placeholder="127.0.0.1" required />
          </div>
          <div class="form-group">
            <label class="form-label" for="db_port">Puerto</label>
            <input id="db_port" type="number" bind:value={localDbConfig.port} class="form-input" placeholder="3306" required />
          </div>
        </div>

        <div class="form-group">
          <label class="form-label" for="db_user">Usuario MySQL</label>
          <input id="db_user" type="text" bind:value={localDbConfig.user} class="form-input" placeholder="root" required />
        </div>

        <div class="form-group">
          <label class="form-label" for="db_pass">Contraseña</label>
          <input id="db_pass" type="password" bind:value={localDbConfig.password} class="form-input" placeholder="••••••••" />
        </div>

        <div class="form-group">
          <label class="form-label" for="db_name">Nombre de Base de Datos</label>
          <input id="db_name" type="text" bind:value={localDbConfig.database} class="form-input" placeholder="pv" required />
        </div>

        <button type="submit" class="btn btn-secondary mt-2" disabled={isTestingConnection}>
          {#if isTestingConnection}
            <RefreshCw size={16} class="animate-spin text-emerald-400" />
            Verificando Conexión...
          {:else}
            <Save size={16} />
            Guardar & Probar Conexión
          {/if}
        </button>
      </form>
    </div>
  </div>
</div>
