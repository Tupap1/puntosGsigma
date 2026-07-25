<script lang="ts">
  import HistoryTable from '$lib/components/HistoryTable.svelte';
  import { History } from 'lucide-svelte';
</script>

<div class="historial-page">
  <header class="page-header">
    <div>
      <h1 class="page-title">Historial & Auditoría Global de Puntos</h1>
      <p class="page-subtitle">Registro cronológico completo de todas las operaciones de acumulaciones y redenciones de puntos.</p>
    </div>
  </header>

  <section class="section">
    <HistoryTable />
  </section>
</div>

<style>
  .historial-page {
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
