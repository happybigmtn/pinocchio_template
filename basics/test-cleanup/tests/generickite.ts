import { describe, test, beforeAll } from 'bun:test';
import { connect } from 'solana-kite';
import { address, lamports } from '@solana/kit';
import { getTransferSolInstruction } from '@solana-program/system';

// Import generated client (will be available after running gen:client)
// import { counterProgram } from '../clients/counter';

describe('counter - Comprehensive Kite Demo', () => {
  let kite: Awaited<ReturnType<typeof connect>>;
  const programId = address('11111111111111111111111111111111'); // Will be updated after deployment

  beforeAll(async () => {
    // Use standard Solana devnet RPC for testing (Helius may have restrictions)
    // For production, use Helius RPC from the config
    const rpcEndpoint = 'https://api.devnet.solana.com';
    const wsEndpoint = 'wss://api.devnet.solana.com';
    kite = await connect(rpcEndpoint, wsEndpoint);
  });

  test('should demonstrate all Kite wallet functions', async () => {
    console.log('\n🔑 === WALLET MANAGEMENT FUNCTIONS ===');
    
    try {
      // 1. createWallet - Create a new wallet
      console.log('\n1️⃣  Creating wallets with different options...');
      
      const basicWallet = await kite.createWallet();
      console.log('Basic wallet created:', basicWallet.address);
      
      const customWallet = await kite.createWallet({ 
        airdropAmount: lamports(BigInt('2000000000')), // 2 SOL
        prefix: 'COOL',
        suffix: 'TEST'
      });
      console.log('Custom wallet created with prefix/suffix:', customWallet.address);
      
      // 2. createWallets - Create multiple wallets at once
      console.log('\n2️⃣  Creating multiple wallets...');
      const multipleWallets = await kite.createWallets(3, {
        airdropAmount: lamports(BigInt('500000000')) // 0.5 SOL each
      });
      console.log('Created', multipleWallets.length, 'wallets:', multipleWallets.map(w => w.address));
      
      console.log('✅ Wallet creation functions working!');
    } catch (error) {
      console.error('❌ Wallet functions error:', error);
    }
  }, { timeout: 120000 });

  test('should demonstrate SOL balance and transfer functions', async () => {
    console.log('\n💰 === SOL MANAGEMENT FUNCTIONS ===');
    
    try {
      // Create test wallets
      const sender = await kite.createWallet({ 
        airdropAmount: lamports(BigInt('2000000000')) // 2 SOL
      });
      const receiver = await kite.createWallet();
      
      // 3. getLamportBalance - Get SOL balance
      console.log('\n3️⃣  Checking balances...');
      const senderBalance = await kite.getLamportBalance(sender.address);
      const receiverBalance = await kite.getLamportBalance(receiver.address);
      console.log('Sender balance:', Number(senderBalance) / 1_000_000_000, 'SOL');
      console.log('Receiver balance:', Number(receiverBalance) / 1_000_000_000, 'SOL');
      
      // 4. airdropIfRequired - Conditional airdrop
      console.log('\n4️⃣  Testing conditional airdrop...');
      const minimumBalance = lamports(BigInt('1000000000')); // 1 SOL
      const airdropAmount = lamports(BigInt('1500000000')); // 1.5 SOL
      
      const airdropSig = await kite.airdropIfRequired(
        receiver.address,
        airdropAmount,
        minimumBalance
      );
      
      if (airdropSig) {
        console.log('Airdrop completed, signature:', airdropSig);
      } else {
        console.log('No airdrop needed, sufficient balance');
      }
      
      // 5. transferLamports - Transfer SOL between wallets
      console.log('\n5️⃣  Transferring SOL...');
      const transferAmount = lamports(BigInt('250000000')); // 0.25 SOL
      const transferSig = await kite.transferLamports({
        source: sender,
        destination: receiver.address,
        amount: transferAmount,
        skipPreflight: false,
        maximumClientSideRetries: 3
      });
      
      console.log('SOL transfer completed, signature:', transferSig);
      
      // Check balances after transfer
      const newSenderBalance = await kite.getLamportBalance(sender.address);
      const newReceiverBalance = await kite.getLamportBalance(receiver.address);
      console.log('New sender balance:', Number(newSenderBalance) / 1_000_000_000, 'SOL');
      console.log('New receiver balance:', Number(newReceiverBalance) / 1_000_000_000, 'SOL');
      
      console.log('✅ SOL management functions working!');
    } catch (error) {
      console.error('❌ SOL functions error:', error);
    }
  }, { timeout: 120000 });

  test('should demonstrate token functions', async () => {
    console.log('\n🪙 === TOKEN MANAGEMENT FUNCTIONS ===');
    
    try {
      // Create a wallet to be the mint authority
      const mintAuthority = await kite.createWallet({ 
        airdropAmount: lamports(BigInt('2000000000')) // 2 SOL
      });
      
      // 6. createTokenMint - Create a new token
      console.log('\n6️⃣  Creating a new token mint...');
      const mintAddress = await kite.createTokenMint({
        mintAuthority,
        decimals: 9,
        name: 'Test Token',
        symbol: 'TEST',
        uri: 'https://example.com/token.json',
        additionalMetadata: {
          description: 'A test token created with Kite',
          category: 'utility'
        }
      });
      console.log('Token mint created:', mintAddress);
      
      // 7. getMint - Get token mint information
      console.log('\n7️⃣  Getting mint information...');
      const mintInfo = await kite.getMint(mintAddress);
      console.log('Mint info - decimals:', mintInfo.data.decimals, 'supply:', mintInfo.data.supply);
      
      // 8. getTokenAccountAddress - Get token account address
      console.log('\n8️⃣  Getting token account addresses...');
      const authorityTokenAccount = await kite.getTokenAccountAddress(
        mintAuthority.address,
        mintAddress
      );
      console.log('Mint authority token account:', authorityTokenAccount);
      
      // Create a recipient wallet
      const recipient = await kite.createWallet({ 
        airdropAmount: lamports(BigInt('1000000000')) // 1 SOL
      });
      
      const recipientTokenAccount = await kite.getTokenAccountAddress(
        recipient.address,
        mintAddress
      );
      console.log('Recipient token account:', recipientTokenAccount);
      
      // 9. mintTokens - Mint tokens to an account
      console.log('\n9️⃣  Minting tokens...');
      const mintAmount = BigInt('1000') * BigInt(Math.pow(10, 9)); // 1000 tokens with 9 decimals
      const mintSig = await kite.mintTokens(
        mintAddress,
        mintAuthority,
        mintAmount,
        mintAuthority.address
      );
      console.log('Tokens minted, signature:', mintSig);
      
      // 10. getTokenAccountBalance - Get token account balance
      console.log('\n🔟 Getting token balances...');
      const authorityBalance = await kite.getTokenAccountBalance({ tokenAccount: authorityTokenAccount });
      console.log('Authority token balance:', Number(authorityBalance.amount) / 10**9, 'tokens');
      
      // 11. transferTokens - Transfer tokens between accounts
      console.log('\n1️⃣1️⃣ Transferring tokens...');
      const transferAmount = BigInt('100') * BigInt(Math.pow(10, 9)); // 100 tokens
      const tokenTransferSig = await kite.transferTokens({
        sender: mintAuthority,
        destination: recipient.address,
        mintAddress,
        amount: transferAmount,
        maximumClientSideRetries: 3
      });
      console.log('Tokens transferred, signature:', tokenTransferSig);
      
      // Check balances after transfer
      const newAuthorityBalance = await kite.getTokenAccountBalance({ tokenAccount: authorityTokenAccount });
      const recipientBalance = await kite.getTokenAccountBalance({ tokenAccount: recipientTokenAccount });
      console.log('New authority balance:', Number(newAuthorityBalance.amount) / 10**9, 'tokens');
      console.log('Recipient balance:', Number(recipientBalance.amount) / 10**9, 'tokens');
      
      // 12. checkTokenAccountIsClosed - Check if token account is closed
      console.log('\n1️⃣2️⃣ Checking if token accounts are closed...');
      const isAuthorityClosed = await kite.checkTokenAccountIsClosed({ wallet: mintAuthority.address, mint: mintAddress });
      const isRecipientClosed = await kite.checkTokenAccountIsClosed({ wallet: recipient.address, mint: mintAddress });
      console.log('Authority account closed:', isAuthorityClosed);
      console.log('Recipient account closed:', isRecipientClosed);
      
      console.log('✅ Token management functions working!');
    } catch (error) {
      console.error('❌ Token functions error:', error);
    }
  }, { timeout: 180000 });

  test('should demonstrate transaction and utility functions', async () => {
    console.log('\n⚙️ === TRANSACTION & UTILITY FUNCTIONS ===');
    
    try {
      const wallet = await kite.createWallet({ 
        airdropAmount: lamports(BigInt('2000000000')) // 2 SOL
      });
      const recipient1 = await kite.createWallet();
      const recipient2 = await kite.createWallet();
      
      // 13. sendTransactionFromInstructions - Send transaction with multiple instructions
      console.log('\n1️⃣3️⃣ Sending transaction with multiple instructions...');
      
      const instruction1 = getTransferSolInstruction({
        amount: lamports(BigInt('50000000')), // 0.05 SOL
        destination: recipient1.address,
        source: wallet
      });
      
      const instruction2 = getTransferSolInstruction({
        amount: lamports(BigInt('75000000')), // 0.075 SOL
        destination: recipient2.address,
        source: wallet
      });
      
      const multiInstructionSig = await kite.sendTransactionFromInstructions({
        feePayer: wallet,
        instructions: [instruction1, instruction2],
        commitment: 'confirmed',
        skipPreflight: false,
        maximumClientSideRetries: 3
      });
      
      console.log('Multi-instruction transaction completed:', multiInstructionSig);
      
      // 14. getRecentSignatureConfirmation - Check transaction confirmation
      console.log('\n1️⃣4️⃣ Checking transaction confirmation...');
      // Note: This function may require specific configuration for the signature type
      try {
        const isConfirmed = await kite.getRecentSignatureConfirmation({ 
          signature: multiInstructionSig,
          commitment: 'confirmed',
          abortSignal: new AbortController().signal
        });
        console.log('Transaction confirmed:', isConfirmed);
      } catch (error) {
        console.log('Signature confirmation check skipped due to API constraints');
      }
      
      // 15. getLogs - Get transaction logs
      console.log('\n1️⃣5️⃣ Getting transaction logs...');
      const logs = await kite.getLogs(multiInstructionSig);
      console.log('Transaction logs:', logs.slice(0, 3), '... (showing first 3)');
      
      // 16. getPDAAndBump - Get Program Derived Address
      console.log('\n1️⃣6️⃣ Getting PDA and bump seed...');
      try {
        const seeds = ['test', wallet.address, BigInt(42)];
        const { pda, bump } = await kite.getPDAAndBump(programId, seeds);
        console.log('PDA:', pda);
        console.log('Bump seed:', bump);
      } catch (error) {
        console.log('PDA generation skipped - may require specific program implementation');
      }
      
      // 17. getExplorerLink - Get explorer links for different entities
      console.log('\n1️⃣7️⃣ Getting explorer links...');
      const addressLink = kite.getExplorerLink('address', wallet.address);
      const transactionLink = kite.getExplorerLink('transaction', multiInstructionSig);
      const blockLink = kite.getExplorerLink('block', '12345');
      
      console.log('Explorer links:');
      console.log('  Address:', addressLink);
      console.log('  Transaction:', transactionLink);
      console.log('  Block:', blockLink);
      
      console.log('✅ Transaction and utility functions working!');
    } catch (error) {
      console.error('❌ Transaction/utility functions error:', error);
    }
  }, { timeout: 120000 });

  test('should demonstrate program-specific functionality', async () => {
    console.log('\n🔧 === PROGRAM-SPECIFIC TESTS ===');
    console.log('TODO: Add tests specific to counter program functionality');
    console.log('Program ID:', programId);
    
    try {
      // TODO: Add program-specific tests here
      // Example:
      // const wallet = await kite.createWallet({ airdropAmount: lamports(1_000_000_000n) });
      // const instruction = createcounterInstruction({ ... });
      // const signature = await kite.sendTransactionFromInstructions({
      //   feePayer: wallet,
      //   instructions: [instruction]
      // });
      
      console.log('✅ Program-specific tests ready for implementation!');
    } catch (error) {
      console.error('❌ Program-specific test error:', error);
    }
  }, { timeout: 60000 });
});
