<script lang="ts">
  import { selectedCustomerStore, customerSummaryStore, addToast, isDbConnected } from '$lib/stores/appStore';
  import { getCustomerPointsSummary } from '$lib/api';
  import { Coins, ArrowUpRight, ArrowDownRight, DollarSign, Gift, RefreshCw, UserCheck, AlertTriangle } from 'lucide-svelte';

  export let onOpenRedeemModal: () => void = () => {};

  let isRefreshing = false;

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('es-CO', {
      style: 'currency',
      currency: 'COP',
      maximumFractionDigits: 0
    }).format(amount);
  }

  function formatNumber(num: number): string {
    return new Intl.NumberFormat('es-CO').format(num);
  }

  async function refreshPoints() {
    if (!$selectedCustomerStore) return;
    isRefreshing = true;
    try {
      const summary = await getCustomerPointsSummary($selectedCustomerStore.trcid);
      customerSummaryStore.set(summary);
      addToast('Saldo de puntos actualizado.', 'info');
    } catch (err: any) {
      console.warn('Error al actualizar puntos:', err);
      addToast('Error al actualizar saldo de puntos.', 'error');
    } finally {
      isRefreshing = false;
    }
  }
</script>

<div class="card summary-wrapper">
  {#if !$selectedCustomerStore}
    <!-- Empty State -->
    <div class="empty-state">
      <div class="empty-icon">
        <UserCheck size={28} />
      </div>
      <h3 class="empty-title">Ningún Cliente Seleccionado</h3>
      <p class="empty-subtitle">
        Busque y seleccione un cliente en el buscador superior para consultar sus puntos acumulados y procesar canjes.
      </p>
    </div>
  {:else}
    {@const fullName = `${$selectedCustomerStore.trcnom} ${$selectedCustomerStore.trcape}`.trim()}
    
    <!-- Demo Mode Warning Banner if DB not connected -->
    {#if !$isDbConnected}
      <div class="demo-banner">
        <AlertTriangle size={16} />
        <span><strong>Modo Simulación (Sin Conexión a Base de Datos Local)</strong>: Mostrando datos de demostración. Configura tu base de datos en <strong>Configuración & BD</strong> para leer los clientes reales de tu POS.</span>
      </div>
    {/if}

    <!-- Customer Info Header -->
    <div class="customer-header">
      <div class="customer-info">
        <div class="customer-title-row">
          <span class="doc-badge">{$selectedCustomerStore.trcnumdoc}</span>
          <h3 class="customer-name">{fullName}</h3>
        </div>
        <p class="customer-contact">
          Tel: {$selectedCustomerStore.trctel1 || 'N/A'} • Email: {$selectedCustomerStore.trcema1 || 'N/A'}
        </p>
      </div>

      <button 
        on:click={refreshPoints}
        disabled={isRefreshing}
        class="btn btn-secondary btn-sm"
        title="Actualizar saldo de puntos en tiempo real"
      >
        <RefreshCw size={14} class={isRefreshing ? 'animate-spin' : ''} />
        <span>Actualizar</span>
      </button>
    </div>

    <!-- Main Balance Breakdown Cards Grid -->
    <div class="balance-grid">
      <!-- Puntos Ganados -->
      <div class="balance-card">
        <div class="card-label">
          <span>PUNTOS GANADOS</span>
          <ArrowUpRight size={16} color="#059669" />
        </div>
        <div class="card-value-row">
          <span class="card-value">{formatNumber($customerSummaryStore?.puntos_acumulados || 0)}</span>
          <span class="card-unit">pts</span>
        </div>
        <p class="card-hint">Acumulado bruto por facturación POS</p>
      </div>

      <!-- Puntos Redimidos -->
      <div class="balance-card">
        <div class="card-label">
          <span>PUNTOS REDIMIDOS</span>
          <ArrowDownRight size={16} color="#d97706" />
        </div>
        <div class="card-value-row">
          <span class="card-value">{formatNumber($customerSummaryStore?.puntos_redimidos || 0)}</span>
          <span class="card-unit">pts</span>
        </div>
        <p class="card-hint">Total canjeado en facturas</p>
      </div>

      <!-- Saldo Disponible (Highlight Emerald) -->
      <div class="balance-card highlight-card">
        <div class="card-label text-emerald">
          <span>SALDO DISPONIBLE</span>
          <Coins size={18} color="#059669" />
        </div>
        <div class="card-value-row">
          <span class="card-value text-emerald">{formatNumber($customerSummaryStore?.saldo_actual || 0)}</span>
          <span class="card-unit text-emerald">PTS</span>
        </div>
        <p class="card-hint text-emerald">Listos para ser canjeados</p>
      </div>

      <!-- Valor COP Equivalente -->
      <div class="balance-card">
        <div class="card-label">
          <span>VALOR EN PESOS COP</span>
          <DollarSign size={16} color="#059669" />
        </div>
        <div class="card-value-row">
          <span class="card-value">{formatCurrency($customerSummaryStore?.valor_cop_disponible || 0)}</span>
        </div>
        <p class="card-hint">Regla: 1 pt = ${$customerSummaryStore?.valor_punto_cop || 50} COP</p>
      </div>
    </div>

    <!-- Action Bar -->
    <div class="action-bar">
      <div class="action-hint">
        <Gift size={14} color="#059669" />
        <span>Aplica el descuento de puntos directamente en la factura activa.</span>
      </div>

      <button 
        on:click={onOpenRedeemModal}
        disabled={!$customerSummaryStore || $customerSummaryStore.saldo_actual <= 0}
        class="btn btn-primary btn-lg"
      >
        <Gift size={18} />
        <span>Procesar Redención de Puntos</span>
      </button>
    </div>
  {/if}
</div>

<style>
  .summary-wrapper {
    display: flex;
    flex-direction: column;
    gap: 20px;
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 14px;
    padding: 24px;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
  }

  .empty-state {
    padding: 32px 16px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .empty-icon {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: #f1f5f9;
    border: 1px solid #cbd5e1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #64748b;
  }

  .empty-title {
    font-size: 16px;
    font-weight: 700;
    color: #0f172a;
  }

  .empty-subtitle {
    font-size: 12px;
    color: #64748b;
    max-width: 320px;
  }

  .demo-banner {
    background: #fffbeb;
    border: 1px solid #fde68a;
    border-radius: 8px;
    padding: 10px 14px;
    font-size: 12px;
    color: #92400e;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .customer-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 16px;
    border-bottom: 1px solid #e2e8f0;
  }

  .customer-title-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .doc-badge {
    padding: 3px 8px;
    background: #0f172a;
    color: #34d399;
    font-family: monospace;
    font-weight: 700;
    font-size: 12px;
    border-radius: 6px;
  }

  .customer-name {
    font-size: 18px;
    font-weight: 800;
    color: #0f172a;
  }

  .customer-contact {
    font-size: 12px;
    color: #64748b;
    margin-top: 4px;
  }

  .balance-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 16px;
  }

  @media (max-width: 900px) {
    .balance-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  .balance-card {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .highlight-card {
    background: #ecfdf5;
    border: 1px solid #a7f3d0;
    box-shadow: 0 4px 12px rgba(5, 150, 105, 0.1);
  }

  .card-label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 11px;
    font-weight: 700;
    color: #64748b;
    letter-spacing: 0.04em;
  }

  .text-emerald { color: #059669 !important; }

  .card-value-row {
    margin-top: 10px;
    display: flex;
    align-items: baseline;
    gap: 4px;
  }

  .card-value {
    font-family: var(--font-display);
    font-size: 24px;
    font-weight: 800;
    color: #0f172a;
  }

  .card-unit {
    font-size: 12px;
    font-weight: 600;
    color: #64748b;
  }

  .card-hint {
    font-size: 11px;
    color: #64748b;
    margin-top: 6px;
  }

  .action-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 12px;
    border-top: 1px solid #e2e8f0;
  }

  .action-hint {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: #64748b;
  }
</style>
