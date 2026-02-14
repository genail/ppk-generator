<script lang="ts">
  import type { Member } from '../lib/types';
  import { listMembers, deleteMember } from '../lib/api';
  import { getCurrentOrg, showToast } from '../lib/stores.svelte';
  import ConfirmDialog from '../components/ConfirmDialog.svelte';

  interface Props {
    onedit: (member: Member) => void;
    onnew: () => void;
  }

  let { onedit, onnew }: Props = $props();

  const currentOrg = $derived(getCurrentOrg());
  let members = $state<Member[]>([]);
  let loading = $state(true);
  let deleteTarget = $state<Member | null>(null);

  async function loadMembers() {
    if (!currentOrg) return;
    loading = true;
    try {
      members = await listMembers(currentOrg.id);
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    if (currentOrg) loadMembers();
  });

  async function handleDelete() {
    if (!deleteTarget) return;
    try {
      await deleteMember(deleteTarget.id);
      showToast('Uczestnik usunięty', 'success');
      await loadMembers();
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      deleteTarget = null;
    }
  }

  function statusLabel(status: string): string {
    switch (status) {
      case 'active': return 'Aktywny';
      case 'resigned': return 'Rezygnacja';
      case 'terminated': return 'Zwolniony';
      default: return status;
    }
  }

  function statusColor(status: string): string {
    switch (status) {
      case 'active': return 'bg-green-100 text-green-700';
      case 'resigned': return 'bg-yellow-100 text-yellow-700';
      case 'terminated': return 'bg-red-100 text-red-700';
      default: return 'bg-gray-100 text-gray-700';
    }
  }

  export { loadMembers as refresh };
</script>

<div class="p-6">
  <div class="flex items-center justify-between mb-4">
    <h2 class="text-xl font-semibold text-gray-900">Uczestnicy PPK</h2>
    <button
      class="px-4 py-2 text-sm text-white bg-blue-600 rounded-lg hover:bg-blue-700"
      onclick={onnew}
    >
      + Dodaj uczestnika
    </button>
  </div>

  {#if loading}
    <p class="text-sm text-gray-500">Ładowanie...</p>
  {:else if members.length === 0}
    <div class="text-center py-12 text-gray-500">
      <p class="text-lg mb-2">Brak uczestników</p>
      <p class="text-sm">Dodaj pierwszego uczestnika PPK</p>
    </div>
  {:else}
    <div class="bg-white rounded-lg border border-gray-200 overflow-hidden">
      <table class="w-full text-sm">
        <thead class="bg-gray-50 border-b border-gray-200">
          <tr>
            <th class="text-left px-4 py-3 font-medium text-gray-600">Nazwisko i imię</th>
            <th class="text-left px-4 py-3 font-medium text-gray-600">PESEL</th>
            <th class="text-left px-4 py-3 font-medium text-gray-600">Data ur.</th>
            <th class="text-left px-4 py-3 font-medium text-gray-600">Status</th>
            <th class="text-right px-4 py-3 font-medium text-gray-600">Akcje</th>
          </tr>
        </thead>
        <tbody>
          {#each members as member}
            <tr class="border-b border-gray-100 hover:bg-gray-50">
              <td class="px-4 py-3">{member.last_name} {member.first_name}</td>
              <td class="px-4 py-3 font-mono">{member.pesel}</td>
              <td class="px-4 py-3">{member.date_of_birth}</td>
              <td class="px-4 py-3">
                <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium {statusColor(member.status)}">
                  {statusLabel(member.status)}
                </span>
              </td>
              <td class="px-4 py-3 text-right">
                <button
                  class="text-blue-600 hover:text-blue-700 mr-3 text-xs"
                  onclick={() => onedit(member)}
                >Edytuj</button>
                <button
                  class="text-red-600 hover:text-red-700 text-xs"
                  onclick={() => deleteTarget = member}
                >Usuń</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<ConfirmDialog
  open={deleteTarget !== null}
  title="Usuń uczestnika"
  message={deleteTarget ? `Czy na pewno chcesz usunąć ${deleteTarget.first_name} ${deleteTarget.last_name}?` : ''}
  onconfirm={handleDelete}
  oncancel={() => deleteTarget = null}
/>
