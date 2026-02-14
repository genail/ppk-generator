import { store, now } from '../mock-store';
import type { StoredGeneration } from '../mock-store';
import type { Generation, GenerateResult } from '../../lib/types';

export function generate_ppk(args: {
  organizationId: number;
  year: number;
  month: number;
}): GenerateResult {
  const orgMemberIds = store.members
    .filter(m => m.organization_id === args.organizationId && m.status === 'active')
    .map(m => m.id);

  const contribs = store.contributions.filter(
    c =>
      orgMemberIds.includes(c.member_id) &&
      c.period_year === args.year &&
      c.period_month === args.month
  );

  const totalEmployeeBasic = contribs
    .reduce((sum, c) => sum + parseFloat(c.employee_basic || '0'), 0)
    .toFixed(2);
  const totalEmployeeAdditional = contribs
    .reduce((sum, c) => sum + parseFloat(c.employee_additional || '0'), 0)
    .toFixed(2);
  const totalEmployerBasic = contribs
    .reduce((sum, c) => sum + parseFloat(c.employer_basic || '0'), 0)
    .toFixed(2);
  const totalEmployerAdditional = contribs
    .reduce((sum, c) => sum + parseFloat(c.employer_additional || '0'), 0)
    .toFixed(2);

  const gen: StoredGeneration = {
    id: store.nextGenerationId++,
    organization_id: args.organizationId,
    period_year: args.year,
    period_month: args.month,
    generated_at: now(),
    file_path: `mock_ppk_${args.year}_${String(args.month).padStart(2, '0')}.zip`,
    total_employee_basic: totalEmployeeBasic,
    total_employer_basic: totalEmployerBasic,
    member_count: contribs.length,
  };
  store.generations.push(gen);

  const generation: Generation = { ...gen };
  // Stub zip_bytes — a minimal placeholder
  const zipBytes = [80, 75, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  const result: GenerateResult = {
    generation,
    zip_bytes: zipBytes,
    total_employee_basic: totalEmployeeBasic,
    total_employee_additional: totalEmployeeAdditional,
    total_employer_basic: totalEmployerBasic,
    total_employer_additional: totalEmployerAdditional,
    member_count: contribs.length,
  };

  store.generationResults.set(gen.id, result);
  return result;
}

export function list_generations(args: { organizationId: number }): Generation[] {
  return store.generations
    .filter(g => g.organization_id === args.organizationId)
    .map(g => ({ ...g }))
    .sort((a, b) => b.id - a.id);
}

export function get_generation(args: { id: number }): GenerateResult {
  const result = store.generationResults.get(args.id);
  if (!result) throw new Error(`Generation ${args.id} not found`);
  return { ...result, generation: { ...result.generation } };
}

export function export_generation(args: { id: number }): GenerateResult {
  return get_generation(args);
}

export function save_zip_file(_args: { zipBytes: number[]; path: string }): void {
  // No-op in mock — browser can't write files
}
