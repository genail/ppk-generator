<script lang="ts">
  import type { Member } from '../lib/types';
  import { createMember, updateMember } from '../lib/api';
  import { getCurrentOrg, showToast } from '../lib/stores.svelte';
  import PeselInput from '../components/PeselInput.svelte';

  interface Props {
    member: Member | null;
    onback: () => void;
    onsaved: () => void;
  }

  let { member, onback, onsaved }: Props = $props();

  const currentOrg = $derived(getCurrentOrg());

  let pesel = $state('');
  let firstName = $state('');
  let lastName = $state('');
  let gender = $state('');
  let dateOfBirth = $state('');
  let citizenship = $state('PL');
  let secondName = $state('');
  let docType = $state('');
  let docNumber = $state('');
  let status = $state('active');
  let saving = $state(false);
  let peselValid = $state(false);

  const isEdit = $derived(member !== null);

  $effect(() => {
    if (member) {
      pesel = member.pesel;
      firstName = member.first_name;
      lastName = member.last_name;
      gender = member.gender;
      dateOfBirth = member.date_of_birth;
      citizenship = member.citizenship;
      secondName = member.second_name;
      docType = member.doc_type;
      docNumber = member.doc_number;
      status = member.status;
      peselValid = true;
    }
  });

  function handlePeselValidated(info: { dateOfBirth: string; gender: string } | null) {
    if (info) {
      dateOfBirth = info.dateOfBirth;
      gender = info.gender;
      peselValid = true;
    } else {
      peselValid = false;
    }
  }

  async function handleSubmit() {
    if (!currentOrg) return;
    if (!firstName.trim() || !lastName.trim()) {
      showToast('Imię i nazwisko są wymagane', 'error');
      return;
    }

    saving = true;
    try {
      if (isEdit && member) {
        await updateMember(member.id, {
          first_name: firstName.trim(),
          last_name: lastName.trim(),
          gender,
          date_of_birth: dateOfBirth,
          citizenship,
          second_name: secondName.trim(),
          doc_type: docType.trim(),
          doc_number: docNumber.trim(),
          status,
        });
        showToast('Uczestnik zaktualizowany', 'success');
      } else {
        if (!peselValid) {
          showToast('Wprowadź prawidłowy PESEL', 'error');
          saving = false;
          return;
        }
        await createMember({
          organization_id: currentOrg.id,
          pesel,
          first_name: firstName.trim(),
          last_name: lastName.trim(),
          gender,
          date_of_birth: dateOfBirth,
          citizenship,
          second_name: secondName.trim(),
          doc_type: docType.trim(),
          doc_number: docNumber.trim(),
        });
        showToast('Uczestnik dodany', 'success');
      }
      onsaved();
    } catch (e: any) {
      showToast(`Błąd: ${e}`, 'error');
    } finally {
      saving = false;
    }
  }
</script>

<div class="p-6 max-w-lg">
  <button
    class="text-sm text-blue-600 hover:text-blue-700 mb-4"
    onclick={onback}
  >
    &larr; Powrót do listy
  </button>

  <h2 class="text-xl font-semibold text-gray-900 mb-6">
    {isEdit ? 'Edytuj uczestnika' : 'Nowy uczestnik'}
  </h2>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">PESEL</label>
      {#if isEdit}
        <input
          type="text"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm bg-gray-50"
          value={pesel}
          disabled
        />
      {:else}
        <PeselInput
          value={pesel}
          oninput={(v) => pesel = v}
          onvalidated={handlePeselValidated}
        />
      {/if}
    </div>

    <div class="grid grid-cols-2 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Imię</label>
        <input
          type="text"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-1 focus:ring-blue-500"
          bind:value={firstName}
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Nazwisko</label>
        <input
          type="text"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-1 focus:ring-blue-500"
          bind:value={lastName}
        />
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Drugie imię</label>
      <input
        type="text"
        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-1 focus:ring-blue-500"
        bind:value={secondName}
        placeholder="(opcjonalnie)"
      />
    </div>

    <div class="grid grid-cols-2 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Płeć</label>
        <select
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm bg-gray-50"
          bind:value={gender}
          disabled
        >
          <option value="">—</option>
          <option value="M">Mężczyzna</option>
          <option value="K">Kobieta</option>
        </select>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Data urodzenia</label>
        <input
          type="date"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm bg-gray-50"
          bind:value={dateOfBirth}
          disabled
        />
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Obywatelstwo</label>
      <input
        type="text"
        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-1 focus:ring-blue-500"
        bind:value={citizenship}
      />
    </div>

    <div class="grid grid-cols-2 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Typ dokumentu</label>
        <select
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm bg-white focus:ring-1 focus:ring-blue-500"
          bind:value={docType}
        >
          <option value="">Brak</option>
          <option value="D">Dowód osobisty</option>
          <option value="P">Paszport</option>
        </select>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Numer dokumentu</label>
        <input
          type="text"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-1 focus:ring-blue-500"
          bind:value={docNumber}
          placeholder="(opcjonalnie)"
        />
      </div>
    </div>

    {#if isEdit}
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Status</label>
        <select
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm bg-white focus:ring-1 focus:ring-blue-500"
          bind:value={status}
        >
          <option value="active">Aktywny</option>
          <option value="resigned">Rezygnacja</option>
          <option value="terminated">Zwolniony</option>
        </select>
      </div>
    {/if}

    <div class="flex gap-3 pt-2">
      <button
        type="submit"
        class="px-4 py-2 text-sm text-white bg-blue-600 rounded-lg hover:bg-blue-700 disabled:opacity-50"
        disabled={saving}
      >
        {saving ? 'Zapisywanie...' : (isEdit ? 'Zapisz' : 'Dodaj')}
      </button>
      <button
        type="button"
        class="px-4 py-2 text-sm text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
        onclick={onback}
      >
        Anuluj
      </button>
    </div>
  </form>
</div>
