<script lang="ts">
  import { onMount } from 'svelte';
  import { isDbConnected, loyaltyConfigStore } from '$lib/stores/appStore';
  import { getLoyaltyConfig } from '$lib/api';
  import { Award, Users, TrendingUp, DollarSign, Search, ArrowRight, ShieldCheck, Database } from 'lucide-svelte';

  onMount(async () => {
    try {
      const cfg = await getLoyaltyConfig();
      if (cfg) loyaltyConfigStore.set(cfg);
    } catch (e) {
      console.warn('Error al cargar configuración de fidelización:', e);
    }
  });

  function formatCurrency(val: number) {
    return new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(val);
  }
</script>

<div class="dashboard-container">
  <!-- Top Header Banner -->
  <header class="dashboard-header">
    <div>
      <h1 class="page-title">Panel General de Fidelización</h1>
      <p class="page-subtitle">Sistema POS GsigmaPOS / Itheke — Métricas y Resumen del Módulo de Puntos</p>
    </div>
    <a href="/clientes" class="btn btn-primary btn-lg">
      <Search size={18} />
      Buscar Cliente
      <ArrowRight size={16} />
    </a>
  </header>

  <!-- Metric Summary Cards -->
  <div class="metrics-grid">
    <div class="card metric-card">
      <div class="metric-icon icon-emerald">
        <Award size={24} />
      </div>
      <div class="metric-info">
        <span class="metric-label">Regla de Conversión</span>
        <span class="metric-value">{formatCurrency($loyaltyConfigStore.monto_por_punto)}</span>
        <span class="metric-hint">= 1 Punto de Fidelización</span>
      </div>
    </div>

    <div class="card metric-card">
      <div class="metric-icon icon-blue">
        <DollarSign size={24} />
      </div>
      <div class="metric-info">
        <span class="metric-label">Valor de Canje en Descuento</span>
        <span class="metric-value">{formatCurrency($loyaltyConfigStore.valor_punto_cop)}</span>
        <span class="metric-hint">por cada punto redimido</span>
      </div>
    </div>

    <div class="card metric-card">
      <div class="metric-icon icon-purple">
        <TrendingUp size={24} />
      </div>
      <div class="metric-info">
        <span class="metric-label">Compra Mínima Elegible</span>
        <span class="metric-value">{formatCurrency($loyaltyConfigStore.min_compra_puntos)}</span>
        <span class="metric-hint">Monto mínimo en factura</span>
      </div>
    </div>

    <div class="card metric-card">
      <div class="metric-icon icon-amber">
        <Database size={24} />
      </div>
      <div class="metric-info">
        <span class="metric-label">Estado de Conexión BD</span>
        <span class="metric-value {$isDbConnected ? 'text-emerald' : 'text-amber'}">
          {$isDbConnected ? 'MySQL 5.5 Activo' : 'Simulación (Sin BD)'}
        </span>
        <span class="metric-hint">Modo Lectura Directa POS</span>
      </div>
    </div>
  </div>

  <!-- Quick Access & Highlights -->
  <div class="dashboard-sections">
    <div class="card action-banner">
      <div class="banner-content">
        <h3>Buscador Instantáneo de Clientes</h3>
        <p>Busca a cualquier cliente por Cédula, NIT, Nombres o Teléfono para consultar sus facturas pasadas y procesar redenciones en tiempo real.</p>
        <div class="banner-actions mt-4">
          <a href="/clientes" class="btn btn-primary">
            <Users size={16} />
            Ir a Clientes & Puntos
          </a>
          <a href="/configuracion" class="btn btn-secondary">
            Ajustar Parámetros COP
          </a>
        </div>
      </div>
    </div>

    <!-- Security Protocol Card -->
    <div class="card security-card">
      <div class="security-header">
        <ShieldCheck size={20} class="text-emerald" />
        <h4>Protocolo de Seguridad Activo</h4>
      </div>
      <ul class="security-list">
        <li>
          <span class="check-dot">✓</span>
          <span><strong>Lectura Estricta (`SELECT ONLY`)</strong> en tablas de ventas (`venta`, `dvent`, `compra`).</span>
        </li>
        <li>
          <span class="check-dot">✓</span>
          <span><strong>Sin Triggers ni Alteraciones</strong> en el esquema nativo de GsigmaPOS.</span>
        </li>
        <li>
          <span class="check-dot">✓</span>
          <span><strong>Escrituras Aisladas</strong> únicamente en tablas auxiliares (`pv.puntos_*`).</span>
        </li>
        <li>
          <span class="check-dot">✓</span>
          <span><strong>Transacciones en Rust con Rollback Automático</strong> ante cualquier error.</span>
        </li>
      </ul>
    </div>
  </div>
</div>

<style>
  .dashboard-container {
    display: flex;
    flex-direction: column;
    gap: 28px;
  }

  .dashboard-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 20px;
    border-bottom: 1px solid #e2e8f0;
  }

  .page-title {
    font-size: 26px;
    font-weight: 800;
    color: #0f172a;
  }

  .page-subtitle {
    font-size: 13px;
    color: #64748b;
    margin-top: 4px;
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 20px;
  }

  @media (max-width: 1024px) {
    .metrics-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  .metric-card {
    display: flex;
    align-items: center;
    gap: 16px;
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.05);
  }

  .metric-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .icon-emerald { background: #ecfdf5; color: #059669; }
  .icon-blue { background: #eff6ff; color: #2563eb; }
  .icon-purple { background: #f3e8ff; color: #9333ea; }
  .icon-amber { background: #fffbeb; color: #d97706; }

  .metric-info {
    display: flex;
    flex-direction: column;
  }

  .metric-label {
    font-size: 11px;
    font-weight: 700;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .metric-value {
    font-family: var(--font-display);
    font-size: 20px;
    font-weight: 800;
    color: #0f172a;
    margin: 2px 0;
  }

  .text-emerald { color: #059669 !important; }
  .text-amber { color: #d97706 !important; }

  .metric-hint {
    font-size: 11px;
    color: #64748b;
  }

  .dashboard-sections {
    display: grid;
    grid-template-columns: 1.5fr 1fr;
    gap: 20px;
  }

  .action-banner {
    background: linear-gradient(135deg, #f0fdf4 0%, #e0f2fe 100%);
    border: 1px solid #bbf7d0;
    padding: 28px;
    border-radius: 14px;
  }

  .banner-content h3 {
    font-size: 20px;
    font-weight: 800;
    color: #0f172a;
    margin-bottom: 8px;
  }

  .banner-content p {
    color: #475569;
    font-size: 13.5px;
    line-height: 1.6;
  }

  .banner-actions {
    display: flex;
    gap: 12px;
    margin-top: 16px;
  }

  .security-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 14px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.05);
  }

  .security-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding-bottom: 12px;
    border-bottom: 1px solid #e2e8f0;
  }

  .security-header h4 {
    font-size: 15px;
    font-weight: 700;
    color: #0f172a;
  }

  .security-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .security-list li {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    font-size: 13px;
    color: #334155;
  }

  .check-dot {
    color: #059669;
    font-weight: bold;
  }

  @media (max-width: 900px) {
    .dashboard-sections {
      grid-template-columns: 1fr;
    }
  }
</style>
