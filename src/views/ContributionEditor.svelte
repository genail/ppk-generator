<script lang="ts">
  import type { ContributionWithMember, GenerateResult } from '../lib/types';
  import { listContributions, upsertContribution, prefillContributions, generatePpk } from '../lib/api';
  import { getCurrentOrg, showToast } from '../lib/stores.svelte';
  import { POLISH_MONTHS, formatMoney, sumMoney, currentPeriod } from '../lib/utils';
  import MoneyInput from '../components/MoneyInput.svelte';
  import SaveIndicator from '../components/SaveIndicator.svelte';
  import GenerationSummary from '../components/GenerationSummary.svelte';

  const currentOrg = $derived(getCurrentOrg());

  let selectedYear = $state(currentPeriod().year);
  let selectedMonth = $state(currentPeriod().month);
  let contributions = $state<ContributionWithMember[]>([]);
  let loading = $state(false);
  let saving = $state(false);
  let saved = $state(false);
  let generating = $state(false);
  let generateResult = $state<GenerateResult | null>(null);

  let saveTimeout: ReturnType<typeof setTimeout> | null = null;
  let savedTimeout: ReturnType<typeof setTimeout> | null = null;

  async function loadContributions() {
    if (!currentOrg) return;
    loading = true;
    try {
      contributions = await listContributions(currentOrg.id, selectedYear, selectedMonth);
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    if (currentOrg) {
      loadContributions();
    }
  });

  // Reload when period changes
  $effect(() => {
    // Access the reactive values to create the dependency
    const _y = selectedYear;
    const _m = selectedMonth;
    if (currentOrg) {
      loadContributions();
    }
  });

  let pendingMemberIds = new Set<number>();

  function getContribData(memberId: number) {
    const contrib = contributions.find(c => c.member_id === memberId);
    if (!contrib) return null;
    return {
      member_id: memberId,
      period_year: selectedYear,
      period_month: selectedMonth,
      employee_basic: contrib.employee_basic,
      employee_additional: contrib.employee_additional,
      employer_basic: contrib.employer_basic,
      employer_additional: contrib.employer_additional,
      reduced_basic_flag: contrib.reduced_basic_flag,
    };
  }

  async function flushPendingSave() {
    if (saveTimeout) {
      clearTimeout(saveTimeout);
      saveTimeout = null;
    }
    const memberIds = [...pendingMemberIds];
    pendingMemberIds.clear();
    for (const memberId of memberIds) {
      const data = getContribData(memberId);
      if (data) {
        await doUpsert(data);
      }
    }
  }

  async function doUpsert(data: ReturnType<typeof getContribData> & {}) {
    try {
      await upsertContribution(data);
      saving = false;
      saved = true;
      if (savedTimeout) clearTimeout(savedTimeout);
      savedTimeout = setTimeout(() => { saved = false; }, 2000);
    } catch (e: any) {
      saving = false;
      showToast(`Błąd zapisu: ${e}`, 'error');
    }
  }

  function scheduleSave(memberId: number) {
    if (saveTimeout) clearTimeout(saveTimeout);
    if (savedTimeout) clearTimeout(savedTimeout);

    saving = true;
    saved = false;
    pendingMemberIds.add(memberId);

    saveTimeout = setTimeout(async () => {
      const memberIds = [...pendingMemberIds];
      pendingMemberIds.clear();
      for (const id of memberIds) {
        const data = getContribData(id);
        if (data) {
          await doUpsert(data);
        }
      }
    }, 500);
  }

  function handleFieldChange(contrib: ContributionWithMember, field: string, value: string) {
    // Update local state immediately
    const idx = contributions.findIndex(c => c.id === contrib.id && c.member_id === contrib.member_id);
    if (idx >= 0) {
      (contributions[idx] as any)[field] = value;
    }
    scheduleSave(contrib.member_id);
  }

  async function handlePrefill() {
    if (!currentOrg) return;
    try {
      const count = await prefillContributions(currentOrg.id, selectedYear, selectedMonth);
      if (count > 0) {
        showToast(`Przeniesiono składki dla ${count} uczestników`, 'success');
        await loadContributions();
      } else {
        showToast('Brak składek do przeniesienia (brak historii lub składki już istnieją)', 'info');
      }
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    }
  }

  async function handleGenerate() {
    if (!currentOrg) return;
    generating = true;
    try {
      await flushPendingSave();
      generateResult = await generatePpk(currentOrg.id, selectedYear, selectedMonth);
    } catch (e: any) {
      showToast(`Błąd generowania: ${e}`, 'error');
    } finally {
      generating = false;
    }
  }

  // Generate year options: current year and a few years back
  const yearOptions = $derived(() => {
    const now = new Date().getFullYear();
    const years: number[] = [];
    for (let y = now; y >= now - 3; y--) years.push(y);
    return years;
  });

  const totals = $derived({
    eb: sumMoney(contributions.map(c => c.employee_basic)),
    ea: sumMoney(contributions.map(c => c.employee_additional)),
    rb: sumMoney(contributions.map(c => c.employer_basic)),
    ra: sumMoney(contributions.map(c => c.employer_additional)),
    get all() { return sumMoney([this.eb, this.ea, this.rb, this.ra]); },
  });
</script>

