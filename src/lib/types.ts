export interface Organization {
  id: number;
  name: string;
  nip: string;
  regon: string;
  contact_person: string;
  created_at: string;
  updated_at: string;
}

export interface CreateOrganization {
  name: string;
  nip: string;
  regon: string;
  contact_person: string;
}

export interface Member {
  id: number;
  organization_id: number;
  pesel: string;
  first_name: string;
  last_name: string;
  gender: string;
  date_of_birth: string;
  citizenship: string;
  second_name: string;
  doc_type: string;
  doc_number: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CreateMember {
  organization_id: number;
  pesel: string;
  first_name: string;
  last_name: string;
  gender: string;
  date_of_birth: string;
  citizenship?: string;
  second_name?: string;
  doc_type?: string;
  doc_number?: string;
}

export interface UpdateMember {
  first_name: string;
  last_name: string;
  gender: string;
  date_of_birth: string;
  citizenship?: string;
  second_name?: string;
  doc_type?: string;
  doc_number?: string;
  status?: string;
}

export interface ContributionWithMember {
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
  pesel: string;
  first_name: string;
  last_name: string;
  gender: string;
  date_of_birth: string;
  citizenship: string;
  second_name: string;
  doc_type: string;
  doc_number: string;
  member_status: string;
}

export interface UpsertContribution {
  member_id: number;
  period_year: number;
  period_month: number;
  employee_basic?: string;
  employee_additional?: string;
  employer_basic?: string;
  employer_additional?: string;
  reduced_basic_flag?: string;
}

export interface Period {
  year: number;
  month: number;
}

export interface Generation {
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

export interface GenerateResult {
  generation: Generation;
  zip_bytes: number[];
  total_employee_basic: string;
  total_employee_additional: string;
  total_employer_basic: string;
  total_employer_additional: string;
  member_count: number;
}

export interface PeselValidationResult {
  valid: boolean;
  date_of_birth: string | null;
  gender: string | null;
  error: string | null;
}
