import { expect, test } from 'bun:test';
import { connect } from 'solana-kite';
import { generateKeyPairSigner } from '@solana/kit';
import { fetchAddressInfo, getCreateInstruction } from '../../../clients/accountdata';
import { getKiteConnection } from '../../../scripts/rpc-config.js';

// Helper function to create padded byte arrays from strings
function createPaddedArray(str: string, size: number): Uint8Array {
  const bytes = new TextEncoder().encode(str);
  const result = new Uint8Array(size);
  const copySize = Math.min(bytes.length, size);
  result.set(bytes.slice(0, copySize));
  return result;
}

// Helper function to convert padded byte arrays back to strings
function parsePaddedString(bytes: Uint8Array): string {
  // Find the first null byte to determine actual string length
  const nullIndex = bytes.indexOf(0);
  const actualBytes = nullIndex === -1 ? bytes : bytes.slice(0, nullIndex);
  return new TextDecoder().decode(actualBytes);
}

test('basics:account-data:create', async () => {
  // Connect to Solana using Kite with Helius devnet RPC
  const { getRpcEndpoint, getWsEndpoint } = await import('../../../scripts/rpc-config.js');
  const rpcEndpoint = getRpcEndpoint('devnet');
  const wsEndpoint = getWsEndpoint('devnet');
  const kite = await connect(rpcEndpoint, wsEndpoint);

  // Create a test payer wallet and ensure it has some SOL
  const payer = await kite.createWallet();
  console.log('Created payer wallet:', payer.address);

  // Airdrop SOL to the payer if needed
  const airdropSignature = await kite.airdropIfRequired(
    payer.address,
    2_000_000_000n, // 2 SOL
    1_000_000_000n  // Minimum 1 SOL
  );
  if (airdropSignature) {
    console.log('Airdropped SOL, signature:', airdropSignature);
  }

  // Check balance
  const balance = await kite.getLamportBalance(payer.address);
  console.log('Payer balance:', Number(balance) / 1_000_000_000, 'SOL');

  // Create the address info account keypair
  const addressInfoKeypair = await generateKeyPairSigner();

  // Create the instruction for creating address info
  const createInstruction = getCreateInstruction({
    payer: payer,
    addressInfo: addressInfoKeypair,
    name: createPaddedArray('John Doe', 50),
    houseNumber: 123,
    street: createPaddedArray('Main St', 50),
    city: createPaddedArray('Anytown', 50),
  });

  // Send the transaction using Kite's simplified API
  const signature = await kite.sendTransactionFromInstructions({
    feePayer: payer,
    instructions: [createInstruction],
    commitment: 'confirmed',
  });

  console.log('Transaction signature:', signature);

  // Fetch the created account data
  const { data } = await fetchAddressInfo(kite.rpc, addressInfoKeypair.address);
  console.log('Address Info:', data);

  // Convert byte arrays back to strings for comparison
  const parsedData = {
    name: parsePaddedString(data.name),
    houseNumber: data.houseNumber,
    street: parsePaddedString(data.street),
    city: parsePaddedString(data.city),
  };

  expect(parsedData).toEqual({
    name: 'John Doe',
    houseNumber: 123,
    street: 'Main St',
    city: 'Anytown',
  });
});
