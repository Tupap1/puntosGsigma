<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import CustomerSearch from '$lib/components/CustomerSearch.svelte';
  import PointsSummaryCard from '$lib/components/PointsSummaryCard.svelte';
  import HistoryTable from '$lib/components/HistoryTable.svelte';
  import RedeemModal from '$lib/components/RedeemModal.svelte';
  import DbConfigModal from '$lib/components/DbConfigModal.svelte';
</script>

<div class="flex flex-col min-h-screen">
  <!-- Top Navigation Bar -->
  <Navbar />

  <!-- Main Content Layout -->
  <main class="flex-1 p-6 max-w-7xl mx-auto w-full space-y-6">
    <!-- Customer Search Panel -->
    <section class="w-full">
      <CustomerSearch />
    </section>

    <!-- Points Financial Summary Card -->
    <section class="w-full">
      <PointsSummaryCard />
    </section>

    <!-- Movement Ledger History Table -->
    <section class="w-full">
      <HistoryTable />
    </section>
  </main>

  <!-- Modals and Slide-Over Drawers -->
  <RedeemModal />
  <DbConfigModal />
</div>
