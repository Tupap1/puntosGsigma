<script lang="ts">
  import { toasts } from '../stores/appStore';
  import { CheckCircle2, AlertCircle, Info, X } from 'lucide-svelte';
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <div 
      class="toast"
      class:toast-success={toast.type === 'success'}
      class:toast-error={toast.type === 'error'}
      class:toast-info={toast.type === 'info'}
    >
      {#if toast.type === 'success'}
        <CheckCircle2 size={18} class="text-emerald-400 shrink-0" />
      {:else if toast.type === 'error'}
        <AlertCircle size={18} class="text-red-400 shrink-0" />
      {:else}
        <Info size={18} class="text-blue-400 shrink-0" />
      {/if}

      <span class="flex-1 font-medium">{toast.message}</span>

      <button 
        on:click={() => toasts.remove(toast.id)}
        class="text-slate-400 hover:text-slate-200 p-0.5 rounded"
      >
        <X size={14} />
      </button>
    </div>
  {/each}
</div>
