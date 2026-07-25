<script lang="ts">
  import { isRedeemModalOpen, selectedCustomer, customerPoints, pointsHistory, toasts } from '../stores/appStore';
  import { api } from '../api';
  import { X, Gift, AlertCircle, CheckCircle2, ArrowRight, ShieldCheck, Loader2 } from 'lucide-svelte';

  let pointsToRedeem: number = 0;
  let invoiceRef: string = '';
  let note: string = '';
  let isSubmitting = false;

  $: valorPunto = $customerPoints?.valor_punto_cop || 50;
  $: copDiscount = pointsToRedeem * valorPunto;
  $: maxAvailable = $customerPoints?.available_points || 0;

  $: isValid = pointsToRedeem > 0 && pointsToRedeem <= maxAvailable && invoiceRef.trim().length > 0;

  function close() {
    isRedeemModalOpen.set(false);
    pointsToRedeem = 0;
    invoiceRef = '';
    note = '';
  }

  function setMaxPoints() {
    pointsToRedeem = maxAvailable;
  }

  async function handleSubmit() {
    if (!isValid || !$selectedCustomer) return;

    isSubmitting = true;
    try {
      const res = await api.redeemPoints($selectedCustomer.id, pointsToRedeem, invoiceRef, note);
      
      if (res.success) {
        toasts.add(`Redención procesada con éxito: ${res.transaction_id}`, 'success');
        
        // Refresh customer points & history
        const updatedPoints = await api.getCustomerPoints($selectedCustomer.id);
        const updatedHistory = await api.getPointsHistory($selectedCustomer.id);
        customerPoints.set(updatedPoints);
        pointsHistory.set(updatedHistory);

        close();
      } else {
        toasts.add('Error al procesar redención: ' + res.message, 'error');
      }
    } catch (err: any) {
      toasts.add('Error en IPC redención: ' + err?.message, 'error');
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
  class:open={$isRedeemModalOpen}
  on:click|self={close}
  on:keydown={e => (e.key === 'Escape' || e.key === 'Enter') && close()}
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
        on:click={close}
        class="p-2 rounded-lg text-slate-400 hover:text-slate-100 hover:bg-slate-800/80 transition-colors"
      >
        <X size={20} />
      </button>
    </div>

    {#if $selectedCustomer && $customerPoints}
      <!-- Selected Customer Brief Card -->
      <div class="p-4 rounded-xl bg-deep border border-subtle space-y-2">
        <div class="flex justify-between items-center text-xs">
          <span class="text-slate-400">Cliente Activo:</span>
          <span class="font-mono font-bold text-emerald-400">{$selectedCustomer.doc_num}</span>
        </div>
        <div class="font-bold text-slate-100 text-sm">{$selectedCustomer.name}</div>
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
              <span class="text-xs font-semibold text-emerald-400 uppercase tracking-wider">Descuento Equivalent en COP</span>
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
            on:click={close}
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
