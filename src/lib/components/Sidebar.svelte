<script lang="ts">
  import { page } from '$app/stores';
  import { isDbConnected, dbConfigStore } from '$lib/stores/appStore';
  import { LayoutDashboard, Users, History, Settings, Database, Award } from 'lucide-svelte';

  export let onOpenDbModal: () => void = () => {};

  $: currentPath = $page.url.pathname;

  const navItems = [
    { href: '/', label: 'Panel General', icon: LayoutDashboard },
    { href: '/clientes', label: 'Clientes & Puntos', icon: Users },
    { href: '/historial', label: 'Historial Global', icon: History },
    { href: '/configuracion', label: 'Configuración & BD', icon: Settings },
  ];
</script>

<aside class="app-sidebar">
  <!-- Brand Logo -->
  <div class="sidebar-brand">
    <div class="brand-icon">
      <Award size={24} style="color: #10b981;" />
    </div>
    <div class="brand-info">
      <span class="brand-title">GSIGMA POS</span>
      <span class="brand-subtitle">Puntos & Fidelización</span>
    </div>
  </div>

  <!-- Navigation Links -->
  <nav class="sidebar-nav">
    <div class="nav-section-label">NAVEGACIÓN</div>
    {#each navItems as item}
      {@const isActive = currentPath === item.href || (item.href !== '/' && currentPath.startsWith(item.href))}
      <a href={item.href} class="nav-item {isActive ? 'active' : ''}">
        <svelte:component this={item.icon} size={18} class="nav-icon" />
        <span>{item.label}</span>
        {#if isActive}
          <div class="active-indicator"></div>
        {/if}
      </a>
    {/each}
  </nav>

  <!-- Database Connection Status Widget at Bottom -->
  <div class="sidebar-footer">
    <div class="db-status-card {$isDbConnected ? 'connected' : 'disconnected'}">
      <div class="db-status-header">
        <div class="db-info">
          <Database size={15} />
          <span>MySQL 5.5 POS</span>
        </div>
        <div class="badge {$isDbConnected ? 'badge-success' : 'badge-danger'}">
          <span class="dot-pulse {$isDbConnected ? 'dot-pulse-green' : 'dot-pulse-red'}"></span>
          {$isDbConnected ? 'Conectado' : 'Desconectado'}
        </div>
      </div>
      <div class="db-status-body">
        <p class="db-host">
          {$dbConfigStore.host}:{$dbConfigStore.port} ({$dbConfigStore.database})
        </p>
      </div>
      <button type="button" class="btn btn-secondary btn-sm db-btn" on:click={onOpenDbModal}>
        <Settings size={13} />
        Ajustes Conexión
      </button>
    </div>
  </div>
</aside>

<style>
  .app-sidebar {
    width: 260px;
    height: 100vh;
    background: var(--bg-deep);
    border-right: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    user-select: none;
  }

  .sidebar-brand {
    padding: 20px 18px;
    display: flex;
    align-items: center;
    gap: 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .brand-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 15px rgba(16, 185, 129, 0.2);
  }

  .brand-info {
    display: flex;
    flex-direction: column;
  }

  .brand-title {
    font-family: var(--font-display);
    font-weight: 800;
    font-size: 15px;
    letter-spacing: -0.01em;
    color: var(--text-main);
  }

  .brand-subtitle {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .sidebar-nav {
    flex: 1;
    padding: 20px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .nav-section-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--text-dim);
    padding: 0 12px 8px 12px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
    border-radius: 8px;
    color: var(--text-muted);
    font-weight: 500;
    font-size: 13.5px;
    text-decoration: none;
    transition: all var(--transition-fast);
    position: relative;
  }

  .nav-item:hover {
    background: var(--bg-surface-hover);
    color: var(--text-main);
  }

  .nav-item.active {
    background: var(--bg-surface-active);
    color: var(--accent-green);
    font-weight: 600;
  }

  .active-indicator {
    position: absolute;
    right: 0;
    top: 6px;
    bottom: 6px;
    width: 3px;
    background: var(--accent-green);
    border-radius: 4px 0 0 4px;
    box-shadow: 0 0 8px var(--accent-green);
  }

  .sidebar-footer {
    padding: 16px;
    border-top: 1px solid var(--border-subtle);
  }

  .db-status-card {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: 10px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .db-status-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .db-info {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
  }

  .db-host {
    font-size: 11px;
    color: var(--text-dim);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .db-btn {
    width: 100%;
    margin-top: 4px;
    font-size: 11px;
    padding: 6px 10px;
  }
</style>
