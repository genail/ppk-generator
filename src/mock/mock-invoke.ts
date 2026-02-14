import * as orgHandlers from './mock-handlers/organizations';
import * as memberHandlers from './mock-handlers/members';
import * as contribHandlers from './mock-handlers/contributions';
import * as genHandlers from './mock-handlers/generations';

type Handler = (args: any) => any;

const handlers: Record<string, Handler> = {
  // Organizations
  list_organizations: orgHandlers.list_organizations,
  get_organization: orgHandlers.get_organization,
  create_organization: orgHandlers.create_organization,
  update_organization: orgHandlers.update_organization,
  delete_organization: orgHandlers.delete_organization,

  // Members
  list_members: memberHandlers.list_members,
  get_member: memberHandlers.get_member,
  create_member: memberHandlers.create_member,
  update_member: memberHandlers.update_member,
  delete_member: memberHandlers.delete_member,
  validate_pesel: memberHandlers.validate_pesel,

  // Contributions
  list_contributions: contribHandlers.list_contributions,
  upsert_contribution: contribHandlers.upsert_contribution,
  prefill_contributions: contribHandlers.prefill_contributions,
  get_available_periods: contribHandlers.get_available_periods,

  // Generations
  generate_ppk: genHandlers.generate_ppk,
  list_generations: genHandlers.list_generations,
  get_generation: genHandlers.get_generation,
  export_generation: genHandlers.export_generation,
  save_zip_file: genHandlers.save_zip_file,
};

export async function mockInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const handler = handlers[cmd];
  if (!handler) {
    throw new Error(`[mock] Unknown command: ${cmd}`);
  }

  return handler(args ?? {}) as T;
}
