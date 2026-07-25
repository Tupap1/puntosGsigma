<script lang="ts">
  import '$lib/styles/theme.css';
  import ToastContainer from '$lib/components/ToastContainer.svelte';
</script>

<div class="min-h-screen bg-app text-slate-100 font-sans flex flex-col antialiased">
  <slot />
  <ToastContainer />
</div>
