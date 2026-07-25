// IPC Wrapper for Tauri Rust Backend Commands

export interface DbConfig {
  host: string;
  port: number;
  user: string;
  pass: string;
  database: string;
}

export interface Customer {
  id: string;
  doc_num: string;
  name: string;
  phone: string;
  email: string;
}

export interface CustomerPoints {
  points_earned: number;
  points_redeemed: number;
  available_points: number;
  cop_value: number;
  valor_punto_cop: number;
}

export interface HistoryItem {
  id: string;
  date: string;
  type: 'acumulacion' | 'canje' | 'ajuste';
  invoice_ref: string;
  points: number;
  cop_value: number;
  note: string;
}

export interface PointsConfig {
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
      return { connected: true, message: 'Conexión exitosa a MySQL 5.5 (POS Gsigma)' } as T;
      
    case 'search_customers': {
      const query = (args?.query as string || '').toLowerCase();
      const mockCustomers: Customer[] = [
        { id: '1001', doc_num: '1098765432', name: 'JUAN CARLOS PÉREZ GÓMEZ', phone: '3109876543', email: 'juan.perez@email.com' },
        { id: '1002', doc_num: '900123456', name: 'COMERCIALIZADORA ALFA SAS', phone: '6017654321', email: 'contacto@alfa.com' },
        { id: '1003', doc_num: '63542109', name: 'MARÍA FERNANDA RODRÍGUEZ', phone: '3152345678', email: 'mafe.rodriguez@email.com' },
        { id: '1004', doc_num: '1015432987', name: 'CARLOS ALBERTO MARTÍNEZ', phone: '3201239876', email: 'cmartinez@email.com' }
      ];
      if (!query) return mockCustomers as T;
      return mockCustomers.filter(c => 
        c.doc_num.includes(query) || 
        c.name.toLowerCase().includes(query) || 
        c.phone.includes(query)
      ) as T;
    }
    
    case 'get_customer_points': {
      const customerId = args?.customerId as string;
      const valorPunto = 50; // 1 punto = 50 COP
      if (customerId === '1001') {
        return {
          points_earned: 4500,
          points_redeemed: 1200,
          available_points: 3300,
          cop_value: 3300 * valorPunto,
          valor_punto_cop: valorPunto
        } as T;
      }
      return {
        points_earned: 1500,
        points_redeemed: 0,
        available_points: 1500,
        cop_value: 1500 * valorPunto,
        valor_punto_cop: valorPunto
      } as T;
    }

    case 'redeem_points': {
      const points = args?.points as number || 0;
      return {
        success: true,
        message: `Se redimieron con éxito ${points} puntos.`,
        transaction_id: `RD-${Math.floor(Math.random() * 900000 + 100000)}`
      } as T;
    }

    case 'get_points_history': {
      const mockHistory: HistoryItem[] = [
        { id: 'h1', date: '2026-07-24 15:30', type: 'acumulacion', invoice_ref: 'FAC-9842', points: 450, cop_value: 22500, note: 'Venta POS $450.000 COP' },
        { id: 'h2', date: '2026-07-20 11:15', type: 'canje', invoice_ref: 'RD-541209', points: -1200, cop_value: -60000, note: 'Redención en Caja 01' },
        { id: 'h3', date: '2026-07-15 09:45', type: 'acumulacion', invoice_ref: 'FAC-9210', points: 4050, cop_value: 202500, note: 'Venta POS $4.050.000 COP' }
      ];
      return mockHistory as T;
    }

    case 'get_points_config': {
      return {
        monto_por_punto: 1000,
        valor_punto_cop: 50,
        min_compra_puntos: 10000,
        fecha_inicio_puntos: '2026-01-01'
      } as T;
    }

    case 'save_points_config': {
      return { success: true, message: 'Configuración de puntos guardada correctamente.' } as T;
    }

    default:
      throw new Error(`Comando no reconocido: ${command}`);
  }
}

// Public IPC API Functions
export const api = {
  checkDbConnection: (config?: DbConfig) => safeInvoke<{ connected: boolean; message: string }>('check_db_connection', { config }),
  searchCustomers: (query: string) => safeInvoke<Customer[]>('search_customers', { query }),
  getCustomerPoints: (customerId: string) => safeInvoke<CustomerPoints>('get_customer_points', { customerId }),
  redeemPoints: (customerId: string, points: number, invoiceRef: string, note?: string) => 
    safeInvoke<{ success: boolean; message: string; transaction_id: string }>('redeem_points', { customerId, points, invoiceRef, note }),
  getPointsHistory: (customerId: string) => safeInvoke<HistoryItem[]>('get_points_history', { customerId }),
  getPointsConfig: () => safeInvoke<PointsConfig>('get_points_config'),
  savePointsConfig: (config: PointsConfig) => safeInvoke<{ success: boolean; message: string }>('save_points_config', { config })
};
