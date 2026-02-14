<script lang="ts">
  import { getCurrentOrg, setCurrentOrg, setCurrentView, showToast } from '../lib/stores.svelte';
  import { createOrganization, updateOrganization, deleteOrganization } from '../lib/api';
  import { validateNip, validateRegon } from '../lib/validation';
  import ConfirmDialog from '../components/ConfirmDialog.svelte';

  interface Props {
    onorgchanged: () => void;
  }

  let { onorgchanged }: Props = $props();

  const currentOrg = $derived(getCurrentOrg());

  let name = $state('');
  let nip = $state('');
  let regon = $state('');
  let contactPerson = $state('');
  let errors = $state<Record<string, string>>({});
  let saving = $state(false);
  let confirmDelete = $state(false);

  // Load current org data when it changes
  $effect(() => {
    if (currentOrg) {
      name = currentOrg.name;
      nip = currentOrg.nip;
      regon = currentOrg.regon;
      contactPerson = currentOrg.contact_person;
    } else {
      name = '';
      nip = '';
      regon = '';
      contactPerson = '';
    }
    errors = {};
  });

  function validate(): boolean {
    const e: Record<string, string> = {};

    if (!name.trim()) e.name = 'Nazwa jest wymagana';

    const nipResult = validateNip(nip);
    if (!nipResult.valid) e.nip = nipResult.error!;

    const regonResult = validateRegon(regon);
    if (!regonResult.valid) e.regon = regonResult.error!;

    errors = e;
    return Object.keys(e).length === 0;
  }

  async function handleSubmit() {
    if (!validate()) return;
    saving = true;
    try {
      const data = { name: name.trim(), nip: nip.trim(), regon: regon.trim(), contact_person: contactPerson.trim() };
      if (currentOrg) {
        const updated = await updateOrganization(currentOrg.id, data);
        setCurrentOrg(updated);
        showToast('Organizacja zaktualizowana', 'success');
      } else {
        const created = await createOrganization(data);
        setCurrentOrg(created);
        showToast('Organizacja utworzona', 'success');
      }
      onorgchanged();
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!currentOrg) return;
    try {
      await deleteOrganization(currentOrg.id);
      setCurrentOrg(null);
      onorgchanged();
      showToast('Organizacja usunięta', 'success');
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      confirmDelete = false;
    }
  }

  function handleNew() {
    setCurrentOrg(null);
  }
</script>

<div class="p-6 max-w-lg">
  <div class="flex items-center justify-between mb-6">
    <h2 class="text-xl font-semibold text-gray-900">
      {currentOrg ? 'Edytuj organizację' : 'Nowa organizacja'}
    </h2>
    {#if currentOrg}
      <button
        class="text-sm text-blue-600 hover:text-blue-700"
        onclick={handleNew}
      >
        + Nowa
      </button>
    {/if}
  </div>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Nazwa</label>
      <input
        type="text"
        class="w-full px-3 py-2 border rounded-lg text-sm focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        class:border-red-500={errors.name}
        class:border-gray-300={!errors.name}
        bind:value={name}
        placeholder="Nazwa firmy"
      />
      {#if errors.name}<p class="text-xs text-red-500 mt-1">{errors.name}</p>{/if}
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">NIP</label>
      <input
        type="text"
        maxlength="10"
        class="w-full px-3 py-2 border rounded-lg text-sm focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        class:border-red-500={errors.nip}
        class:border-gray-300={!errors.nip}
        bind:value={nip}
        placeholder="0000000000"
      />
      {#if errors.nip}<p class="text-xs text-red-500 mt-1">{errors.nip}</p>{/if}
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">REGON</label>
      <input
        type="text"
        maxlength="9"
        class="w-full px-3 py-2 border rounded-lg text-sm focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        class:border-red-500={errors.regon}
        class:border-gray-300={!errors.regon}
        bind:value={regon}
        placeholder="000000000"
      />
      {#if errors.regon}<p class="text-xs text-red-500 mt-1">{errors.regon}</p>{/if}
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Osoba kontaktowa</label>
      <input
        type="text"
        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        bind:value={contactPerson}
        placeholder="Imię Nazwisko"
      />
    </div>

    <div class="flex gap-3 pt-2">
      <button
        type="submit"
        class="px-4 py-2 text-sm text-white bg-blue-600 rounded-lg hover:bg-blue-700 disabled:opacity-50"
        disabled={saving}
      >
        {saving ? 'Zapisywanie...' : (currentOrg ? 'Zapisz' : 'Utwórz')}
      </button>

      {#if currentOrg}
        <button
          type="button"
          class="px-4 py-2 text-sm text-red-600 bg-red-50 rounded-lg hover:bg-red-100"
          onclick={() => confirmDelete = true}
        >
          Usuń
        </button>
      {/if}
    </div>
  </form>
</div>

<ConfirmDialog
  open={confirmDelete}
  title="Usuń organizację"
  message="Czy na pewno chcesz usunąć tę organizację? Zostaną usunięci wszyscy uczestnicy i składki."
  onconfirm={handleDelete}
  oncancel={() => confirmDelete = false}
/>
