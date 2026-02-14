import { invoke } from './invoke';

export const POLISH_MONTHS = [
  '', 'Styczeń', 'Luty', 'Marzec', 'Kwiecień', 'Maj', 'Czerwiec',
  'Lipiec', 'Sierpień', 'Wrzesień', 'Październik', 'Listopad', 'Grudzień'
];

export function formatPeriod(year: number, month: number): string {
  return `${POLISH_MONTHS[month]} ${year}`;
}

/** Convert dot decimal to comma display ("94.38" -> "94,38") */
export function dotToComma(value: string): string {
  return value.replace('.', ',');
}

/** Convert comma display to dot storage ("94,38" -> "94.38") */
export function commaToDot(value: string): string {
  return value.replace(',', '.');
}

/** Format money for display with comma decimal */
export function formatMoney(value: string): string {
  const num = parseFloat(value);
  if (isNaN(num)) return '0,00';
  return num.toFixed(2).replace('.', ',');
}

/** Validate money string (dot notation), returns error message or null */
export function validateMoney(value: string): string | null {
  if (!value || value.trim() === '') return null;
  const dotValue = commaToDot(value);
  const num = parseFloat(dotValue);
  if (isNaN(num)) return 'Nieprawidłowa kwota';
  if (num < 0) return 'Kwota nie może być ujemna';
  const parts = dotValue.split('.');
  if (parts.length === 2 && parts[1].length > 2) return 'Maksymalnie 2 miejsca po przecinku';
  return null;
}

/** Save ZIP bytes to file via native dialog + Rust write */
export async function saveZipFile(zipBytes: number[], suggestedName: string): Promise<boolean> {
  let saveFn: (options: any) => Promise<string | null>;

  if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
    const { save } = await import('@tauri-apps/plugin-dialog');
    saveFn = save;
  } else {
    // Mock: pretend user picked a path
    saveFn = async () => `/mock/${suggestedName}`;
  }

  const path = await saveFn({
    defaultPath: suggestedName,
    filters: [{ name: 'ZIP', extensions: ['zip'] }],
  });

  if (!path) return false;

  await invoke<void>('save_zip_file', { zipBytes, path });
  return true;
}

export function currentPeriod(): { year: number; month: number } {
  const now = new Date();
  // Default to previous month
  let month = now.getMonth(); // 0-based, so this is previous month
  let year = now.getFullYear();
  if (month === 0) {
    month = 12;
    year--;
  }
  return { year, month };
}
