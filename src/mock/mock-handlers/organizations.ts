import { store, now } from '../mock-store';
import type { CreateOrganization, Organization } from '../../lib/types';

export function list_organizations(): Organization[] {
  return [...store.organizations];
}

export function get_organization(args: { id: number }): Organization {
  const org = store.organizations.find(o => o.id === args.id);
  if (!org) throw new Error(`Organization ${args.id} not found`);
  return { ...org };
}

export function create_organization(args: { data: CreateOrganization }): Organization {
  const org: Organization = {
    id: store.nextOrgId++,
    name: args.data.name,
    nip: args.data.nip,
    regon: args.data.regon,
    contact_person: args.data.contact_person,
    created_at: now(),
    updated_at: now(),
  };
  store.organizations.push(org);
  return { ...org };
}

export function update_organization(args: { id: number; data: CreateOrganization }): Organization {
  const org = store.organizations.find(o => o.id === args.id);
  if (!org) throw new Error(`Organization ${args.id} not found`);
  org.name = args.data.name;
  org.nip = args.data.nip;
  org.regon = args.data.regon;
  org.contact_person = args.data.contact_person;
  org.updated_at = now();
  return { ...org };
}

export function delete_organization(args: { id: number }): void {
  const idx = store.organizations.findIndex(o => o.id === args.id);
  if (idx === -1) throw new Error(`Organization ${args.id} not found`);

  // Cascade: find members of this org
  const memberIds = store.members
    .filter(m => m.organization_id === args.id)
    .map(m => m.id);

  // Remove contributions for those members
  store.contributions = store.contributions.filter(
    c => !memberIds.includes(c.member_id)
  );

  // Remove members
  store.members = store.members.filter(m => m.organization_id !== args.id);

  // Remove generations
  store.generations = store.generations.filter(g => g.organization_id !== args.id);

  // Remove the org
  store.organizations.splice(idx, 1);
}
