<script lang="ts">
  import { getToast } from '../lib/stores.svelte';

  const toast = $derived(getToast());
</script>

{#if toast}
  <div
    class="fixed bottom-4 right-4 z-50 px-4 py-3 rounded-lg shadow-lg text-white text-sm max-w-md transition-all"
    class:bg-green-600={toast.type === 'success'}
    class:bg-red-600={toast.type === 'error'}
    class:bg-blue-600={toast.type === 'info'}
  >
    {toast.message}
  </div>
{/if}
