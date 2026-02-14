import { store, now } from '../mock-store';
import { validatePesel } from '../../lib/validation';
import type { Member, CreateMember, UpdateMember, PeselValidationResult } from '../../lib/types';

export function list_members(args: { organizationId: number }): Member[] {
  return store.members
    .filter(m => m.organization_id === args.organizationId)
    .map(m => ({ ...m }));
}

export function get_member(args: { id: number }): Member {
  const member = store.members.find(m => m.id === args.id);
  if (!member) throw new Error(`Member ${args.id} not found`);
  return { ...member };
}

export function create_member(args: { data: CreateMember }): Member {
  const member: Member = {
    id: store.nextMemberId++,
    organization_id: args.data.organization_id,
    pesel: args.data.pesel,
    first_name: args.data.first_name,
    last_name: args.data.last_name,
    gender: args.data.gender,
    date_of_birth: args.data.date_of_birth,
    citizenship: args.data.citizenship ?? 'PL',
    second_name: args.data.second_name ?? '',
    doc_type: args.data.doc_type ?? '',
    doc_number: args.data.doc_number ?? '',
    status: 'active',
    created_at: now(),
    updated_at: now(),
  };
  store.members.push(member);
  return { ...member };
}

export function update_member(args: { id: number; data: UpdateMember }): Member {
  const member = store.members.find(m => m.id === args.id);
  if (!member) throw new Error(`Member ${args.id} not found`);

  member.first_name = args.data.first_name;
  member.last_name = args.data.last_name;
  member.gender = args.data.gender;
  member.date_of_birth = args.data.date_of_birth;
  member.citizenship = args.data.citizenship ?? member.citizenship;
  member.second_name = args.data.second_name ?? member.second_name;
  member.doc_type = args.data.doc_type ?? member.doc_type;
  member.doc_number = args.data.doc_number ?? member.doc_number;
  member.status = args.data.status ?? member.status;
  member.updated_at = now();

  return { ...member };
}

export function delete_member(args: { id: number }): void {
  const idx = store.members.findIndex(m => m.id === args.id);
  if (idx === -1) throw new Error(`Member ${args.id} not found`);

  // Cascade: remove member's contributions
  store.contributions = store.contributions.filter(c => c.member_id !== args.id);

  store.members.splice(idx, 1);
}

export function validate_pesel(args: { peselStr: string }): PeselValidationResult {
  const result = validatePesel(args.peselStr);
  return {
    valid: result.valid,
    date_of_birth: result.info?.dateOfBirth ?? null,
    gender: result.info?.gender ?? null,
    error: result.error ?? null,
  };
}
