import type { Organization } from './types';

// Global app state using Svelte 5 runes
let _currentOrg = $state<Organization | null>(null);
let _currentView = $state<string>('contributions');
let _toast = $state<{ message: string; type: 'success' | 'error' | 'info' } | null>(null);
let _toastTimeout: ReturnType<typeof setTimeout> | null = null;

export function getCurrentOrg(): Organization | null {
  return _currentOrg;
}

export function setCurrentOrg(org: Organization | null) {
  _currentOrg = org;
}

export function getCurrentView(): string {
  return _currentView;
}

export function setCurrentView(view: string) {
  _currentView = view;
}

export function getToast() {
  return _toast;
}

export function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
  if (_toastTimeout) clearTimeout(_toastTimeout);
  _toast = { message, type };
  _toastTimeout = setTimeout(() => {
    _toast = null;
  }, 4000);
}
