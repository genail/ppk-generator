<script lang="ts">
  import { commaToDot, dotToComma } from '../lib/utils';

  interface Props {
    value: string;
    onchange: (dotValue: string) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  let displayValue = $state(dotToComma(value));
  let error = $state('');

  $effect(() => {
    displayValue = dotToComma(value);
  });

  function handleInput(e: Event) {
    const input = e.target as HTMLInputElement;
    displayValue = input.value;
  }

  function handleBlur() {
    // Normalize: allow comma or dot input
    let normalized = displayValue.replace(',', '.');
    // Remove trailing dots, leading zeros etc
    const num = parseFloat(normalized);
    if (isNaN(num) || num < 0) {
      error = 'Nieprawidłowa kwota';
      return;
    }
    error = '';
    const dotValue = num.toFixed(2);
    displayValue = dotToComma(dotValue);
    onchange(dotValue);
  }
</script>

<input
  type="text"
  class="w-24 px-2 py-1 text-right text-sm border rounded focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
  class:border-red-500={error}
  class:border-gray-300={!error}
  {disabled}
  value={displayValue}
  oninput={handleInput}
  onblur={handleBlur}
/>
