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
  puntos_acumulados: number;
  puntos_redimidos: number;
  saldo_actual: number;
  valor_cop_disponible: number;
  valor_punto_cop: number;
}

export interface PointTransaction {
  id: number;
  trcid: string;
  tipo: 'acumulacion' | 'canje' | 'ajuste';
  puntos: number;
  monto_cop: number;
  concepto: string;
  referencia_doc: string;
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

// Safe invoke helper with fallback for web testing
async function safeInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(command, args);
  }
  
  // Simulated fallback data when testing in browser without Tauri engine
  console.log(`[Browser Mock IPC] Calling command: ${command}`, args);
  return getMockData<T>(command, args);
}

// Mock fallback logic
function getMockData<T>(command: string, args?: Record<string, unknown>): T {
  switch (command) {
    case 'check_db_connection':
      return true as T;
      
    case 'search_customers': {
      const query = (args?.query as string || '').toLowerCase();
      const mockCustomers: Customer[] = [
        { trcid: '1001', trcnumdoc: '1098765432', trcnom: 'JUAN CARLOS', trcape: 'PÉREZ GÓMEZ', trctel1: '3109876543', trcema1: 'juan.perez@email.com' },
        { trcid: '1002', trcnumdoc: '900123456', trcnom: 'COMERCIALIZADORA ALFA SAS', trcape: '', trctel1: '6017654321', trcema1: 'contacto@alfa.com' },
        { trcid: '1003', trcnumdoc: '63542109', trcnom: 'MARÍA FERNANDA', trcape: 'RODRÍGUEZ', trctel1: '3152345678', trcema1: 'mafe.rodriguez@email.com' },
        { trcid: '1004', trcnumdoc: '1015432987', trcnom: 'CARLOS ALBERTO', trcape: 'MARTÍNEZ', trctel1: '3201239876', trcema1: 'cmartinez@email.com' }
      ];
      if (!query) return mockCustomers as T;
      return mockCustomers.filter(c => 
        c.trcnumdoc.includes(query) || 
        c.trcnom.toLowerCase().includes(query) || 
        c.trcape.toLowerCase().includes(query) ||
        c.trctel1.includes(query)
      ) as T;
    }
    
    case 'get_customer_points_summary': {
      const trcid = args?.trcid as string;
      const valorPunto = 50; // 1 punto = 50 COP
      if (trcid === '1001') {
        return {
          trcid,
          puntos_acumulados: 4500,
          puntos_redimidos: 1200,
          saldo_actual: 3300,
          valor_cop_disponible: 3300 * valorPunto,
          valor_punto_cop: valorPunto
        } as T;
      }
      return {
        trcid,
        puntos_acumulados: 1500,
        puntos_redimidos: 0,
        saldo_actual: 1500,
        valor_cop_disponible: 1500 * valorPunto,
        valor_punto_cop: valorPunto
      } as T;
    }

    case 'redeem_points': {
      const puntos = args?.puntos as number || 0;
      return {
        id: Math.floor(Math.random() * 900000 + 100000),
        trcid: args?.trcid as string,
        tipo: 'canje',
        puntos: -puntos,
        monto_cop: -puntos * 50,
        concepto: (args?.concepto as string) || 'Redención de Puntos',
        referencia_doc: (args?.referencia_doc as string) || `RD-${Math.floor(Math.random() * 90000)}`,
        fecha: new Date().toISOString()
      } as T;
    }

    case 'get_points_history': {
      const mockHistory: PointTransaction[] = [
        { id: 1, trcid: '1001', fecha: '2026-07-24 15:30', tipo: 'acumulacion', referencia_doc: 'FAC-9842', puntos: 450, monto_cop: 22500, concepto: 'Venta POS $450.000 COP' },
        { id: 2, trcid: '1001', fecha: '2026-07-20 11:15', tipo: 'canje', referencia_doc: 'RD-541209', puntos: -1200, monto_cop: -60000, concepto: 'Redención en Caja 01' },
        { id: 3, trcid: '1001', fecha: '2026-07-15 09:45', tipo: 'acumulacion', referencia_doc: 'FAC-9210', puntos: 4050, monto_cop: 202500, concepto: 'Venta POS $4.050.000 COP' }
      ];
      return mockHistory as T;
    }

    case 'get_loyalty_config': {
      return {
        monto_por_punto: 1000,
        valor_punto_cop: 50,
        min_compra_puntos: 10000,
        fecha_inicio_puntos: '2026-01-01'
      } as T;
    }

    case 'save_loyalty_config': {
      return true as T;
    }

    case 'save_db_config': {
      return true as T;
    }

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
      throw new Error(`Comando no reconocido: ${command}`);
  }
}

// Public IPC API Functions matching Tauri Rust handlers exactly
export async function checkDbConnection(): Promise<boolean> {
  return safeInvoke<boolean>('check_db_connection');
}

export async function searchCustomers(query: string): Promise<Customer[]> {
  return safeInvoke<Customer[]>('search_customers', { query });
}

export async function getCustomerPointsSummary(trcid: string): Promise<PointSummary> {
  return safeInvoke<PointSummary>('get_customer_points_summary', { trcid });
}

export async function redeemPoints(trcid: string, puntos: number, referenciaDoc?: string, concepto?: string): Promise<PointTransaction> {
  return safeInvoke<PointTransaction>('redeem_points', { trcid, puntos, referencia_doc: referenciaDoc, concepto });
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
