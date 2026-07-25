<script lang="ts">
  import { dbConfigStore, isDbConnected, addToast } from '$lib/stores/appStore';
  import { saveDbConfig, checkDbConnection, type DbConfig } from '$lib/api';
  import { X, Database, Server, Key, Shield, CheckCircle2, AlertTriangle, Loader2 } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void = () => {};

  let localConfig: DbConfig = { ...$dbConfigStore, database: 'pv' };
  let isTesting = false;
  let testMessage = '';
  let testSuccess: boolean | null = null;

  $: if (isOpen) {
    localConfig = { ...$dbConfigStore, database: 'pv' };
  }

  function handleClose() {
    testMessage = '';
    testSuccess = null;
    onClose();
  }

  async function testConnection() {
    isTesting = true;
    testMessage = 'Conectando al servidor MySQL...';
    testSuccess = null;
    localConfig.database = 'pv';

    try {
      await saveDbConfig(localConfig);
      dbConfigStore.set(localConfig);

      const connected = await checkDbConnection();
      testSuccess = connected;
      isDbConnected.set(connected);

      if (connected) {
        testMessage = '¡Conexión exitosa a MySQL 5.5 POS!';
        addToast('Conexión con MySQL verificada con éxito', 'success');
      } else {
        testMessage = 'No se pudo conectar. Verifica credenciales y puerto del servidor.';
        addToast('Falló la conexión a MySQL', 'error');
      }
    } catch (err: any) {
      testSuccess = false;
      testMessage = 'Error de conexión: ' + (err?.message || 'Servidor inalcanzable');
      addToast('Error al probar conexión con MySQL', 'error');
    } finally {
      isTesting = false;
    }
  }

  async function handleSubmit() {
    await testConnection();
    if (testSuccess) {
      handleClose();
    }
  }
</script>

<!-- Modal Backdrop Overlay -->
<div 
  class="modal-backdrop"
  class:open={isOpen}
  on:click|self={handleClose}
  on:keydown={e => (e.key === 'Escape' || e.key === 'Enter') && handleClose()}
  role="button"
  tabindex="0"
>
  <!-- Modal Card -->
  <div class="modal-card p-6 space-y-5">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-subtle pb-4">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-blue-500/20 border border-blue-500/30 text-blue-400 flex items-center justify-center">
          <Database size={20} />
        </div>
        <div>
          <h2 class="text-lg font-bold text-slate-100">Configuración Base de Datos POS</h2>
          <p class="text-xs text-slate-400">Credenciales de conexión local MySQL 5.5 / Itheke</p>
        </div>
      </div>

      <button 
        type="button"
        on:click={handleClose}
        class="p-2 rounded-lg text-slate-400 hover:text-slate-100 hover:bg-slate-800/80 transition-colors"
      >
        <X size={20} />
      </button>
    </div>

    <!-- Form Inputs -->
    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
        <div class="form-group md:col-span-2">
          <label for="dbHost" class="form-label">Servidor / Host MySQL</label>
          <div class="relative">
            <Server size={15} class="absolute left-3 top-3 text-slate-500" />
            <input 
              id="dbHost"
              type="text" 
              bind:value={localConfig.host}
              placeholder="127.0.0.1"
              class="form-input pl-9 font-mono text-xs"
              required
            />
          </div>
        </div>

        <div class="form-group">
          <label for="dbPort" class="form-label">Puerto</label>
          <input 
            id="dbPort"
            type="number" 
            bind:value={localConfig.port}
            placeholder="3306"
            class="form-input font-mono text-xs"
            required
          />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div class="form-group">
          <label for="dbUser" class="form-label">Usuario MySQL</label>
          <input 
            id="dbUser"
            type="text" 
            bind:value={localConfig.user}
            placeholder="root"
            class="form-input font-mono text-xs"
            required
          />
        </div>

        <div class="form-group">
          <label for="dbPass" class="form-label">Contraseña</label>
          <div class="relative">
            <Key size={15} class="absolute left-3 top-3 text-slate-500" />
            <input 
              id="dbPass"
              type="password" 
              bind:value={localConfig.password}
              placeholder="••••••••"
              class="form-input pl-9 font-mono text-xs"
            />
          </div>
        </div>
      </div>

      <!-- Test Status Banner -->
      {#if testMessage}
        <div 
          class="p-3 rounded-lg border text-xs flex items-center gap-2.5 {testSuccess ? 'bg-emerald-950/30 border-emerald-500/40 text-emerald-300' : testSuccess === false ? 'bg-red-950/30 border-red-500/40 text-red-300' : 'bg-slate-900 border-slate-700'}"
        >
          {#if testSuccess}
            <CheckCircle2 size={16} class="text-emerald-400 shrink-0" />
          {:else if testSuccess === false}
            <AlertTriangle size={16} class="text-red-400 shrink-0" />
          {:else}
            <Loader2 size={16} class="animate-spin text-slate-400 shrink-0" />
          {/if}
          <span>{testMessage}</span>
        </div>
      {/if}

      <!-- Actions -->
      <div class="pt-3 border-t border-subtle flex items-center justify-between gap-3">
        <button 
          type="button"
          on:click={testConnection}
          disabled={isTesting}
          class="btn btn-secondary text-xs flex items-center gap-1.5"
        >
          {#if isTesting}
            <Loader2 size={14} class="animate-spin" />
            <span>Probando...</span>
          {:else}
            <Shield size={14} />
            <span>Probar Conexión</span>
          {/if}
        </button>

        <div class="flex items-center gap-2">
          <button 
            type="button"
            on:click={handleClose}
            class="btn btn-ghost text-xs"
          >
            Cancelar
          </button>
          <button 
            type="submit"
            disabled={isTesting}
            class="btn btn-primary text-xs"
          >
            Guardar & Conectar
          </button>
        </div>
      </div>
    </form>
  </div>
</div>
