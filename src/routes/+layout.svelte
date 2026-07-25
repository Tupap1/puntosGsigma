<script lang="ts">
  import '$lib/styles/theme.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import DbConfigModal from '$lib/components/DbConfigModal.svelte';
  import ToastContainer from '$lib/components/ToastContainer.svelte';
  import { onMount } from 'svelte';
  import { checkDbConnection, getDbConfig } from '$lib/api';
  import { isDbConnected, dbConfigStore, addToast } from '$lib/stores/appStore';

  let showDbModal = false;

  onMount(async () => {
    try {
      const savedConfig = await getDbConfig();
      if (savedConfig) {
        dbConfigStore.set(savedConfig);
      }
      const isConnected = await checkDbConnection();
      isDbConnected.set(isConnected);
    } catch (err) {
      console.warn('Error al verificar conexión inicial:', err);
      isDbConnected.set(false);
    }
  });

  function handleOpenDbModal() {
    showDbModal = true;
  }

  function handleCloseDbModal() {
    showDbModal = false;
  }
</script>

<div class="app-layout">
  <!-- Left Navigation Sidebar -->
  <Sidebar onOpenDbModal={handleOpenDbModal} />

  <!-- Main Content View Area -->
  <div class="app-main-content">
    <main class="content-wrapper">
      <slot />
    </main>
  </div>

  <!-- Global Modals & Notifications -->
  <DbConfigModal isOpen={showDbModal} onClose={handleCloseDbModal} />
  <ToastContainer />
</div>

<style>
  .app-layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-app);
    color: var(--text-main);
  }

  .app-main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow-y: auto;
    background-color: var(--bg-app);
  }

  .content-wrapper {
    padding: 28px 32px;
    max-width: 1400px;
    width: 100%;
    margin: 0 auto;
  }
</style>
