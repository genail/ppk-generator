const PESEL_WEIGHTS = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];

export interface PeselInfo {
  dateOfBirth: string;
  gender: string;
}

export function validatePesel(pesel: string): { valid: boolean; info?: PeselInfo; error?: string } {
  if (pesel.length !== 11) return { valid: false, error: 'PESEL musi mieć 11 cyfr' };
  if (!/^\d{11}$/.test(pesel)) return { valid: false, error: 'PESEL może zawierać tylko cyfry' };

  const digits = pesel.split('').map(Number);

  const sum = digits.slice(0, 10).reduce((acc, d, i) => acc + d * PESEL_WEIGHTS[i], 0);
  const check = (10 - (sum % 10)) % 10;
  if (check !== digits[10]) return { valid: false, error: 'Nieprawidłowa suma kontrolna' };

  const yearPart = digits[0] * 10 + digits[1];
  const monthPart = digits[2] * 10 + digits[3];
  const day = digits[4] * 10 + digits[5];

  let year: number, month: number;
  if (monthPart >= 1 && monthPart <= 12) { year = 1900 + yearPart; month = monthPart; }
  else if (monthPart >= 21 && monthPart <= 32) { year = 2000 + yearPart; month = monthPart - 20; }
  else if (monthPart >= 41 && monthPart <= 52) { year = 2100 + yearPart; month = monthPart - 40; }
  else if (monthPart >= 61 && monthPart <= 72) { year = 2200 + yearPart; month = monthPart - 60; }
  else if (monthPart >= 81 && monthPart <= 92) { year = 1800 + yearPart; month = monthPart - 80; }
  else return { valid: false, error: 'Nieprawidłowy miesiąc' };

  const dateOfBirth = `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
  const gender = digits[9] % 2 === 1 ? 'M' : 'K';

  return { valid: true, info: { dateOfBirth, gender } };
}

const NIP_WEIGHTS = [6, 5, 7, 2, 3, 4, 5, 6, 7];

export function validateNip(nip: string): { valid: boolean; error?: string } {
  const clean = nip.replace(/\D/g, '');
  if (clean.length !== 10) return { valid: false, error: 'NIP musi mieć 10 cyfr' };

  const digits = clean.split('').map(Number);
  const sum = digits.slice(0, 9).reduce((acc, d, i) => acc + d * NIP_WEIGHTS[i], 0);
  const check = sum % 11;

  if (check === 10 || check !== digits[9]) return { valid: false, error: 'Nieprawidłowa suma kontrolna' };
  return { valid: true };
}

const REGON_WEIGHTS = [8, 9, 2, 3, 4, 5, 6, 7];

export function validateRegon(regon: string): { valid: boolean; error?: string } {
  const clean = regon.replace(/\D/g, '');
  if (clean.length !== 9) return { valid: false, error: 'REGON musi mieć 9 cyfr' };

  const digits = clean.split('').map(Number);
  const sum = digits.slice(0, 8).reduce((acc, d, i) => acc + d * REGON_WEIGHTS[i], 0);
  let check = sum % 11;
  if (check === 10) check = 0;

  if (check !== digits[8]) return { valid: false, error: 'Nieprawidłowa suma kontrolna' };
  return { valid: true };
}
