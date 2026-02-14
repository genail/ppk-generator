<script lang="ts">
  import type { Snippet } from 'svelte';
  import OrgSelector from './OrgSelector.svelte';
  import Toast from './Toast.svelte';
  import { getCurrentOrg, getCurrentView, setCurrentView } from '../lib/stores.svelte';

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  let orgSelector: ReturnType<typeof OrgSelector> | undefined = $state();

  const currentOrg = $derived(getCurrentOrg());
  const currentView = $derived(getCurrentView());

  interface NavItem {
    id: string;
    label: string;
    icon: string;
  }

  const navItems: NavItem[] = [
    { id: 'contributions', label: 'Składki', icon: '💰' },
    { id: 'members', label: 'Uczestnicy', icon: '👥' },
    { id: 'generations', label: 'Generacje', icon: '📄' },
    { id: 'org-form', label: 'Organizacja', icon: '🏢' },
  ];

  export function refreshOrgs() {
    orgSelector?.refresh();
  }
</script>

<div class="flex h-screen w-full">
  <!-- Sidebar -->
  <aside class="w-56 bg-white border-r border-gray-200 flex flex-col flex-shrink-0">
    <div class="px-4 py-3 border-b border-gray-200">
      <h1 class="text-base font-bold text-gray-800">PPK Generator</h1>
    </div>

    <OrgSelector bind:this={orgSelector} />

    <nav class="flex-1 py-2">
      {#each navItems as item}
        <button
          class="w-full text-left px-4 py-2 text-sm flex items-center gap-2 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          class:bg-blue-50={currentView === item.id}
          class:text-blue-700={currentView === item.id}
          class:text-gray-700={currentView !== item.id}
          class:hover:bg-gray-50={currentView !== item.id && !(!currentOrg && item.id !== 'org-form')}
          onclick={() => setCurrentView(item.id)}
          disabled={!currentOrg && item.id !== 'org-form'}
        >
          <span>{item.icon}</span>
          <span>{item.label}</span>
        </button>
      {/each}
    </nav>

    <div class="px-4 py-3 border-t border-gray-200 text-xs text-gray-400">
      v0.1.0
    </div>
  </aside>

  <!-- Main content -->
  <main class="flex-1 overflow-auto bg-gray-50">
    {@render children()}
  </main>
</div>

<Toast />
