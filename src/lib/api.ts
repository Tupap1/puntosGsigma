// IPC Wrapper for Tauri Rust Backend Commands

export interface DbConfig {
  host: string;
  port: number;
  user: string;
  password?: string;
  database: string;
}

export interface Customer {
  trcid: string;
  trcnumdoc: string;
  trcnom: string;
  trcape: string;
  trctel1: string;
  trcema1: string;
}

export interface PointSummary {
  trcid: string;
  puntos_acumulados_brutos: number;
  puntos_redimidos: number;
  saldo_disponible: number;
  total_ventas_cop: number;
  valor_equivalente_cop: number;
}

export interface PointTransaction {
  id?: number;
  trcid: string;
  tipo: string;
  puntos: number;
  monto_cop: number;
  concepto: string;
  referencia_doc?: string;
  fecha: string;
}

export interface LoyaltyConfig {
  monto_por_punto: number;
  valor_punto_cop: number;
  min_compra_puntos: number;
  fecha_inicio_puntos: string;
}

// Detect if running inside Tauri environment
export function isTauri(): boolean {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
}

// Safe invoke helper (calls Tauri Rust handlers directly)
async function safeInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<T>(command, args);
    } catch (err: any) {
      console.error(`[Tauri IPC Error] ${command}:`, err);
      throw err;
    }
  }
  
  // Clean Empty States when running in browser mode without Tauri engine (No Mock Fallback Data)
  console.log(`[Browser Env] Command called: ${command}`, args);
  return getEmptyStateData<T>(command, args);
}

// Empty state handler (Zero fake data)
function getEmptyStateData<T>(command: string, args?: Record<string, unknown>): T {
  switch (command) {
    case 'check_db_connection':
      return false as T;
      
    case 'search_customers':
      return [] as T;
    
    case 'get_customer_points_summary': {
      const trcid = (args?.trcid as string) || '';
      return {
        trcid,
        puntos_acumulados_brutos: 0,
        puntos_redimidos: 0,
        saldo_disponible: 0,
        total_ventas_cop: 0,
        valor_equivalente_cop: 0
      } as T;
    }

    case 'redeem_points':
      throw new Error('No se puede redimir puntos sin estar conectado a la base de datos MySQL local.');

    case 'get_points_history':
      return [] as T;

    case 'get_loyalty_config': {
      return {
        monto_por_punto: 1000,
        valor_punto_cop: 50,
        min_compra_puntos: 10000,
        fecha_inicio_puntos: '2000-01-01'
      } as T;
    }

    case 'save_loyalty_config':
    case 'save_db_config':
      return true as T;

    case 'get_db_config': {
      return {
        host: '127.0.0.1',
        port: 3306,
        user: 'root',
        password: '',
        database: 'pv'
      } as T;
    }

    default:
      return [] as T;
  }
}

// Public IPC API Functions matching Tauri Rust handlers exactly
export async function checkDbConnection(config?: DbConfig): Promise<boolean> {
  return safeInvoke<boolean>('check_db_connection', { config });
}

export async function searchCustomers(query: string): Promise<Customer[]> {
  if (!query || query.trim().length === 0) {
    return [];
  }
  return safeInvoke<Customer[]>('search_customers', { query });
}

export async function getCustomerPointsSummary(trcid: string): Promise<PointSummary> {
  return safeInvoke<PointSummary>('get_customer_points_summary', { trcid });
}

export async function redeemPoints(trcid: string, puntos: number, referenciaDoc?: string, concepto?: string): Promise<PointTransaction> {
  return safeInvoke<PointTransaction>('redeem_points', { trcid, puntos_a_redimir: puntos, referencia_doc: referenciaDoc, concepto });
}

export async function getPointsHistory(trcid?: string): Promise<PointTransaction[]> {
  return safeInvoke<PointTransaction[]>('get_points_history', { trcid });
}

export async function getLoyaltyConfig(): Promise<LoyaltyConfig> {
  return safeInvoke<LoyaltyConfig>('get_loyalty_config');
}

export async function saveLoyaltyConfig(config: LoyaltyConfig): Promise<boolean> {
  return safeInvoke<boolean>('save_loyalty_config', { config });
}

export async function saveDbConfig(config: DbConfig): Promise<boolean> {
  return safeInvoke<boolean>('save_db_config', { config });
}

export async function getDbConfig(): Promise<DbConfig> {
  return safeInvoke<DbConfig>('get_db_config');
}
