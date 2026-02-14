<script lang="ts">
  import type { GenerateResult } from '../lib/types';
  import { formatMoney, formatPeriod, saveZipFile } from '../lib/utils';
  import { showToast } from '../lib/stores.svelte';

  interface Props {
    result: GenerateResult;
    onclose: () => void;
  }

  let { result, onclose }: Props = $props();

  async function handleSave() {
    try {
      const fileName = `SKLADKA_${result.generation.period_year}${String(result.generation.period_month).padStart(2, '0')}.zip`;
      const saved = await saveZipFile(result.zip_bytes, fileName);
      if (saved) {
        showToast('Plik ZIP zapisany pomyślnie', 'success');
      }
    } catch (e: any) {
      showToast(`Błąd zapisu: ${e}`, 'error');
    }
  }
</script>

<div class="fixed inset-0 z-40 bg-black/50 flex items-center justify-center" role="dialog">
  <div class="bg-white rounded-lg shadow-xl p-6 max-w-lg w-full mx-4">
    <h3 class="text-lg font-semibold text-gray-900 mb-4">
      Wygenerowano plik PPK
    </h3>

    <div class="bg-gray-50 rounded-lg p-4 mb-4 space-y-2">
      <div class="flex justify-between text-sm">
        <span class="text-gray-600">Okres:</span>
        <span class="font-medium">{formatPeriod(result.generation.period_year, result.generation.period_month)}</span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-gray-600">Liczba uczestników:</span>
        <span class="font-medium">{result.member_count}</span>
      </div>
      <hr class="border-gray-200" />
      <div class="flex justify-between text-sm">
        <span class="text-gray-600">Składki pracownika (podst.):</span>
        <span class="font-medium">{formatMoney(result.total_employee_basic)} zł</span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-gray-600">Składki pracownika (dod.):</span>
        <span class="font-medium">{formatMoney(result.total_employee_additional)} zł</span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-gray-600">Składki pracodawcy (podst.):</span>
        <span class="font-medium">{formatMoney(result.total_employer_basic)} zł</span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-gray-600">Składki pracodawcy (dod.):</span>
        <span class="font-medium">{formatMoney(result.total_employer_additional)} zł</span>
      </div>
    </div>

    <div class="flex justify-end gap-3">
      <button
        class="px-4 py-2 text-sm text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
        onclick={onclose}
      >
        Zamknij
      </button>
      <button
        class="px-4 py-2 text-sm text-white bg-blue-600 rounded-lg hover:bg-blue-700"
        onclick={handleSave}
      >
        Zapisz ZIP
      </button>
    </div>
  </div>
</div>
