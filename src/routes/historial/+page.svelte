<script lang="ts">
  import HistoryTable from '$lib/components/HistoryTable.svelte';
  import { History } from 'lucide-svelte';
</script>

<div class="historial-view flex flex-col gap-6">
  <!-- Page Header -->
  <header class="flex items-center justify-between pb-4 border-b border-slate-800">
    <div>
      <div class="flex items-center gap-2">
        <History class="text-emerald-400" size={24} />
        <h1 class="text-2xl font-extrabold text-slate-100">Historial Global de Movimientos</h1>
      </div>
      <p class="text-xs text-slate-400 mt-1">Extracto de auditoría de todas las acumulaciones, canjes y ajustes registrados en la base de datos.</p>
    </div>
  </header>

  <!-- Tabla Densa con Filtros -->
  <section class="mt-2">
    <HistoryTable />
  </section>
</div>
