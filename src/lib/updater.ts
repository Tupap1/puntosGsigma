// Flujo de auto-actualización (Tauri v2 + tauri-plugin-updater / tauri-plugin-process).
//
// - En arranque: verificación silenciosa; si hay versión nueva, solo notifica.
// - Manual (botón): si hay versión nueva, descarga + instala + reinicia (el clic
//   del usuario es el consentimiento). Si no hay, avisa que ya está actualizada.
//
// Fuera de Tauri (navegador de desarrollo) no hace nada destructivo.

import { addToast } from '$lib/stores/appStore';
import { isTauri } from '$lib/api';

let updateInProgress = false;

/**
 * Verifica si hay una actualización disponible.
 * @param silent  true = solo notifica si encuentra una (arranque);
 *                false = flujo interactivo completo (botón manual).
 */
export async function checkForUpdates(silent = false): Promise<void> {
  if (!isTauri()) {
    if (!silent) {
      addToast('Las actualizaciones solo están disponibles en la aplicación instalada.', 'info');
    }
    return;
  }

  if (updateInProgress) return;

  try {
    const { check } = await import('@tauri-apps/plugin-updater');
    const update = await check();

    if (!update) {
      if (!silent) addToast('La aplicación ya está en la última versión.', 'success');
      return;
    }

    // Hay actualización disponible.
    if (silent) {
      addToast(
        `Actualización v${update.version} disponible. Instálala desde el panel lateral.`,
        'info'
      );
      return;
    }

    // Flujo manual: el usuario pidió actualizar -> descargar, instalar y reiniciar.
    updateInProgress = true;
    addToast(`Descargando actualización v${update.version}...`, 'info');

    await update.downloadAndInstall();

    addToast('Actualización instalada. Reiniciando la aplicación...', 'success');

    const { relaunch } = await import('@tauri-apps/plugin-process');
    await relaunch();
  } catch (err: any) {
    console.error('[updater] Error al buscar/instalar actualización:', err);
    if (!silent) {
      addToast('Error al actualizar: ' + (err?.message || String(err)), 'error');
    }
  } finally {
    updateInProgress = false;
  }
}
