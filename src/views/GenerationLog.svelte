<script lang="ts">
  import type { Generation, GenerateResult } from '../lib/types';
  import { listGenerations, exportGeneration } from '../lib/api';
  import { getCurrentOrg, showToast } from '../lib/stores.svelte';
  import { formatPeriod, saveZipFile } from '../lib/utils';

  const currentOrg = $derived(getCurrentOrg());
  let generations = $state<Generation[]>([]);
  let loading = $state(true);

  async function loadGenerations() {
    if (!currentOrg) return;
    loading = true;
    try {
      generations = await listGenerations(currentOrg.id);
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    if (currentOrg) loadGenerations();
  });

  async function handleExport(gen: Generation) {
    try {
      const result = await exportGeneration(gen.id);
      const fileName = `SKLADKA_${gen.period_year}${String(gen.period_month).padStart(2, '0')}.zip`;
      const saved = await saveZipFile(result.zip_bytes, fileName);
      if (saved) {
        showToast('Plik ZIP zapisany', 'success');
      }
    } catch (e: any) {
      showToast(`Błąd eksportu: ${e}`, 'error');
    }
  }

  function formatDate(dt: string): string {
    return dt.replace('T', ' ').slice(0, 19);
  }
</script>

<div class="p-6">
  <h2 class="text-xl font-semibold text-gray-900 mb-4">Historia generacji</h2>

  {#if loading}
    <p class="text-sm text-gray-500">Ładowanie...</p>
  {:else if generations.length === 0}
    <div class="text-center py-12 text-gray-500">
      <p class="text-lg mb-2">Brak generacji</p>
      <p class="text-sm">Wygeneruj plik PPK w zakładce "Składki"</p>
    </div>
  {:else}
    <div class="bg-white rounded-lg border border-gray-200 overflow-hidden">
      <table class="w-full text-sm">
        <thead class="bg-gray-50 border-b border-gray-200">
          <tr>
            <th class="text-left px-4 py-3 font-medium text-gray-600">Okres</th>
            <th class="text-left px-4 py-3 font-medium text-gray-600">Data generacji</th>
            <th class="text-right px-4 py-3 font-medium text-gray-600">Uczestnicy</th>
            <th class="text-right px-4 py-3 font-medium text-gray-600">Składki prac.</th>
            <th class="text-right px-4 py-3 font-medium text-gray-600">Składki pracodawcy</th>
            <th class="text-right px-4 py-3 font-medium text-gray-600">Akcje</th>
          </tr>
        </thead>
        <tbody>
          {#each generations as gen}
            <tr class="border-b border-gray-100 hover:bg-gray-50">
              <td class="px-4 py-3">{formatPeriod(gen.period_year, gen.period_month)}</td>
              <td class="px-4 py-3 text-gray-600">{formatDate(gen.generated_at)}</td>
              <td class="px-4 py-3 text-right">{gen.member_count}</td>
              <td class="px-4 py-3 text-right">{gen.total_employee_basic.replace('.', ',')} zł</td>
              <td class="px-4 py-3 text-right">{gen.total_employer_basic.replace('.', ',')} zł</td>
              <td class="px-4 py-3 text-right">
                <button
                  class="text-blue-600 hover:text-blue-700 text-xs"
                  onclick={() => handleExport(gen)}
                >
                  Pobierz ZIP
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
