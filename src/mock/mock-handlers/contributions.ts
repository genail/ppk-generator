import { store, now } from '../mock-store';
import type { RawContribution } from '../mock-store';
import type { ContributionWithMember, UpsertContribution, Period } from '../../lib/types';

export function list_contributions(args: {
  organizationId: number;
  year: number;
  month: number;
}): ContributionWithMember[] {
  const orgMemberIds = store.members
    .filter(m => m.organization_id === args.organizationId)
    .map(m => m.id);

  const contribs = store.contributions.filter(
    c =>
      orgMemberIds.includes(c.member_id) &&
      c.period_year === args.year &&
      c.period_month === args.month
  );

  return contribs.map(c => {
    const member = store.members.find(m => m.id === c.member_id)!;
    return {
      id: c.id,
      member_id: c.member_id,
      period_year: c.period_year,
      period_month: c.period_month,
      employee_basic: c.employee_basic,
      employee_additional: c.employee_additional,
      employer_basic: c.employer_basic,
      employer_additional: c.employer_additional,
      reduced_basic_flag: c.reduced_basic_flag,
      source: c.source,
      updated_at: c.updated_at,
      pesel: member.pesel,
      first_name: member.first_name,
      last_name: member.last_name,
      gender: member.gender,
      date_of_birth: member.date_of_birth,
      citizenship: member.citizenship,
      second_name: member.second_name,
      doc_type: member.doc_type,
      doc_number: member.doc_number,
      member_status: member.status,
    };
  });
}

export function upsert_contribution(args: { data: UpsertContribution }): void {
  const existing = store.contributions.find(
    c =>
      c.member_id === args.data.member_id &&
      c.period_year === args.data.period_year &&
      c.period_month === args.data.period_month
  );

  if (existing) {
    existing.employee_basic = args.data.employee_basic ?? existing.employee_basic;
    existing.employee_additional = args.data.employee_additional ?? existing.employee_additional;
    existing.employer_basic = args.data.employer_basic ?? existing.employer_basic;
    existing.employer_additional = args.data.employer_additional ?? existing.employer_additional;
    existing.reduced_basic_flag = args.data.reduced_basic_flag ?? existing.reduced_basic_flag;
    existing.source = 'manual';
    existing.updated_at = now();
  } else {
    const contrib: RawContribution = {
      id: store.nextContributionId++,
      member_id: args.data.member_id,
      period_year: args.data.period_year,
      period_month: args.data.period_month,
      employee_basic: args.data.employee_basic ?? '0.00',
      employee_additional: args.data.employee_additional ?? '0.00',
      employer_basic: args.data.employer_basic ?? '0.00',
      employer_additional: args.data.employer_additional ?? '0.00',
      reduced_basic_flag: args.data.reduced_basic_flag ?? 'N',
      source: 'manual',
      updated_at: now(),
    };
    store.contributions.push(contrib);
  }
}

export function prefill_contributions(args: {
  organizationId: number;
  year: number;
  month: number;
}): number {
  // Determine previous period
  let prevYear = args.year;
  let prevMonth = args.month - 1;
  if (prevMonth === 0) {
    prevMonth = 12;
    prevYear--;
  }

  const orgMemberIds = store.members
    .filter(m => m.organization_id === args.organizationId && m.status === 'active')
    .map(m => m.id);

  // Members who already have contributions for the target period
  const existingMemberIds = new Set(
    store.contributions
      .filter(
        c =>
          orgMemberIds.includes(c.member_id) &&
          c.period_year === args.year &&
          c.period_month === args.month
      )
      .map(c => c.member_id)
  );

  // Previous period contributions for members missing current period
  const prevContribs = store.contributions.filter(
    c =>
      orgMemberIds.includes(c.member_id) &&
      !existingMemberIds.has(c.member_id) &&
      c.period_year === prevYear &&
      c.period_month === prevMonth
  );

  let count = 0;
  for (const prev of prevContribs) {
    const contrib: RawContribution = {
      id: store.nextContributionId++,
      member_id: prev.member_id,
      period_year: args.year,
      period_month: args.month,
      employee_basic: prev.employee_basic,
      employee_additional: prev.employee_additional,
      employer_basic: prev.employer_basic,
      employer_additional: prev.employer_additional,
      reduced_basic_flag: prev.reduced_basic_flag,
      source: 'prefill',
      updated_at: now(),
    };
    store.contributions.push(contrib);
    count++;
  }

  // Also create zero contributions for active members with no previous data and no current data
  for (const memberId of orgMemberIds) {
    if (existingMemberIds.has(memberId)) continue;
    if (prevContribs.some(c => c.member_id === memberId)) continue;

    const contrib: RawContribution = {
      id: store.nextContributionId++,
      member_id: memberId,
      period_year: args.year,
      period_month: args.month,
      employee_basic: '0.00',
      employee_additional: '0.00',
      employer_basic: '0.00',
      employer_additional: '0.00',
      reduced_basic_flag: 'N',
      source: 'prefill',
      updated_at: now(),
    };
    store.contributions.push(contrib);
    count++;
  }

  return count;
}

export function get_available_periods(args: { organizationId: number }): Period[] {
  const orgMemberIds = store.members
    .filter(m => m.organization_id === args.organizationId)
    .map(m => m.id);

  const periodSet = new Set<string>();
  for (const c of store.contributions) {
    if (orgMemberIds.includes(c.member_id)) {
      periodSet.add(`${c.period_year}-${c.period_month}`);
    }
  }

  // Also include periods from generations
  for (const g of store.generations) {
    if (g.organization_id === args.organizationId) {
      periodSet.add(`${g.period_year}-${g.period_month}`);
    }
  }

  return Array.from(periodSet)
    .map(p => {
      const [y, m] = p.split('-').map(Number);
      return { year: y, month: m };
    })
    .sort((a, b) => b.year - a.year || b.month - a.month);
}
