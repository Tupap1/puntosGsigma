<script lang="ts">
  import { selectedCustomerStore, customerSummaryStore, addToast } from '$lib/stores/appStore';
  import { redeemPoints, getCustomerPointsSummary, getPointsHistory } from '$lib/api';
  import { X, Gift, AlertCircle, ShieldCheck, Loader2 } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void = () => {};
  export let onSuccess: () => void = () => {};

  let pointsToRedeem: number = 0;
  let invoiceRef: string = '';
  let note: string = '';
  let isSubmitting = false;

  $: valorPunto = $customerSummaryStore?.valor_punto_cop || 50;
  $: copDiscount = pointsToRedeem * valorPunto;
  $: maxAvailable = $customerSummaryStore?.saldo_actual || 0;

  $: isValid = pointsToRedeem > 0 && pointsToRedeem <= maxAvailable && invoiceRef.trim().length > 0;

  function handleClose() {
    pointsToRedeem = 0;
    invoiceRef = '';
    note = '';
    onClose();
  }

  function setMaxPoints() {
    pointsToRedeem = maxAvailable;
  }

  async function handleSubmit() {
    if (!isValid || !$selectedCustomerStore) return;

    isSubmitting = true;
    try {
      const res = await redeemPoints($selectedCustomerStore.trcid, pointsToRedeem, invoiceRef, note);
      
      addToast(`Redención procesada con éxito: Ref ${res.referencia_doc || 'OK'}`, 'success');
      onSuccess();
      handleClose();
    } catch (err: any) {
      console.warn('Error al redimir puntos:', err);
      addToast('Error al procesar la redención de puntos: ' + (err?.message || err), 'error');
    } finally {
      isSubmitting = false;
    }
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('es-CO', {
      style: 'currency',
      currency: 'COP',
      maximumFractionDigits: 0
    }).format(amount);
  }
</script>

<!-- Drawer Backdrop Overlay -->
<div 
  class="drawer-backdrop" 
  class:open={isOpen}
  on:click|self={handleClose}
  on:keydown={e => (e.key === 'Escape' || e.key === 'Enter') && handleClose()}
  role="button"
  tabindex="0"
>
  <!-- Slide-Over Drawer Content -->
  <aside class="drawer-content p-6 space-y-6">
    <!-- Drawer Header -->
    <div class="flex items-center justify-between border-b border-subtle pb-4">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 flex items-center justify-center">
          <Gift size={20} />
        </div>
        <div>
          <h2 class="text-lg font-bold text-slate-100">Redención de Puntos POS</h2>
          <p class="text-xs text-slate-400">Canjear saldo acumulado por descuento en factura</p>
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

    {#if $selectedCustomerStore && $customerSummaryStore}
      {@const fullName = `${$selectedCustomerStore.trcnom} ${$selectedCustomerStore.trcape}`.trim()}
      <!-- Selected Customer Brief Card -->
      <div class="p-4 rounded-xl bg-deep border border-subtle space-y-2">
        <div class="flex justify-between items-center text-xs">
          <span class="text-slate-400">Cliente Activo:</span>
          <span class="font-mono font-bold text-emerald-400">{$selectedCustomerStore.trcnumdoc}</span>
        </div>
        <div class="font-bold text-slate-100 text-sm">{fullName}</div>
        <div class="flex justify-between items-center text-xs border-t border-subtle/50 pt-2 mt-2">
          <span class="text-slate-400">Saldo Máximo Disponible:</span>
          <span class="font-extrabold text-emerald-400 text-base">{maxAvailable} PTS</span>
        </div>
      </div>

      <!-- Form Inputs -->
      <form on:submit|preventDefault={handleSubmit} class="space-y-4 flex-1 flex flex-col justify-between">
        <div class="space-y-4">
          <!-- Points Input -->
          <div class="form-group">
            <div class="flex justify-between items-center">
              <label for="pointsInput" class="form-label">Cantidad de Puntos a Redimir</label>
              <button 
                type="button" 
                on:click={setMaxPoints}
                class="text-xs font-semibold text-emerald-400 hover:underline"
              >
                Usar Máximo ({maxAvailable})
              </button>
            </div>
            <input 
              id="pointsInput"
              type="number"
              min="1"
              max={maxAvailable}
              bind:value={pointsToRedeem}
              placeholder="Ej: 500"
              class="form-input text-lg font-bold font-mono text-emerald-400"
            />
            {#if pointsToRedeem > maxAvailable}
              <p class="text-xs text-red-400 flex items-center gap-1 mt-1">
                <AlertCircle size={13} /> Excede el saldo disponible de {maxAvailable} PTS.
              </p>
            {/if}
          </div>

          <!-- Live COP Discount Preview -->
          <div class="p-4 rounded-xl bg-emerald-950/20 border border-emerald-500/30 flex items-center justify-between">
            <div>
              <span class="text-xs font-semibold text-emerald-400 uppercase tracking-wider">Descuento Equivalente en COP</span>
              <div class="text-2xl font-extrabold font-display text-emerald-400">
                {formatCurrency(copDiscount)}
              </div>
            </div>
            <div class="text-right text-[11px] text-slate-400">
              1 PT = ${valorPunto} COP
            </div>
          </div>

          <!-- Invoice Reference Input -->
          <div class="form-group">
            <label for="invoiceRef" class="form-label">Número / Ref. de Factura POS *</label>
            <input 
              id="invoiceRef"
              type="text"
              bind:value={invoiceRef}
              placeholder="Ej: FAC-9845 o VTA-102"
              class="form-input uppercase font-mono"
              required
            />
          </div>

          <!-- Note Input -->
          <div class="form-group">
            <label for="noteInput" class="form-label">Observaciones (Opcional)</label>
            <textarea 
              id="noteInput"
              bind:value={note}
              rows="2"
              placeholder="Ej: Autorizado por administración en caja 01"
              class="form-input text-xs resize-none"
            ></textarea>
          </div>
        </div>

        <!-- Footer Actions -->
        <div class="pt-4 border-t border-subtle flex items-center gap-3">
          <button 
            type="button"
            on:click={handleClose}
            class="btn btn-secondary flex-1"
          >
            Cancelar
          </button>

          <button 
            type="submit"
            disabled={!isValid || isSubmitting}
            class="btn btn-primary flex-1 py-3"
          >
            {#if isSubmitting}
              <Loader2 size={16} class="animate-spin" />
              <span>Procesando...</span>
            {:else}
              <ShieldCheck size={16} />
              <span>Confirmar Redención</span>
            {/if}
          </button>
        </div>
      </form>
    {/if}
  </aside>
</div>
