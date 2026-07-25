<script lang="ts">
  import { selectedCustomer, customerPoints, isRedeemModalOpen, toasts } from '../stores/appStore';
  import { api } from '../api';
  import { Coins, ArrowUpRight, ArrowDownRight, DollarSign, Gift, RefreshCw, UserCheck } from 'lucide-svelte';

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
    if (!$selectedCustomer) return;
    isRefreshing = true;
    try {
      const points = await api.getCustomerPoints($selectedCustomer.id);
      customerPoints.set(points);
      toasts.add('Saldo de puntos actualizado', 'info');
    } catch (err: any) {
      toasts.add('Error al actualizar puntos: ' + err?.message, 'error');
    } finally {
      isRefreshing = false;
    }
  }
</script>

<div class="card space-y-5 bg-surface border-subtle relative overflow-hidden">
  <!-- Decorative Background Glow -->
  <div class="absolute -right-16 -top-16 w-48 h-48 bg-emerald-500/10 rounded-full blur-3xl pointer-events-none"></div>

  {#if !$selectedCustomer}
    <!-- Empty State -->
    <div class="py-10 text-center space-y-3">
      <div class="w-14 h-14 rounded-full bg-slate-800/80 border border-slate-700 mx-auto flex items-center justify-center text-slate-400">
        <UserCheck size={28} />
      </div>
      <h3 class="text-base font-bold text-slate-300">Ningún Cliente Seleccionado</h3>
      <p class="text-xs text-slate-500 max-w-xs mx-auto">
        Busque y seleccione un cliente en el panel superior para consultar su estado financiero y saldo acumulado de puntos.
      </p>
    </div>
  {:else}
    <!-- Customer Info Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-subtle pb-4">
      <div>
        <div class="flex items-center gap-2">
          <span class="px-2 py-0.5 rounded text-xs font-mono font-bold bg-slate-800 text-emerald-400 border border-slate-700">
            {$selectedCustomer.doc_num}
          </span>
          <h3 class="text-lg font-bold text-slate-100">{$selectedCustomer.name}</h3>
        </div>
        <p class="text-xs text-slate-400 mt-0.5">
          Tel: {$selectedCustomer.phone || 'N/A'} • Email: {$selectedCustomer.email || 'N/A'}
        </p>
      </div>

      <button 
        on:click={refreshPoints}
        disabled={isRefreshing}
        class="btn btn-secondary btn-sm flex items-center gap-1.5 self-start sm:self-auto"
        title="Actualizar saldo de puntos en tiempo real"
      >
        <RefreshCw size={14} class={isRefreshing ? 'animate-spin text-emerald-400' : 'text-slate-400'} />
        <span>Actualizar</span>
      </button>
    </div>

    <!-- Main Balance Breakdown Cards Grid -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <!-- Puntos Ganados -->
      <div class="p-4 rounded-xl bg-deep/80 border border-subtle/80 flex flex-col justify-between">
        <div class="flex items-center justify-between text-slate-400 text-xs font-semibold">
          <span>PUNTOS GANADOS</span>
          <ArrowUpRight size={16} class="text-emerald-400" />
        </div>
        <div class="mt-3">
          <span class="text-2xl font-bold font-display text-slate-100">
            {formatNumber($customerPoints?.points_earned || 0)}
          </span>
          <span class="text-xs text-slate-400 ml-1">pts</span>
        </div>
        <p class="text-[11px] text-slate-500 mt-1">Acumulado bruto por facturación</p>
      </div>

      <!-- Puntos Redimidos -->
      <div class="p-4 rounded-xl bg-deep/80 border border-subtle/80 flex flex-col justify-between">
        <div class="flex items-center justify-between text-slate-400 text-xs font-semibold">
          <span>PUNTOS REDIMIDOS</span>
          <ArrowDownRight size={16} class="text-amber-400" />
        </div>
        <div class="mt-3">
          <span class="text-2xl font-bold font-display text-slate-100">
            {formatNumber($customerPoints?.points_redeemed || 0)}
          </span>
          <span class="text-xs text-slate-400 ml-1">pts</span>
        </div>
        <p class="text-[11px] text-slate-500 mt-1">Total canjeado en facturas</p>
      </div>

      <!-- Saldo Disponible (Highlight Emerald) -->
      <div class="p-4 rounded-xl bg-emerald-950/30 border border-emerald-500/40 flex flex-col justify-between shadow-lg shadow-emerald-950/40 relative">
        <div class="flex items-center justify-between text-emerald-400 text-xs font-bold uppercase">
          <span>SALDO DISPONIBLE</span>
          <Coins size={18} class="text-emerald-400 animate-pulse" />
        </div>
        <div class="mt-3">
          <span class="text-3xl font-extrabold font-display text-emerald-400 drop-shadow">
            {formatNumber($customerPoints?.available_points || 0)}
          </span>
          <span class="text-xs font-semibold text-emerald-300 ml-1">PTS</span>
        </div>
        <p class="text-[11px] text-emerald-300/70 mt-1">Listos para ser canjeados</p>
      </div>

      <!-- Valor COP Equivalente -->
      <div class="p-4 rounded-xl bg-deep/80 border border-subtle/80 flex flex-col justify-between">
        <div class="flex items-center justify-between text-slate-400 text-xs font-semibold">
          <span>VALOR EN PESOS COP</span>
          <DollarSign size={16} class="text-emerald-400" />
        </div>
        <div class="mt-3">
          <span class="text-2xl font-bold font-display text-slate-100">
            {formatCurrency($customerPoints?.cop_value || 0)}
          </span>
        </div>
        <p class="text-[11px] text-slate-500 mt-1">
          Regla: 1 pt = ${$customerPoints?.valor_punto_cop || 50} COP
        </p>
      </div>
    </div>

    <!-- Action Bar -->
    <div class="flex items-center justify-between pt-2 border-t border-subtle/60">
      <div class="text-xs text-slate-400 flex items-center gap-1.5">
        <Gift size={14} class="text-emerald-400" />
        <span>Seleccione el botón para aplicar descuento de puntos en la factura POS activa.</span>
      </div>

      <button 
        on:click={() => isRedeemModalOpen.set(true)}
        disabled={!$customerPoints || $customerPoints.available_points <= 0}
        class="btn btn-primary btn-lg shadow-lg flex items-center gap-2"
      >
        <Gift size={18} />
        <span>Procesar Redención de Puntos</span>
      </button>
    </div>
  {/if}
</div>
