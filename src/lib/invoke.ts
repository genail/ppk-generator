type InvokeFn = <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

let cachedInvoke: InvokeFn | null = null;

function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!cachedInvoke) {
    if (isTauri()) {
      const mod = await import('@tauri-apps/api/core');
      cachedInvoke = mod.invoke;
    } else {
      const { mockInvoke } = await import('../mock/mock-invoke');
      cachedInvoke = mockInvoke;
      console.info('[mock] Tauri runtime not detected — using in-memory mock backend');
    }
  }
  return cachedInvoke<T>(cmd, args);
}
