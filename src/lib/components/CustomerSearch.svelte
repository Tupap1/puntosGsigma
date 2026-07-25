<script lang="ts">
  import { searchCustomers, getCustomerPointsSummary, getPointsHistory, type Customer } from '$lib/api';
  import { selectedCustomerStore, customerSummaryStore, customerHistoryStore, addToast } from '$lib/stores/appStore';
  import { Search, X, User, Phone, Mail, CheckCircle2, Loader2 } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let searchQuery = '';
  let searchResults: Customer[] = [];
  let isSearching = false;
  let debounceTimer: any = null;

  function handleInput() {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      performSearch();
    }, 300);
  }

  async function performSearch() {
    isSearching = true;
    try {
      searchResults = await searchCustomers(searchQuery);
    } catch (err: any) {
      console.warn('Error en búsqueda de clientes:', err);
      addToast('Error al consultar la tabla de clientes.', 'error');
    } finally {
      isSearching = false;
    }
  }

  function clearSearch() {
    searchQuery = '';
    performSearch();
  }

  async function selectCustomer(customer: Customer) {
    selectedCustomerStore.set(customer);
    try {
      const summary = await getCustomerPointsSummary(customer.trcid);
      const history = await getPointsHistory(customer.trcid);
      customerSummaryStore.set(summary);
      customerHistoryStore.set(history);
      const fullName = `${customer.trcnom} ${customer.trcape}`.trim();
      addToast(`Cliente ${fullName} seleccionado`, 'success');
    } catch (err: any) {
      console.warn('Error al cargar datos del cliente:', err);
      addToast('Error al obtener los puntos del cliente.', 'error');
    }
  }

  onMount(() => {
    performSearch();
  });
</script>

<div class="card space-y-4">
  <div class="flex items-center justify-between">
    <h2 class="text-base font-bold text-slate-100 flex items-center gap-2">
      <Search size={18} class="text-emerald-400" />
      Buscador Instantáneo de Clientes POS
    </h2>
    <span class="text-xs text-slate-400 font-medium">Búsqueda directa sobre tabla `trc` de GsigmaPOS</span>
  </div>

  <!-- Input Field with Debounce -->
  <div class="relative">
    <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none text-slate-400">
      {#if isSearching}
        <Loader2 size={18} class="animate-spin text-emerald-400" />
      {:else}
        <Search size={18} />
      {/if}
    </div>
    <input 
      type="text"
      bind:value={searchQuery}
      on:input={handleInput}
      placeholder="Ingrese Cédula, NIT, Apellidos o Teléfono del cliente..."
      class="form-input pl-10 pr-10 py-2.5 text-sm rounded-lg bg-deep border-subtle focus:border-emerald-500 shadow-inner"
    />
    {#if searchQuery}
      <button 
        on:click={clearSearch}
        class="absolute inset-y-0 right-0 pr-3 flex items-center text-slate-400 hover:text-slate-200"
      >
        <X size={16} />
      </button>
    {/if}
  </div>

  <!-- High-Speed Dense Results Table -->
  <div class="dense-table-wrapper max-h-56 overflow-y-auto">
    <table class="dense-table">
      <thead>
        <tr>
          <th>Cédula / NIT</th>
          <th>Nombre Completo / Razón Social</th>
          <th>Teléfono</th>
          <th>Correo Electrónico</th>
          <th class="text-right">Acción</th>
        </tr>
      </thead>
      <tbody>
        {#if isSearching && searchResults.length === 0}
          <tr>
            <td colspan="5" class="text-center py-6 text-slate-400">
              <Loader2 size={20} class="animate-spin inline-block mr-2" /> Buscando clientes...
            </td>
          </tr>
        {:else if searchResults.length === 0}
          <tr>
            <td colspan="5" class="text-center py-6 text-slate-500 italic">
              No se encontraron clientes coincidentes.
            </td>
          </tr>
        {:else}
          {#each searchResults as customer}
            {@const isSelected = $selectedCustomerStore?.trcid === customer.trcid}
            {@const fullName = `${customer.trcnom} ${customer.trcape}`.trim()}
            <tr 
              class="cursor-pointer" 
              class:selected={isSelected}
              on:click={() => selectCustomer(customer)}
            >
              <td class="font-mono font-semibold text-emerald-400">{customer.trcnumdoc}</td>
              <td class="font-medium text-slate-200 flex items-center gap-2">
                <User size={14} class="text-slate-400" />
                {fullName}
              </td>
              <td class="text-slate-300">
                {#if customer.trctel1}
                  <span class="inline-flex items-center gap-1">
                    <Phone size={12} class="text-slate-500" />
                    {customer.trctel1}
                  </span>
                {:else}
                  <span class="text-slate-600">-</span>
                {/if}
              </td>
              <td class="text-slate-400 text-xs">
                {#if customer.trcema1}
                  <span class="inline-flex items-center gap-1">
                    <Mail size={12} class="text-slate-500" />
                    {customer.trcema1}
                  </span>
                {:else}
                  <span class="text-slate-600">-</span>
                {/if}
              </td>
              <td class="text-right">
                {#if isSelected}
                  <span class="btn btn-sm btn-primary py-0.5 px-2 text-xs pointer-events-none">
                    <CheckCircle2 size={12} /> Seleccionado
                  </span>
                {:else}
                  <button 
                    on:click|stopPropagation={() => selectCustomer(customer)}
                    class="btn btn-sm btn-secondary py-0.5 px-2 text-xs hover:border-emerald-500"
                  >
                    Seleccionar
                  </button>
                {/if}
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>
