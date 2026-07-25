import { writable } from 'svelte/store';
import type { Customer, PointSummary, PointTransaction, DbConfig, LoyaltyConfig } from '$lib/api';

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
}

export const isDbConnected = writable<boolean>(false);
export const dbConnected = isDbConnected;

export const dbConfigStore = writable<DbConfig>({
  host: '127.0.0.1',
  port: 3306,
  user: 'root',
  password: '',
  database: 'pv'
});

export const loyaltyConfigStore = writable<LoyaltyConfig>({
  monto_por_punto: 1000.0,
  valor_punto_cop: 50.0,
  min_compra_puntos: 10000.0,
  fecha_inicio_puntos: '2000-01-01'
});

export const selectedCustomerStore = writable<Customer | null>(null);
export const selectedCustomer = selectedCustomerStore;

export const customerSummaryStore = writable<PointSummary | null>(null);
export const customerPoints = customerSummaryStore;

export const customerHistoryStore = writable<PointTransaction[]>([]);
export const pointsHistory = customerHistoryStore;

export const isRedeemModalOpen = writable<boolean>(false);
export const isDbConfigModalOpen = writable<boolean>(false);

// Toast Store
function createToastStore() {
  const { subscribe, update } = writable<ToastMessage[]>([]);

  return {
    subscribe,
    add: (message: string, type: 'success' | 'error' | 'info' = 'info') => {
      const id = Math.random().toString(36).substring(2, 9);
      update(toasts => [...toasts, { id, message, type }]);
      setTimeout(() => {
        update(toasts => toasts.filter(t => t.id !== id));
      }, 4000);
    },
    remove: (id: string) => {
      update(toasts => toasts.filter(t => t.id !== id));
    }
  };
}

export const toasts = createToastStore();

export function addToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
  toasts.add(message, type);
}
