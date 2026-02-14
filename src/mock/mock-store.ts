// In-memory state for mock backend

export interface RawContribution {
  id: number;
  member_id: number;
  period_year: number;
  period_month: number;
  employee_basic: string;
  employee_additional: string;
  employer_basic: string;
  employer_additional: string;
  reduced_basic_flag: string;
  source: string;
  updated_at: string;
}

export interface StoredGeneration {
  id: number;
  organization_id: number;
  period_year: number;
  period_month: number;
  generated_at: string;
  file_path: string;
  total_employee_basic: string;
  total_employer_basic: string;
  member_count: number;
}

import type { Organization, Member } from '../lib/types';
import type { GenerateResult } from '../lib/types';

export const store = {
  organizations: [] as Organization[],
  members: [] as Member[],
  contributions: [] as RawContribution[],
  generations: [] as StoredGeneration[],
  generationResults: new Map<number, GenerateResult>(),

  nextOrgId: 1,
  nextMemberId: 1,
  nextContributionId: 1,
  nextGenerationId: 1,
};

export function now(): string {
  return new Date().toISOString();
}
