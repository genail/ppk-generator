<script lang="ts">
  interface Props {
    saving: boolean;
    saved: boolean;
  }

  let { saving, saved }: Props = $props();
</script>

<div class="text-xs h-5">
  {#if saving}
    <span class="text-amber-600">Zapisywanie...</span>
  {:else if saved}
    <span class="text-green-600">Zapisano</span>
  {/if}
</div>
