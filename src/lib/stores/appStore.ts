import { writable } from 'svelte/store';
import type { Customer, CustomerPoints, HistoryItem } from '../api';

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
}

export const dbConnected = writable<boolean>(false);
export const isDbTesting = writable<boolean>(false);
export const selectedCustomer = writable<Customer | null>(null);
export const customerPoints = writable<CustomerPoints | null>(null);
export const pointsHistory = writable<HistoryItem[]>([]);

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
