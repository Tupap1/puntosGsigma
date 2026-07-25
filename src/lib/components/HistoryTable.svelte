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

<div class="card space-y-4">
  <!-- Table Header & Filter Controls -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 border-b border-subtle pb-3">
    <div class="flex items-center gap-2">
      <History size={18} class="text-emerald-400" />
      <h2 class="text-base font-bold text-slate-100">Historial de Movimientos de Puntos</h2>
      <span class="text-xs text-slate-400">({filteredHistory.length} registros)</span>
    </div>

    <!-- Filter Buttons -->
    <div class="flex items-center gap-1 bg-deep p-1 rounded-lg border border-subtle">
      <button 
        type="button"
        on:click={() => filterType = 'all'}
        class="px-2.5 py-1 text-xs font-semibold rounded-md transition-colors"
        class:bg-emerald-500={filterType === 'all'}
        class:text-white={filterType === 'all'}
        class:text-slate-400={filterType !== 'all'}
      >
        Todos
      </button>
      <button 
        type="button"
        on:click={() => filterType = 'acumulacion'}
        class="px-2.5 py-1 text-xs font-semibold rounded-md transition-colors"
        class:bg-emerald-500={filterType === 'acumulacion'}
        class:text-white={filterType === 'acumulacion'}
        class:text-slate-400={filterType !== 'acumulacion'}
      >
        Acumulaciones
      </button>
      <button 
        type="button"
        on:click={() => filterType = 'canje'}
        class="px-2.5 py-1 text-xs font-semibold rounded-md transition-colors"
        class:bg-emerald-500={filterType === 'canje'}
        class:text-white={filterType === 'canje'}
        class:text-slate-400={filterType !== 'canje'}
      >
        Canjes / Redenciones
      </button>
    </div>
  </div>

  {#if !$selectedCustomerStore}
    <div class="py-8 text-center text-slate-500 text-xs italic">
      Seleccione un cliente para consultar el historial detallado de movimientos.
    </div>
  {:else if filteredHistory.length === 0}
    <div class="py-8 text-center text-slate-500 text-xs italic">
      No hay registros de movimientos para los filtros seleccionados.
    </div>
  {:else}
    <!-- High Speed Dense History Table -->
    <div class="dense-table-wrapper max-h-72 overflow-y-auto">
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
              <td class="font-mono text-slate-400 text-xs">{item.fecha}</td>
              <td>
                {#if item.tipo === 'acumulacion'}
                  <span class="badge badge-success py-0.5 px-2 text-[11px]">
                    <ArrowUpRight size={12} /> Venta POS
                  </span>
                {:else if item.tipo === 'canje'}
                  <span class="badge badge-amber py-0.5 px-2 text-[11px]">
                    <ArrowDownRight size={12} /> Redención
                  </span>
                {:else}
                  <span class="badge badge-danger py-0.5 px-2 text-[11px]">
                    Ajuste
                  </span>
                {/if}
              </td>
              <td class="font-mono font-semibold text-slate-200">{item.referencia_doc || '-'}</td>
              <td class="text-right font-mono font-bold" class:text-emerald-400={item.puntos > 0} class:text-amber-400={item.puntos < 0}>
                {item.puntos > 0 ? `+${formatNumber(item.puntos)}` : formatNumber(item.puntos)}
              </td>
              <td class="text-right font-mono text-slate-300">
                {formatCurrency(item.monto_cop)}
              </td>
              <td class="text-slate-400 text-xs">
                {item.concepto || '-'}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
