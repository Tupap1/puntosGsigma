<script lang="ts">
  import CustomerSearch from '$lib/components/CustomerSearch.svelte';
  import PointsSummaryCard from '$lib/components/PointsSummaryCard.svelte';
  import HistoryTable from '$lib/components/HistoryTable.svelte';
  import RedeemModal from '$lib/components/RedeemModal.svelte';
  import { selectedCustomerStore, customerSummaryStore, customerHistoryStore, addToast } from '$lib/stores/appStore';
  import { getCustomerPointsSummary, getPointsHistory } from '$lib/api';

  let isRedeemModalOpen = false;

  $: if ($selectedCustomerStore) {
    loadCustomerData($selectedCustomerStore.trcid);
  }

  async function loadCustomerData(trcid: string) {
    try {
      const summary = await getCustomerPointsSummary(trcid);
      customerSummaryStore.set(summary);

      const history = await getPointsHistory(trcid);
      customerHistoryStore.set(history);
    } catch (err: any) {
      console.warn('Error al cargar datos de puntos del cliente:', err);
      addToast('Error al cargar saldo de puntos del cliente.', 'error');
    }
  }

  function handleOpenRedeemModal() {
    if (!$selectedCustomerStore || !$customerSummaryStore) {
      addToast('Por favor selecciona un cliente antes de procesar un canje.', 'info');
      return;
    }
    if ($customerSummaryStore.saldo_actual <= 0) {
      addToast('El cliente no dispone de saldo acumulado de puntos para redimir.', 'info');
      return;
    }
    isRedeemModalOpen = true;
  }

  function handleCloseRedeemModal() {
    isRedeemModalOpen = false;
  }

  function handleRedeemSuccess() {
    if ($selectedCustomerStore) {
      loadCustomerData($selectedCustomerStore.trcid);
    }
  }
</script>

<div class="page-container">
  <!-- Page Header -->
  <header class="page-header">
    <div>
      <h1 class="page-title">Gestión de Clientes & Puntos POS</h1>
      <p class="page-subtitle">Consulte la tabla de clientes (`trc`), saldos acumulados y ejecute redenciones con validación en tiempo real.</p>
    </div>
  </header>

  <!-- 1. Buscador Instantáneo -->
  <section class="section">
    <CustomerSearch />
  </section>

  <!-- 2. Tarjeta Financiera de Puntos -->
  <section class="section">
    <PointsSummaryCard onOpenRedeemModal={handleOpenRedeemModal} />
  </section>

  <!-- 3. Tabla Densa de Historial -->
  <section class="section">
    <HistoryTable />
  </section>

  <!-- Modal / Drawer Lateral para Redención -->
  <RedeemModal
    isOpen={isRedeemModalOpen}
    onClose={handleCloseRedeemModal}
    onSuccess={handleRedeemSuccess}
  />
</div>

<style>
  .page-container {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    padding-bottom: 16px;
    border-bottom: 1px solid #e2e8f0;
  }

  .page-title {
    font-size: 22px;
    font-weight: 800;
    color: #0f172a;
    letter-spacing: -0.02em;
  }

  .page-subtitle {
    font-size: 13px;
    color: #64748b;
    margin-top: 4px;
  }

  .section {
    width: 100%;
  }
</style>
