/**
 * Tauri API wrapper for AFFiNE
 * This replaces the Electron IPC API with Tauri commands
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, emit } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { check, Update } from '@tauri-apps/plugin-updater';
import { 
  readText as clipboardReadText,
  writeText as clipboardWriteText 
} from '@tauri-apps/plugin-clipboard-manager';
import { message, ask, open, save } from '@tauri-apps/plugin-dialog';
import { 
  readTextFile, 
  writeTextFile,
  readDir,
  removeFile,
  removeDir,
  createDir,
  exists,
  BaseDirectory
} from '@tauri-apps/plugin-fs';
import { open as shellOpen } from '@tauri-apps/plugin-shell';

// Types
export interface WindowState {
  fullscreen: boolean;
  maximized: boolean;
  focused: boolean;
  visible: boolean;
}

export interface UpdateInfo {
  version: string;
  current_version: string;
  date?: string;
  body?: string;
}

export interface UpdaterConfig {
  auto_check: boolean;
  auto_download: boolean;
  check_on_startup: boolean;
}

export interface RecordingStatus {
  is_recording: boolean;
  available: boolean;
}

// Debug APIs
export const debug = {
  revealLogFile: async (): Promise<void> => {
    return invoke('reveal_log_file');
  },
  
  logFilePath: async (): Promise<string> => {
    return invoke('get_log_file_path');
  },
};

// UI APIs
export const ui = {
  showMainWindow: async (): Promise<void> => {
    return invoke('show_main_window');
  },
  
  hideMainWindow: async (): Promise<void> => {
    return invoke('hide_main_window');
  },
  
  toggleWindow: async (): Promise<void> => {
    return invoke('toggle_window');
  },
  
  getWindowState: async (): Promise<WindowState> => {
    return invoke('get_window_state');
  },
  
  setWindowState: async (fullscreen?: boolean, maximized?: boolean): Promise<void> => {
    return invoke('set_window_state', { fullscreen, maximized });
  },
  
  findInPage: async (text: string, options?: any): Promise<void> => {
    return invoke('find_in_page', { text, options });
  },
  
  stopFindInPage: async (): Promise<void> => {
    return invoke('stop_find_in_page');
  },
};

// Clipboard APIs
export const clipboard = {
  readText: async (): Promise<string> => {
    return clipboardReadText();
  },
  
  writeText: async (text: string): Promise<void> => {
    return clipboardWriteText(text);
  },
  
  readImage: async (): Promise<Uint8Array> => {
    return invoke('read_image');
  },
  
  writeImage: async (data: Uint8Array): Promise<void> => {
    return invoke('write_image', { data });
  },
};

// Storage APIs
export const configStorage = {
  get: async (key?: string): Promise<any> => {
    return invoke('get_config', { key });
  },
  
  set: async (key: string, value: any): Promise<void> => {
    return invoke('set_config', { key, value });
  },
  
  delete: async (key: string): Promise<void> => {
    return invoke('delete_config', { key });
  },
};

export const sharedStorage = {
  get: async (key?: string): Promise<any> => {
    return invoke('get_shared_storage', { key });
  },
  
  set: async (key: string, value: any): Promise<void> => {
    return invoke('set_shared_storage', { key, value });
  },
};

// Updater APIs
export const updater = {
  currentVersion: async (): Promise<string> => {
    return invoke('get_current_version');
  },
  
  checkForUpdates: async (): Promise<UpdateInfo | null> => {
    return invoke('check_for_updates');
  },
  
  downloadUpdate: async (): Promise<void> => {
    return invoke('download_update');
  },
  
  quitAndInstall: async (): Promise<void> => {
    return invoke('install_update');
  },
  
  getConfig: async (): Promise<UpdaterConfig> => {
    return invoke('get_updater_config');
  },
  
  setConfig: async (config: Partial<UpdaterConfig>): Promise<void> => {
    return invoke('set_updater_config', { config });
  },
};

// Recording APIs
export const recording = {
  getStatus: async (): Promise<RecordingStatus> => {
    return invoke('get_recording_status');
  },
  
  start: async (): Promise<string> => {
    return invoke('start_recording');
  },
  
  stop: async (): Promise<void> => {
    return invoke('stop_recording');
  },
  
  getRecordings: async (): Promise<any[]> => {
    return invoke('get_recordings');
  },
};

// I18n APIs
export const i18n = {
  changeLanguage: async (language: string): Promise<void> => {
    return invoke('change_language', { language });
  },
};

// Worker APIs
export const worker = {
  spawn: async (workerId: string, script: string): Promise<string> => {
    return invoke('spawn_worker', { workerId, script });
  },
  
  terminate: async (workerId: string): Promise<void> => {
    return invoke('terminate_worker', { workerId });
  },
};

// Native Storage APIs (nbstore integration)
export const nativeStorage = {
  // Blob storage
  getBlob: async (key: string): Promise<Uint8Array> => {
    return invoke('native_storage_get_blob', { key });
  },
  
  setBlob: async (key: string, data: Uint8Array): Promise<void> => {
    return invoke('native_storage_set_blob', { key, data });
  },
  
  deleteBlob: async (key: string): Promise<void> => {
    return invoke('native_storage_delete_blob', { key });
  },
  
  // Doc storage
  getDoc: async (docId: string): Promise<Uint8Array> => {
    return invoke('native_storage_get_doc', { docId });
  },
  
  setDoc: async (docId: string, data: Uint8Array): Promise<void> => {
    return invoke('native_storage_set_doc', { docId, data });
  },
  
  deleteDoc: async (docId: string): Promise<void> => {
    return invoke('native_storage_delete_doc', { docId });
  },
};

// Events
export const events = {
  on: listen,
  emit,
};

// Export all APIs as a single object compatible with Electron API
export const api = {
  debug,
  ui,
  clipboard,
  configStorage,
  sharedStorage,
  updater,
  recording,
  i18n,
  worker,
  nativeStorage,
  events,
};

// App info (similar to Electron's appInfo)
export const appInfo = {
  electron: false,
  tauri: true,
  windowName: 'main',
  scheme: 'affine',
};

export default api;
