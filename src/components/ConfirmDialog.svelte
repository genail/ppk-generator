<script lang="ts">
  interface Props {
    open: boolean;
    title: string;
    message: string;
    onconfirm: () => void;
    oncancel: () => void;
  }

  let { open, title, message, onconfirm, oncancel }: Props = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-40 bg-black/50 flex items-center justify-center" role="dialog">
    <div class="bg-white rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
      <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
      <p class="text-gray-600 mb-6">{message}</p>
      <div class="flex justify-end gap-3">
        <button
          class="px-4 py-2 text-sm text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
          onclick={oncancel}
        >
          Anuluj
        </button>
        <button
          class="px-4 py-2 text-sm text-white bg-red-600 rounded-lg hover:bg-red-700"
          onclick={onconfirm}
        >
          Usuń
        </button>
      </div>
    </div>
  </div>
{/if}
