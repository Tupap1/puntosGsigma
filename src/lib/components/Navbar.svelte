<script lang="ts">
  import { dbConnected, isDbConfigModalOpen, toasts } from '../stores/appStore';
  import { api } from '../api';
  import { Database, Settings, Award, RefreshCw } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let checkingStatus = false;

  async function checkConnection() {
    checkingStatus = true;
    try {
      const res = await api.checkDbConnection();
      dbConnected.set(res.connected);
      if (res.connected) {
        toasts.add('Conexión BD establecida', 'success');
      } else {
        toasts.add('No se pudo conectar a la BD POS', 'error');
      }
    } catch (err: any) {
      dbConnected.set(false);
      toasts.add('Error al verificar conexión BD: ' + err?.message, 'error');
    } finally {
      checkingStatus = false;
    }
  }

  onMount(() => {
    checkConnection();
  });
</script>

<header class="navbar bg-surface border-b border-subtle px-6 py-3.5 flex items-center justify-between shadow-sm">
  <!-- Brand / Logo -->
  <div class="flex items-center gap-3">
    <div class="w-10 h-10 rounded-xl bg-gradient-to-tr from-emerald-600 to-emerald-400 flex items-center justify-center shadow-lg shadow-emerald-900/30 text-white font-bold">
      <Award size={22} class="stroke-[2.2]" />
    </div>
    <div>
      <h1 class="text-base font-bold text-slate-100 flex items-center gap-2">
        GSIGMA <span class="text-emerald-400 font-extrabold">POS PUNTOS</span>
      </h1>
      <p class="text-xs text-slate-400">Sistema de Fidelización y Ledger de Clientes</p>
    </div>
  </div>

  <!-- Actions & Status Badges -->
  <div class="flex items-center gap-4">
    <!-- Pulsing DB Status Badge -->
    <button 
      on:click={checkConnection}
      disabled={checkingStatus}
      class="badge cursor-pointer transition-transform hover:scale-105 active:scale-95"
      class:badge-success={$dbConnected}
      class:badge-danger={!$dbConnected}
      title="Haga clic para re-verificar estado de BD POS"
    >
      <span class="dot-pulse" class:dot-pulse-green={$dbConnected} class:dot-pulse-red={!$dbConnected}></span>
      <span>{$dbConnected ? 'BD Conectada' : 'BD Desconectada'}</span>
      {#if checkingStatus}
        <RefreshCw size={12} class="animate-spin ml-1" />
      {/if}
    </button>

    <!-- DB Config Settings Button -->
    <button 
      on:click={() => isDbConfigModalOpen.set(true)}
      class="btn btn-secondary text-xs py-1.5 px-3 flex items-center gap-2 border-muted hover:border-bright"
    >
      <Settings size={15} class="text-slate-400" />
      <span>Ajustes BD</span>
    </button>
  </div>
</header>

<style>
  .navbar {
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 24px;
  }
</style>
