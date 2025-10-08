#!/usr/bin/env node

/**
 * Build script for AFFiNE Tauri application
 */

import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = resolve(__dirname, '..');

console.log('🚀 Building AFFiNE Tauri application...');

// Build the frontend first
console.log('📦 Building frontend...');

const frontendBuild = spawn('yarn', ['affine', '@affine/core', 'build'], {
  cwd: resolve(rootDir, '../../../..'),
  stdio: 'inherit',
  shell: true,
});

frontendBuild.on('close', (code) => {
  if (code !== 0) {
    console.error('❌ Frontend build failed');
    process.exit(code);
  }
  
  console.log('✅ Frontend built successfully');
  console.log('📦 Building Tauri application...');
  
  // Build Tauri
  const tauriBuild = spawn('yarn', ['tauri', 'build'], {
    cwd: rootDir,
    stdio: 'inherit',
    shell: true,
  });
  
  tauriBuild.on('close', (code) => {
    if (code !== 0) {
      console.error('❌ Tauri build failed');
      process.exit(code);
    }
    
    console.log('✅ Tauri application built successfully');
    console.log('📦 Output: src-tauri/target/release/bundle/');
  });
});
