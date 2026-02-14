<script lang="ts">
  import type { Organization } from '../lib/types';
  import { listOrganizations } from '../lib/api';
  import { getCurrentOrg, setCurrentOrg, setCurrentView } from '../lib/stores.svelte';

  let organizations = $state<Organization[]>([]);
  let loading = $state(true);

  const currentOrg = $derived(getCurrentOrg());

  export async function refresh() {
    loading = true;
    try {
      organizations = await listOrganizations();
      // If current org was deleted, reset
      if (currentOrg && !organizations.find(o => o.id === currentOrg.id)) {
        setCurrentOrg(organizations[0] || null);
      }
      // Auto-select first org if none selected
      if (!currentOrg && organizations.length > 0) {
        setCurrentOrg(organizations[0]);
      }
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    refresh();
  });

  function handleSelect(e: Event) {
    const select = e.target as HTMLSelectElement;
    const id = parseInt(select.value);
    if (id === -1) {
      // "New org" option
      setCurrentView('org-form');
      return;
    }
    const org = organizations.find(o => o.id === id);
    if (org) setCurrentOrg(org);
  }
</script>

<div class="px-3 py-3 border-b border-gray-200">
  <label class="block text-xs font-medium text-gray-500 mb-1">Organizacja</label>
  {#if loading}
    <div class="text-sm text-gray-400">Ładowanie...</div>
  {:else}
    <select
      class="w-full px-2 py-1.5 text-sm border border-gray-300 rounded-lg bg-white focus:ring-1 focus:ring-blue-500"
      value={currentOrg?.id ?? ''}
      onchange={handleSelect}
    >
      {#each organizations as org}
        <option value={org.id}>{org.name}</option>
      {/each}
      <option value={-1}>+ Nowa organizacja</option>
    </select>
  {/if}
</div>
