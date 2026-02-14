<script lang="ts">
  import type { Member } from './lib/types';
  import Layout from './components/Layout.svelte';
  import { getCurrentOrg, getCurrentView, setCurrentView } from './lib/stores.svelte';
  import ContributionEditor from './views/ContributionEditor.svelte';
  import MemberList from './views/MemberList.svelte';
  import MemberForm from './views/MemberForm.svelte';
  import GenerationLog from './views/GenerationLog.svelte';
  import OrganizationForm from './views/OrganizationForm.svelte';

  let layout: ReturnType<typeof Layout> | undefined = $state();
  let memberList: ReturnType<typeof MemberList> | undefined = $state();

  const currentOrg = $derived(getCurrentOrg());
  const currentView = $derived(getCurrentView());

  // Member form state
  let editingMember = $state<Member | null>(null);
  let showMemberForm = $state(false);

  function handleEditMember(member: Member) {
    editingMember = member;
    showMemberForm = true;
  }

  function handleNewMember() {
    editingMember = null;
    showMemberForm = true;
  }

  function handleMemberSaved() {
    showMemberForm = false;
    editingMember = null;
    memberList?.refresh();
  }

  function handleOrgChanged() {
    layout?.refreshOrgs();
  }
</script>

<Layout bind:this={layout}>
  {#if currentView === 'contributions'}
    {#if currentOrg}
      <ContributionEditor />
    {:else}
      <div class="p-6 text-gray-500 text-center pt-20">
        <p class="text-lg">Wybierz lub utwórz organizację</p>
      </div>
    {/if}
  {:else if currentView === 'members'}
    {#if currentOrg}
      {#if showMemberForm}
        <MemberForm
          member={editingMember}
          onback={() => showMemberForm = false}
          onsaved={handleMemberSaved}
        />
      {:else}
        <MemberList
          bind:this={memberList}
          onedit={handleEditMember}
          onnew={handleNewMember}
        />
      {/if}
    {:else}
      <div class="p-6 text-gray-500 text-center pt-20">
        <p class="text-lg">Wybierz lub utwórz organizację</p>
      </div>
    {/if}
  {:else if currentView === 'generations'}
    {#if currentOrg}
      <GenerationLog />
    {:else}
      <div class="p-6 text-gray-500 text-center pt-20">
        <p class="text-lg">Wybierz lub utwórz organizację</p>
      </div>
    {/if}
  {:else if currentView === 'org-form'}
    <OrganizationForm onorgchanged={handleOrgChanged} />
  {/if}
</Layout>
