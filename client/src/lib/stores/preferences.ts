/**
 * Local storage backed preferences store
 */

import { browser } from '$app/environment';

const STORAGE_KEY = 'devtools_preferences';

interface Preferences {
  lastTool?: string;
  base64Mode?: 'encode' | 'decode';
}

function loadPreferences(): Preferences {
  if (!browser) return {};
  
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored ? JSON.parse(stored) : {};
  } catch {
    return {};
  }
}

function savePreferences(prefs: Preferences): void {
  if (!browser) return;
  
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs));
  } catch {
    // Ignore storage errors
  }
}

export function getPreference<K extends keyof Preferences>(key: K): Preferences[K] {
  return loadPreferences()[key];
}

export function setPreference<K extends keyof Preferences>(
  key: K,
  value: Preferences[K]
): void {
  const prefs = loadPreferences();
  prefs[key] = value;
  savePreferences(prefs);
}
