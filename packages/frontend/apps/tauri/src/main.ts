/**
 * Tauri application entry point
 * This file initializes the AFFiNE application in Tauri environment
 */

// Import Tauri API
import { appInfo } from './api';

// Inject the Tauri API into window for compatibility with AFFiNE core
declare global {
  interface Window {
    affine?: any;
    appInfo?: typeof appInfo;
  }
}

// Make API available globally
window.appInfo = appInfo;

// Import and bootstrap the AFFiNE core application
async function bootstrap() {
  try {
    console.log('[Tauri] Initializing AFFiNE...');
    
    // Import AFFiNE core bootstrap
    // This assumes @affine/core exports a bootstrap function
    const { bootstrap: affineBootstrap } = await import('@affine/core/bootstrap');
    
    await affineBootstrap();
    
    console.log('[Tauri] AFFiNE initialized successfully');
  } catch (error) {
    console.error('[Tauri] Failed to initialize AFFiNE:', error);
  }
}

// Start the application
bootstrap();
