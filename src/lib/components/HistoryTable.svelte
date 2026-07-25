<script lang="ts">
  import { customerHistoryStore, selectedCustomerStore } from '$lib/stores/appStore';
  import { History, ArrowUpRight, ArrowDownRight } from 'lucide-svelte';

  let filterType: 'all' | 'acumulacion' | 'canje' = 'all';

  $: filteredHistory = $customerHistoryStore.filter(item => {
    if (filterType === 'all') return true;
    return item.tipo === filterType;
  });

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
</script>

<div class="history-card">
  <!-- Table Header & Filter Controls -->
  <div class="history-header">
    <div class="title-group">
      <History size={18} class="icon-emerald" />
      <h2 class="history-title">Historial de Movimientos de Puntos</h2>
      <span class="count-badge">({filteredHistory.length} registros)</span>
    </div>

    <!-- Filter Buttons -->
    <div class="filter-group">
      <button 
        type="button"
        on:click={() => filterType = 'all'}
        class="filter-btn"
        class:active={filterType === 'all'}
      >
        Todos
      </button>
      <button 
        type="button"
        on:click={() => filterType = 'acumulacion'}
        class="filter-btn"
        class:active={filterType === 'acumulacion'}
      >
        Acumulaciones
      </button>
      <button 
        type="button"
        on:click={() => filterType = 'canje'}
        class="filter-btn"
        class:active={filterType === 'canje'}
      >
        Canjes / Redenciones
      </button>
    </div>
  </div>

  {#if !$selectedCustomerStore}
    <div class="empty-msg">
      Seleccione un cliente para consultar el historial detallado de movimientos.
    </div>
  {:else if filteredHistory.length === 0}
    <div class="empty-msg">
      No hay registros de movimientos para los filtros seleccionados.
    </div>
  {:else}
    <!-- High Speed Dense History Table -->
    <div class="dense-table-wrapper max-h-72">
      <table class="dense-table">
        <thead>
          <tr>
            <th>Fecha / Hora</th>
            <th>Tipo</th>
            <th>Factura / Ref</th>
            <th class="text-right">Puntos</th>
            <th class="text-right">Valor COP</th>
            <th>Detalles / Observaciones</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredHistory as item}
            <tr>
              <td class="font-mono text-date">{item.fecha}</td>
              <td>
                {#if item.tipo === 'acumulacion'}
                  <span class="badge badge-success">
                    <ArrowUpRight size={12} /> Venta POS
                  </span>
                {:else if item.tipo === 'canje'}
                  <span class="badge badge-amber">
                    <ArrowDownRight size={12} /> Redención
                  </span>
                {:else}
                  <span class="badge badge-danger">
                    Ajuste
                  </span>
                {/if}
              </td>
              <td class="font-mono font-ref">{item.referencia_doc || '-'}</td>
              <td class="text-right font-mono font-pts" class:text-green={item.puntos > 0} class:text-amber={item.puntos < 0}>
                {item.puntos > 0 ? `+${formatNumber(item.puntos)}` : formatNumber(item.puntos)}
              </td>
              <td class="text-right font-mono text-cop">
                {formatCurrency(item.monto_cop)}
              </td>
              <td class="text-details">
                {item.concepto || '-'}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .history-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 14px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.05);
  }

  .history-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 12px;
    border-bottom: 1px solid #e2e8f0;
  }

  .title-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .icon-emerald { color: #059669; }

  .history-title {
    font-size: 16px;
    font-weight: 700;
    color: #0f172a;
  }

  .count-badge {
    font-size: 12px;
    color: #64748b;
  }

  .filter-group {
    display: flex;
    align-items: center;
    gap: 4px;
    background: #f1f5f9;
    padding: 4px;
    border-radius: 8px;
    border: 1px solid #cbd5e1;
  }

  .filter-btn {
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: #64748b;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .filter-btn.active {
    background: #059669;
    color: #ffffff;
  }

  .empty-msg {
    padding: 24px;
    text-align: center;
    font-size: 12px;
    color: #64748b;
    font-style: italic;
  }

  .max-h-72 {
    max-height: 300px;
    overflow-y: auto;
  }

  .text-date { color: #64748b; font-size: 12px; }
  .font-ref { font-weight: 700; color: #0f172a; }
  .font-pts { font-weight: 800; font-size: 13px; }
  .text-green { color: #059669; }
  .text-amber { color: #d97706; }
  .text-cop { color: #0f172a; }
  .text-details { color: #64748b; font-size: 12px; }
</style>
