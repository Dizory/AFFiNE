#!/usr/bin/env node

/**
 * Development script for AFFiNE Tauri application
 */

import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = resolve(__dirname, '..');

console.log('🚀 Starting AFFiNE Tauri development server...');

// Start the development server
const devServer = spawn('yarn', ['tauri', 'dev'], {
  cwd: rootDir,
  stdio: 'inherit',
  shell: true,
  env: {
    ...process.env,
    RUST_LOG: 'info',
    RUST_BACKTRACE: '1',
  },
});

devServer.on('close', (code) => {
  console.log(`Dev server exited with code ${code}`);
  process.exit(code);
});

// Handle process termination
process.on('SIGINT', () => {
  console.log('\n🛑 Stopping development server...');
  devServer.kill('SIGINT');
});

process.on('SIGTERM', () => {
  console.log('\n🛑 Stopping development server...');
  devServer.kill('SIGTERM');
});
