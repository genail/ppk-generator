import { invoke } from './invoke';
import type {
  Organization, CreateOrganization,
  Member, CreateMember, UpdateMember,
  ContributionWithMember, UpsertContribution, Period,
  Generation, GenerateResult, PeselValidationResult,
} from './types';

// Organizations
export const listOrganizations = () => invoke<Organization[]>('list_organizations');
export const getOrganization = (id: number) => invoke<Organization>('get_organization', { id });
export const createOrganization = (data: CreateOrganization) => invoke<Organization>('create_organization', { data });
export const updateOrganization = (id: number, data: CreateOrganization) => invoke<Organization>('update_organization', { id, data });
export const deleteOrganization = (id: number) => invoke<void>('delete_organization', { id });

// Members
export const listMembers = (organizationId: number) => invoke<Member[]>('list_members', { organizationId });
export const getMember = (id: number) => invoke<Member>('get_member', { id });
export const createMember = (data: CreateMember) => invoke<Member>('create_member', { data });
export const updateMember = (id: number, data: UpdateMember) => invoke<Member>('update_member', { id, data });
export const deleteMember = (id: number) => invoke<void>('delete_member', { id });
export const validatePesel = (peselStr: string) => invoke<PeselValidationResult>('validate_pesel', { peselStr });

// Contributions
export const listContributions = (organizationId: number, year: number, month: number) =>
  invoke<ContributionWithMember[]>('list_contributions', { organizationId, year, month });
export const upsertContribution = (data: UpsertContribution) => invoke<void>('upsert_contribution', { data });
export const prefillContributions = (organizationId: number, year: number, month: number) =>
  invoke<number>('prefill_contributions', { organizationId, year, month });
export const getAvailablePeriods = (organizationId: number) => invoke<Period[]>('get_available_periods', { organizationId });

// Generations
export const generatePpk = (organizationId: number, year: number, month: number) =>
  invoke<GenerateResult>('generate_ppk', { organizationId, year, month });
export const listGenerations = (organizationId: number) => invoke<Generation[]>('list_generations', { organizationId });
export const getGeneration = (id: number) => invoke<GenerateResult>('get_generation', { id });
export const exportGeneration = (id: number) => invoke<GenerateResult>('export_generation', { id });
