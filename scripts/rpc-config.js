#!/usr/bin/env node

/**
 * RPC Configuration utility for Helius endpoints
 * Provides centralized RPC endpoint management for the project
 */

import dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Load environment variables
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
dotenv.config({ path: join(__dirname, '..', '.env') });

const HELIUS_API_KEY = process.env.HELIUS_API_KEY;

if (!HELIUS_API_KEY) {
  console.error('❌ HELIUS_API_KEY not found in environment variables');
  console.error('Please set HELIUS_API_KEY in your .env file');
  process.exit(1);
}

// Default RPC endpoints
const RPC_ENDPOINTS = {
  devnet: process.env.HELIUS_DEVNET_RPC || `https://devnet.helius-rpc.com/?api-key=${HELIUS_API_KEY}`,
  mainnet: process.env.HELIUS_MAINNET_RPC || `https://mainnet.helius-rpc.com/?api-key=${HELIUS_API_KEY}`,
  testnet: 'https://api.testnet.solana.com', // Helius doesn't provide testnet
  localhost: 'http://localhost:8899',
};

/**
 * Get RPC endpoint for a specific network
 * @param {string} network - Network name (devnet, mainnet, testnet, localhost)
 * @returns {string} RPC endpoint URL
 */
function getRpcEndpoint(network = 'devnet') {
  const endpoint = RPC_ENDPOINTS[network];
  if (!endpoint) {
    throw new Error(`Unsupported network: ${network}. Supported: ${Object.keys(RPC_ENDPOINTS).join(', ')}`);
  }
  return endpoint;
}

/**
 * Get WebSocket endpoint for a specific network
 * @param {string} network - Network name
 * @returns {string} WebSocket endpoint URL
 */
function getWsEndpoint(network = 'devnet') {
  const rpcUrl = getRpcEndpoint(network);
  
  // Convert HTTP to WebSocket for Helius endpoints
  if (rpcUrl.includes('helius-rpc.com')) {
    return rpcUrl.replace('https://', 'wss://').replace('http://', 'ws://');
  }
  
  // Default Solana WebSocket endpoints
  switch (network) {
    case 'devnet':
      return 'wss://api.devnet.solana.com';
    case 'testnet':
      return 'wss://api.testnet.solana.com';
    case 'mainnet':
      return 'wss://api.mainnet-beta.solana.com';
    case 'localhost':
      return 'ws://localhost:8900';
    default:
      throw new Error(`No WebSocket endpoint for network: ${network}`);
  }
}

/**
 * Get Kite connection string for a specific network
 * @param {string} network - Network name
 * @returns {string} Kite connection string
 */
function getKiteConnection(network = 'devnet') {
  // For Helius networks, return custom URL
  if (network === 'devnet' || network === 'mainnet') {
    return getRpcEndpoint(network);
  }
  
  // For other networks, use Kite's built-in names
  return network;
}

/**
 * Print available RPC endpoints
 */
function printEndpoints() {
  console.log('🔗 Available RPC Endpoints:');
  console.log('');
  Object.entries(RPC_ENDPOINTS).forEach(([network, endpoint]) => {
    console.log(`  ${network.padEnd(10)}: ${endpoint}`);
  });
  console.log('');
}

// CLI usage
if (import.meta.url === `file://${process.argv[1]}`) {
  const command = process.argv[2];
  const network = process.argv[3] || 'devnet';

  switch (command) {
    case 'get':
      console.log(getRpcEndpoint(network));
      break;
    case 'ws':
      console.log(getWsEndpoint(network));
      break;
    case 'kite':
      console.log(getKiteConnection(network));
      break;
    case 'list':
    case 'endpoints':
      printEndpoints();
      break;
    default:
      console.log('Usage: node scripts/rpc-config.js <command> [network]');
      console.log('');
      console.log('Commands:');
      console.log('  get [network]       Get RPC endpoint for network');
      console.log('  ws [network]        Get WebSocket endpoint for network');
      console.log('  kite [network]      Get Kite connection string for network');
      console.log('  list                List all available endpoints');
      console.log('');
      console.log('Networks: devnet, mainnet, testnet, localhost');
      break;
  }
}

export {
  getRpcEndpoint,
  getWsEndpoint,
  getKiteConnection,
  printEndpoints,
  RPC_ENDPOINTS,
  HELIUS_API_KEY,
};
