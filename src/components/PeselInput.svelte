<script lang="ts">
  import { validatePesel } from '../lib/validation';

  interface Props {
    value: string;
    oninput: (pesel: string) => void;
    onvalidated?: (info: { dateOfBirth: string; gender: string } | null) => void;
  }

  let { value, oninput, onvalidated }: Props = $props();

  let error = $state('');
  let valid = $state(false);

  function handleInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const pesel = input.value.replace(/\D/g, '').slice(0, 11);
    oninput(pesel);

    if (pesel.length === 11) {
      const result = validatePesel(pesel);
      if (result.valid && result.info) {
        error = '';
        valid = true;
        onvalidated?.(result.info);
      } else {
        error = result.error || 'Nieprawidłowy PESEL';
        valid = false;
        onvalidated?.(null);
      }
    } else if (pesel.length > 0) {
      error = '';
      valid = false;
      onvalidated?.(null);
    } else {
      error = '';
      valid = false;
    }
  }
</script>

<div>
  <input
    type="text"
    maxlength="11"
    class="w-full px-3 py-2 border rounded-lg text-sm focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
    class:border-red-500={error}
    class:border-green-500={valid}
    class:border-gray-300={!error && !valid}
    value={value}
    oninput={handleInput}
    placeholder="00000000000"
  />
  {#if error}
    <p class="text-xs text-red-500 mt-1">{error}</p>
  {/if}
</div>
