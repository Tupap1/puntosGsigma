<script lang="ts">
  import { searchCustomers, getCustomerPointsSummary, getPointsHistory, type Customer } from '$lib/api';
  import { selectedCustomerStore, customerSummaryStore, customerHistoryStore, addToast } from '$lib/stores/appStore';
  import { Search, X, User, Phone, Mail, CheckCircle2, Loader2 } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let searchQuery = '';
  let searchResults: Customer[] = [];
  let isSearching = false;
  let hasSearched = false;
  let debounceTimer: any = null;

  function handleInput() {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      performSearch();
    }, 300);
  }

  async function performSearch() {
    if (!searchQuery || searchQuery.trim().length === 0) {
      searchResults = [];
      hasSearched = false;
      return;
    }

    isSearching = true;
    hasSearched = true;
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
    searchResults = [];
    hasSearched = false;
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
</script>

<div class="search-card">
  <div class="search-card-header">
    <h2 class="search-card-title">
      <Search size={18} color="#059669" />
      Buscador Instantáneo de Clientes POS
    </h2>
    <span class="search-card-hint">Consultando tabla `trc` de GsigmaPOS</span>
  </div>

  <!-- Input Field with Debounce -->
  <div class="search-input-wrapper">
    <span class="search-input-icon">
      {#if isSearching}
        <Loader2 size={18} class="animate-spin" color="#059669" />
      {:else}
        <Search size={18} color="#64748b" />
      {/if}
    </span>
    <input 
      type="text"
      bind:value={searchQuery}
      on:input={handleInput}
      placeholder="Ingrese Cédula, NIT, Apellidos o Teléfono del cliente para buscar..."
      class="search-input"
    />
    {#if searchQuery}
      <button 
        type="button"
        on:click={clearSearch}
        class="search-clear-btn"
      >
        <X size={16} color="#64748b" />
      </button>
    {/if}
  </div>

  <!-- High-Speed Dense Results Table / Empty State -->
  <div class="results-container">
    {#if isSearching}
      <div class="state-box">
        <Loader2 size={24} class="animate-spin" color="#059669" />
        <p class="state-text">Buscando en la tabla de clientes nativos...</p>
      </div>
    {:else if hasSearched && searchResults.length === 0}
      <div class="state-box">
        <User size={24} color="#94a3b8" />
        <p class="state-text">No se encontraron clientes que coincidan con <strong>"{searchQuery}"</strong> en la base de datos.</p>
      </div>
    {:else if !hasSearched}
      <div class="state-box quiet">
        <p class="state-text-quiet">Ingrese una Cédula, NIT o Nombre arriba para buscar un cliente en tiempo real.</p>
      </div>
    {:else}
      <div class="dense-table-wrapper max-h-56">
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
            {#each searchResults as customer}
              {@const isSelected = $selectedCustomerStore?.trcid === customer.trcid}
              {@const fullName = `${customer.trcnom} ${customer.trcape}`.trim()}
              <tr 
                class="row-item" 
                class:selected={isSelected}
                on:click={() => selectCustomer(customer)}
              >
                <td class="font-mono text-doc">{customer.trcnumdoc}</td>
                <td class="font-medium text-name">
                  <User size={14} color="#64748b" />
                  {fullName}
                </td>
                <td class="text-contact">
                  {#if customer.trctel1}
                    <span class="inline-flex">
                      <Phone size={12} color="#94a3b8" />
                      {customer.trctel1}
                    </span>
                  {:else}
                    <span class="text-dim">-</span>
                  {/if}
                </td>
                <td class="text-contact">
                  {#if customer.trcema1}
                    <span class="inline-flex">
                      <Mail size={12} color="#94a3b8" />
                      {customer.trcema1}
                    </span>
                  {:else}
                    <span class="text-dim">-</span>
                  {/if}
                </td>
                <td class="text-right">
                  {#if isSelected}
                    <span class="btn btn-sm btn-primary py-0.5 pointer-events-none">
                      <CheckCircle2 size={12} /> Seleccionado
                    </span>
                  {:else}
                    <button 
                      type="button"
                      on:click|stopPropagation={() => selectCustomer(customer)}
                      class="btn btn-sm btn-secondary"
                    >
                      Seleccionar
                    </button>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>

<style>
  .search-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 14px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.05);
  }

  .search-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .search-card-title {
    font-size: 16px;
    font-weight: 700;
    color: #0f172a;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .search-card-hint {
    font-size: 12px;
    color: #64748b;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .search-input-icon {
    position: absolute;
    left: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    z-index: 2;
  }

  .search-input {
    width: 100%;
    height: 44px;
    padding: 12px 40px 12px 42px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 10px;
    font-size: 14px;
    color: #0f172a;
    outline: none;
    transition: all 150ms ease;
  }

  .search-input:focus {
    border-color: #059669;
    background: #ffffff;
    box-shadow: 0 0 0 3px rgba(5, 150, 105, 0.15);
  }

  .search-clear-btn {
    position: absolute;
    right: 12px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .results-container {
    width: 100%;
  }

  .state-box {
    padding: 24px 16px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    background: #f8fafc;
    border: 1px dashed #cbd5e1;
    border-radius: 10px;
  }

  .state-box.quiet {
    padding: 16px;
    background: #ffffff;
    border-style: solid;
    border-color: #e2e8f0;
  }

  .state-text {
    font-size: 13px;
    color: #475569;
  }

  .state-text-quiet {
    font-size: 12.5px;
    color: #64748b;
    font-style: italic;
  }

  .max-h-56 {
    max-height: 230px;
    overflow-y: auto;
  }

  .row-item {
    cursor: pointer;
  }

  .text-doc {
    font-weight: 700;
    color: #059669;
  }

  .text-name {
    color: #0f172a;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .text-contact { color: #334155; font-size: 12px; }
  .text-dim { color: #94a3b8; }
  .text-right { text-align: right; }
  .inline-flex { display: inline-flex; align-items: center; gap: 4px; }
</style>