<div class="p-6">
  <div class="flex items-center justify-between mb-4">
    <h2 class="text-xl font-semibold text-gray-900">Składki PPK</h2>
    <SaveIndicator {saving} {saved} />
  </div>

  <!-- Period selector -->
  <div class="flex items-center gap-3 mb-4">
    <div class="flex items-center gap-2">
      <label class="text-sm text-gray-600">Okres:</label>
      <select
        class="px-3 py-1.5 text-sm border border-gray-300 rounded-lg bg-white"
        bind:value={selectedMonth}
      >
        {#each Array.from({length: 12}, (_, i) => i + 1) as m}
          <option value={m}>{POLISH_MONTHS[m]}</option>
        {/each}
      </select>
      <select
        class="px-3 py-1.5 text-sm border border-gray-300 rounded-lg bg-white"
        bind:value={selectedYear}
      >
        {#each yearOptions() as y}
          <option value={y}>{y}</option>
        {/each}
      </select>
    </div>

    <div class="flex gap-2 ml-auto">
      <button
        class="px-3 py-1.5 text-sm text-blue-600 bg-blue-50 rounded-lg hover:bg-blue-100"
        onclick={handlePrefill}
      >
        Przenies z poprzedniego
      </button>
      <button
        class="px-3 py-1.5 text-sm text-white bg-green-600 rounded-lg hover:bg-green-700 disabled:opacity-50"
        onclick={handleGenerate}
        disabled={generating || contributions.length === 0}
      >
        {generating ? 'Generowanie...' : 'Generuj plik PPK'}
      </button>
    </div>
  </div>

  {#if loading}
    <p class="text-sm text-gray-500">Ładowanie...</p>
  {:else if contributions.length === 0}
    <div class="text-center py-12 text-gray-500">
      <p class="text-lg mb-2">Brak składek dla wybranego okresu</p>
      <p class="text-sm">Użyj "Przenies z poprzedniego" aby skopiować składki z ostatniego okresu, lub dodaj uczestników w zakładce "Uczestnicy"</p>
    </div>
  {:else}
    <div class="bg-white rounded-lg border border-gray-200 overflow-x-auto">
      <table class="w-full text-sm">
        <thead class="bg-gray-50 border-b border-gray-200">
          <tr>
            <th class="text-left px-3 py-2 font-medium text-gray-600 whitespace-nowrap">Uczestnik</th>
            <th class="text-right px-3 py-2 font-medium text-gray-600 whitespace-nowrap">Prac. podst.</th>
            <th class="text-right px-3 py-2 font-medium text-gray-600 whitespace-nowrap">Prac. dod.</th>
            <th class="text-right px-3 py-2 font-medium text-gray-600 whitespace-nowrap">Firma podst.</th>
            <th class="text-right px-3 py-2 font-medium text-gray-600 whitespace-nowrap">Firma dod.</th>
            <th class="text-center px-3 py-2 font-medium text-gray-600 whitespace-nowrap">Obniż.</th>
          </tr>
        </thead>
        <tbody>
          {#each contributions as contrib}
            <tr class="border-b border-gray-100">
              <td class="px-3 py-2 whitespace-nowrap">
                <div class="font-medium">{contrib.last_name} {contrib.first_name}</div>
                <div class="text-xs text-gray-400 font-mono">{contrib.pesel}</div>
              </td>
              <td class="px-3 py-2 text-right">
                <MoneyInput
                  value={contrib.employee_basic}
                  onchange={(v) => handleFieldChange(contrib, 'employee_basic', v)}
                />
              </td>
              <td class="px-3 py-2 text-right">
                <MoneyInput
                  value={contrib.employee_additional}
                  onchange={(v) => handleFieldChange(contrib, 'employee_additional', v)}
                />
              </td>
              <td class="px-3 py-2 text-right">
                <MoneyInput
                  value={contrib.employer_basic}
                  onchange={(v) => handleFieldChange(contrib, 'employer_basic', v)}
                />
              </td>
              <td class="px-3 py-2 text-right">
                <MoneyInput
                  value={contrib.employer_additional}
                  onchange={(v) => handleFieldChange(contrib, 'employer_additional', v)}
                />
              </td>
              <td class="px-3 py-2 text-center">
                <select
                  class="px-2 py-1 text-sm border border-gray-300 rounded bg-white"
                  value={contrib.reduced_basic_flag}
                  onchange={(e) => {
                    const val = (e.target as HTMLSelectElement).value;
                    handleFieldChange(contrib, 'reduced_basic_flag', val);
                  }}
                >
                  <option value="N">N</option>
                  <option value="T">T</option>
                </select>
              </td>
            </tr>
          {/each}
        </tbody>
        <tfoot class="bg-gray-50 border-t border-gray-200">
          <tr>
            <td class="px-3 py-2 font-medium text-gray-700">Razem</td>
            <td class="px-3 py-2 text-right font-medium">
              {formatMoney(totals.eb)}
            </td>
            <td class="px-3 py-2 text-right font-medium">
              {formatMoney(totals.ea)}
            </td>
            <td class="px-3 py-2 text-right font-medium">
              {formatMoney(totals.rb)}
            </td>
            <td class="px-3 py-2 text-right font-medium">
              {formatMoney(totals.ra)}
            </td>
            <td class="px-3 py-2 text-right font-semibold text-gray-900">
              {formatMoney(totals.all)} zł
            </td>
          </tr>
        </tfoot>
      </table>
    </div>
  {/if}
</div>

{#if generateResult}
  <GenerationSummary
    result={generateResult}
    onclose={() => generateResult = null}
  />
{/if}
