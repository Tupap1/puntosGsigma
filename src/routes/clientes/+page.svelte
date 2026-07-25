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

<div class="clientes-view flex flex-col gap-6">
  <!-- Page Header -->
  <header class="flex items-center justify-between pb-4 border-b border-slate-800">
    <div>
      <h1 class="text-2xl font-extrabold text-slate-100">Buscador & Gestión de Puntos por Cliente</h1>
      <p class="text-xs text-slate-400 mt-1">Consulte clientes nativos del POS (`trc`), saldos acumulados e historial de movimientos.</p>
    </div>
  </header>

  <!-- 1. Buscador Instantáneo -->
  <section>
    <CustomerSearch />
  </section>

  <!-- 2. Tarjeta Financiera de Puntos -->
  <section>
    <PointsSummaryCard onOpenRedeemModal={handleOpenRedeemModal} />
  </section>

  <!-- 3. Tabla Densa de Historial -->
  <section>
    <HistoryTable />
  </section>

  <!-- Modal / Drawer Lateral para Redención -->
  <RedeemModal
    isOpen={isRedeemModalOpen}
    onClose={handleCloseRedeemModal}
    onSuccess={handleRedeemSuccess}
  />
</div>
