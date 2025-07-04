import { describe, test, beforeAll } from 'bun:test';
import { generateKeyPair } from '@solana/web3.js';
import { createSolanaRpc, createSolanaRpcSubscriptions, devnet } from '@solana/web3.js';

// Import generated client (will be available after running gen:client)
// import { cross_program_invocationProgram } from '../clients/cross-program-invocation';

describe('cross-program-invocation', () => {
  let rpc: ReturnType<typeof createSolanaRpc>;
  let rpcSubscriptions: ReturnType<typeof createSolanaRpcSubscriptions>;
  let payer: CryptoKeyPair;

  beforeAll(async () => {
    rpc = createSolanaRpc(devnet('https://api.devnet.solana.com'));
    rpcSubscriptions = createSolanaRpcSubscriptions(devnet('wss://api.devnet.solana.com'));
    payer = await generateKeyPair();
  });

  test('should initialize successfully', async () => {
    // TODO: Implement test logic
    console.log('Test for cross-program-invocation - implement your test logic here');
  });
});
