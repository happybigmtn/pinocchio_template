# Craps-Pinocchio Mobile App Implementation Guide

## Overview

This implementation guide provides concrete code examples and architectural patterns for building the Craps-Pinocchio mobile app using React Native, Anza Kit, and Codama-generated clients.

## Project Setup

### 1. Initialize React Native Project

```bash
# Create new React Native project
npx react-native@latest init CrapsPinocchio --template react-native-template-typescript
cd CrapsPinocchio

# Install core dependencies
npm install @solana/kit @helius-dev/kite @solana-mobile/mobile-wallet-adapter-protocol-web3js
npm install @reduxjs/toolkit react-redux redux-persist @react-native-async-storage/async-storage
npm install react-native-paper react-native-vector-icons react-native-haptic-feedback
npm install react-native-svg react-native-reanimated react-native-gesture-handler

# Copy Codama clients
cp -r ../../../clients/crapspinocchio ./src/clients/

# iOS setup
cd ios && pod install && cd ..
```

### 2. Configure Metro for Solana

```javascript
// metro.config.js
const { getDefaultConfig, mergeConfig } = require('@react-native/metro-config');

const config = {
  resolver: {
    extraNodeModules: {
      crypto: require.resolve('react-native-crypto'),
      stream: require.resolve('readable-stream'),
      buffer: require.resolve('buffer'),
    },
    sourceExts: ['jsx', 'js', 'ts', 'tsx', 'cjs', 'json'],
  },
  transformer: {
    getTransformOptions: async () => ({
      transform: {
        experimentalImportSupport: false,
        inlineRequires: true,
      },
    }),
  },
};

module.exports = mergeConfig(getDefaultConfig(__dirname), config);
```

## Core Architecture Implementation

### 1. Service Layer Architecture

```typescript
// src/services/core/ServiceRegistry.ts
import { createSolanaRpc } from '@solana/kit';
import { connect as kiteConnect } from '@helius-dev/kite';
import { WalletService } from './WalletService';
import { CrapsTransactionService } from './CrapsTransactionService';
import { CrapsGameStateService } from './CrapsGameStateService';
import { CrapsTokenService } from './CrapsTokenService';
import { CrapsErrorHandler } from './CrapsErrorHandler';
import { CrapsPDAService } from './CrapsPDAService';
import { CRAPS_PINOCCHIO_PROGRAM_ADDRESS } from '@/clients/crapspinocchio';

export class ServiceRegistry {
  private static instance: ServiceRegistry;
  
  public readonly rpc: ReturnType<typeof createSolanaRpc>;
  public readonly kite: ReturnType<typeof kiteConnect>;
  public readonly wallet: WalletService;
  public readonly transaction: CrapsTransactionService;
  public readonly gameState: CrapsGameStateService;
  public readonly token: CrapsTokenService;
  public readonly errorHandler: CrapsErrorHandler;
  public readonly pda: CrapsPDAService;
  
  // Program constants
  public readonly programId = CRAPS_PINOCCHIO_PROGRAM_ADDRESS;
  
  private constructor() {
    // Initialize RPC with Helius
    const endpoint = `https://mainnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`;
    this.rpc = createSolanaRpc(endpoint);
    this.kite = kiteConnect(endpoint);
    
    // Initialize services in dependency order
    this.errorHandler = new CrapsErrorHandler();
    this.pda = new CrapsPDAService(this.programId);
    this.wallet = new WalletService(this.rpc, this.errorHandler);
    this.transaction = new CrapsTransactionService(this.rpc, this.wallet, this.pda);
    this.gameState = new CrapsGameStateService(this.rpc, this.pda);
    this.token = new CrapsTokenService(this.rpc, this.transaction, this.pda);
  }
  
  static getInstance(): ServiceRegistry {
    if (!ServiceRegistry.instance) {
      ServiceRegistry.instance = new ServiceRegistry();
    }
    return ServiceRegistry.instance;
  }
}

### 2. Wallet Service Implementation

```typescript
// src/services/core/WalletService.ts
import { transact, Web3MobileWallet } from '@solana-mobile/mobile-wallet-adapter-protocol-web3js';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import AsyncStorage from '@react-native-async-storage/async-storage';

export class WalletService {
  private currentSigner: TransactionSigner | null = null;
  private authToken: string | null = null;
  
  async connect(): Promise<TransactionSigner> {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorize(wallet);
      
      this.currentSigner = {
        address: address(auth.accounts[0].address),
        signTransactionMessage: async (message) => {
          return await transact(async (w) => {
            const signed = await w.signTransactions({
              transactions: [message]
            });
            return signed.signedTransactions[0];
          });
        }
      };
      
      return this.currentSigner;
    });
  }
  
  private async authorize(wallet: Web3MobileWallet) {
    // Try reauthorization first
    if (this.authToken) {
      try {
        return await wallet.reauthorize({
          auth_token: this.authToken,
          identity: {
            name: 'Craps Pinocchio',
            uri: 'https://craps-pinocchio.com',
            icon: 'data:image/png;base64,...'
          }
        });
      } catch {
        this.authToken = null;
      }
    }
    
    // Fresh authorization
    const result = await wallet.authorize({
      cluster: 'mainnet-beta',
      identity: {
        name: 'Craps Pinocchio',
        uri: 'https://craps-pinocchio.com',
        icon: 'data:image/png;base64,...'
      }
    });
    
    this.authToken = result.auth_token;
    await AsyncStorage.setItem('mwa_auth_token', result.auth_token);
    
    return result;
  }
  
  async disconnect(): Promise<void> {
    if (this.authToken) {
      await transact(async (wallet) => {
        await wallet.deauthorize({ auth_token: this.authToken! });
      });
      
      this.authToken = null;
      this.currentSigner = null;
      await AsyncStorage.removeItem('mwa_auth_token');
    }
  }
  
  getSigner(): TransactionSigner {
    if (!this.currentSigner) {
      throw new Error('Wallet not connected');
    }
    return this.currentSigner;
  }
}
```

### 2. PDA Service for Address Derivation

```typescript
// src/services/core/CrapsPDAService.ts
import { address, type Address } from '@solana/kit';
import { findProgramAddressSync } from '@solana/keys';

export class CrapsPDAService {
  constructor(private programId: Address) {}
  
  // Core game PDAs
  getGlobalGameStatePDA(): [Address, number] {
    return findProgramAddressSync(
      [Buffer.from('global_game_state')],
      this.programId
    );
  }
  
  getTreasuryPDA(): [Address, number] {
    return findProgramAddressSync(
      [Buffer.from('treasury')],
      this.programId
    );
  }
  
  getRngStatePDA(): [Address, number] {
    return findProgramAddressSync(
      [Buffer.from('rng_state')],
      this.programId
    );
  }
  
  getBonusStatePDA(): [Address, number] {
    return findProgramAddressSync(
      [Buffer.from('bonus_state')],
      this.programId
    );
  }
  
  // Player-specific PDAs
  getPlayerStatePDA(playerAddress: Address): [Address, number] {
    return findProgramAddressSync(
      [
        Buffer.from('scalable_player'),
        address(playerAddress).bytes
      ],
      this.programId
    );
  }
  
  getBetBatchPDA(
    playerAddress: Address, 
    epoch: bigint, 
    batchIndex: number = 0
  ): [Address, number] {
    const epochBytes = new Uint8Array(8);
    new DataView(epochBytes.buffer).setBigUint64(0, epoch, true);
    
    const batchIndexBytes = new Uint8Array(4);
    new DataView(batchIndexBytes.buffer).setUint32(0, batchIndex, true);
    
    return findProgramAddressSync(
      [
        Buffer.from('bet_batch'),
        address(playerAddress).bytes,
        epochBytes,
        batchIndexBytes
      ],
      this.programId
    );
  }
  
  // Epoch-specific PDAs
  getEpochOutcomePDA(epoch: bigint): [Address, number] {
    const epochBytes = new Uint8Array(8);
    new DataView(epochBytes.buffer).setBigUint64(0, epoch, true);
    
    return findProgramAddressSync(
      [
        Buffer.from('epoch_outcome'),
        epochBytes
      ],
      this.programId
    );
  }
  
  // Token-related PDAs
  getTreasuryTokenAccountPDA(tokenMint: Address): [Address, number] {
    const [treasuryPDA] = this.getTreasuryPDA();
    return findProgramAddressSync(
      [
        treasuryPDA.bytes,
        Buffer.from('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'), // Token Program ID
        address(tokenMint).bytes
      ],
      address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL') // Associated Token Program
    );
  }
}
```

### 3. Transaction Service with Codama Clients

```typescript
// src/services/core/CrapsTransactionService.ts
import {
  pipe,
  createTransactionMessage,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  appendTransactionMessageInstructions,
  address,
  type TransactionMessage,
  type IInstruction
} from '@solana/kit';
import {
  getInitializePlayerInstruction,
  getPlaceBetInstruction,
  getDepositV2Instruction,
  getDepositWithAutoClaimV2Instruction,
  getWithdrawV2Instruction,
  getWithdrawWithAutoClaimV2Instruction,
  getClaimEpochPayoutsUnifiedInstruction,
  getCleanupBetBatchInstruction,
  CRAPS_PINOCCHIO_PROGRAM_ADDRESS
} from '@/clients/crapspinocchio';
import { transact } from '@solana-mobile/mobile-wallet-adapter-protocol-web3js';
import { BET_TYPES, CRAP_TOKEN_DECIMALS } from '@/constants/crapsConstants';

// Bet encoding system for the craps program
export class CrapsBetEncoder {
  // Amount encoding ranges (supports 1-100,000 CRAP tokens)
  private static readonly AMOUNT_RANGES = [
    { min: 1, max: 100, increment: 1 },        // 1-100 CRAP (100 values)
    { min: 101, max: 500, increment: 5 },      // 101-500 CRAP (80 values)
    { min: 501, max: 1500, increment: 10 },    // 501-1500 CRAP (100 values)
    { min: 1501, max: 5000, increment: 25 },   // 1501-5000 CRAP (140 values)
    { min: 5001, max: 10000, increment: 50 },  // 5001-10000 CRAP (100 values)
    { min: 10001, max: 20000, increment: 100 }, // 10001-20000 CRAP (100 values)
    { min: 20001, max: 40000, increment: 250 }, // 20001-40000 CRAP (80 values)
    { min: 40001, max: 60000, increment: 500 }, // 40001-60000 CRAP (40 values)
    { min: 60001, max: 80000, increment: 1000 }, // 60001-80000 CRAP (20 values)
    { min: 80001, max: 100000, increment: 2500 } // 80001-100000 CRAP (8 values)
  ];
  
  static encodeAmount(amount: number): number {
    let cumulativeIndex = 0;
    
    for (const range of this.AMOUNT_RANGES) {
      if (amount >= range.min && amount <= range.max) {
        const rangeIndex = Math.floor((amount - range.min) / range.increment);
        return cumulativeIndex + rangeIndex;
      }
      
      cumulativeIndex += Math.floor((range.max - range.min) / range.increment) + 1;
    }
    
    throw new Error(`Invalid bet amount: ${amount}. Must be between 1 and 100,000 CRAP`);
  }
  
  static decodeAmount(index: number): number {
    let cumulativeIndex = 0;
    
    for (const range of this.AMOUNT_RANGES) {
      const rangeSize = Math.floor((range.max - range.min) / range.increment) + 1;
      
      if (index < cumulativeIndex + rangeSize) {
        const rangeIndex = index - cumulativeIndex;
        return range.min + (rangeIndex * range.increment);
      }
      
      cumulativeIndex += rangeSize;
    }
    
    throw new Error(`Invalid amount index: ${index}`);
  }
  
  static encodeBet(betType: number, amount: number): number {
    if (betType < 0 || betType > 63) {
      throw new Error(`Invalid bet type: ${betType}. Must be between 0 and 63`);
    }
    
    const amountIndex = this.encodeAmount(amount);
    
    if (amountIndex > 1023) {
      throw new Error(`Amount index overflow: ${amountIndex}`);
    }
    
    // 16-bit encoding: 6 bits for bet type, 10 bits for amount index
    return (betType << 10) | (amountIndex & 0x3FF);
  }
  
  static decodeBet(encodedBet: number): { betType: number; amount: number } {
    const betType = (encodedBet >> 10) & 0x3F;
    const amountIndex = encodedBet & 0x3FF;
    
    return {
      betType,
      amount: this.decodeAmount(amountIndex)
    };
  }
}

export class CrapsTransactionService {
  constructor(
    private rpc: ReturnType<typeof createSolanaRpc>,
    private walletService: WalletService,
    private pdaService: CrapsPDAService
  ) {}
  
  // Player lifecycle
  async initializePlayer(): Promise<string> {
    const signer = this.walletService.getSigner();
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(signer.address);
    const [globalGameStatePDA] = this.pdaService.getGlobalGameStatePDA();
    
    const instruction = getInitializePlayerInstruction({
      playerState: playerStatePDA,
      player: signer,
      globalGameState: globalGameStatePDA,
      systemProgram: address('11111111111111111111111111111111')
    });
    
    return await this.sendTransaction([instruction], signer);
  }
  
  // Token operations
  async deposit(amount: bigint, autoClaimPrevious: boolean = true): Promise<string> {
    const signer = this.walletService.getSigner();
    const [treasuryPDA] = this.pdaService.getTreasuryPDA();
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(signer.address);
    
    // Get token accounts
    const playerTokenAccount = await this.getAssociatedTokenAddress(
      signer.address,
      process.env.CRAP_TOKEN_MINT!
    );
    const treasuryTokenAccount = await this.getAssociatedTokenAddress(
      treasuryPDA,
      process.env.CRAP_TOKEN_MINT!
    );
    
    const instruction = autoClaimPrevious
      ? getDepositWithAutoClaimV2Instruction({
          treasury: treasuryPDA,
          playerState: playerStatePDA,
          playerTokenAccount,
          treasuryTokenAccount,
          player: signer,
          tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
        })
      : getDepositV2Instruction({
          treasury: treasuryPDA,
          playerState: playerStatePDA,
          playerTokenAccount,
          treasuryTokenAccount,
          player: signer,
          tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
        });
    
    return await this.sendTransaction([instruction], signer);
  }
  
  async withdraw(amount: bigint, autoClaimPrevious: boolean = true): Promise<string> {
    const signer = this.walletService.getSigner();
    const [treasuryPDA] = this.pdaService.getTreasuryPDA();
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(signer.address);
    
    // Get token accounts
    const playerTokenAccount = await this.getAssociatedTokenAddress(
      signer.address,
      process.env.CRAP_TOKEN_MINT!
    );
    const treasuryTokenAccount = await this.getAssociatedTokenAddress(
      treasuryPDA,
      process.env.CRAP_TOKEN_MINT!
    );
    
    const instruction = autoClaimPrevious
      ? getWithdrawWithAutoClaimV2Instruction({
          treasury: treasuryPDA,
          playerState: playerStatePDA,
          treasuryTokenAccount,
          playerTokenAccount,
          player: signer,
          treasuryAuthority: treasuryPDA, // Treasury is its own authority
          tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
        })
      : getWithdrawV2Instruction({
          treasury: treasuryPDA,
          playerState: playerStatePDA,
          treasuryTokenAccount,
          playerTokenAccount,
          player: signer,
          treasuryAuthority: treasuryPDA,
          tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
        });
    
    return await this.sendTransaction([instruction], signer);
  }
  
  // Betting operations
  async placeBet(
    betType: number,
    amount: number,
    currentEpoch: bigint
  ): Promise<string> {
    const signer = this.walletService.getSigner();
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(signer.address);
    const [betBatchPDA] = this.pdaService.getBetBatchPDA(signer.address, currentEpoch);
    const [globalGameStatePDA] = this.pdaService.getGlobalGameStatePDA();
    
    const instruction = getPlaceBetInstruction({
      betBatch: betBatchPDA,
      playerState: playerStatePDA,
      globalGameState: globalGameStatePDA,
      player: signer,
      systemProgram: address('11111111111111111111111111111111')
    });
    
    return await this.sendTransaction([instruction], signer);
  }
  
  async placeBatchBets(
    bets: Array<{ betType: number; amount: number }>,
    currentEpoch: bigint
  ): Promise<string> {
    if (bets.length > 16) {
      throw new Error('Maximum 16 bets per batch');
    }
    
    // Validate and encode bets
    const encodedBets = bets.map(bet => 
      CrapsBetEncoder.encodeBet(bet.betType, bet.amount)
    );
    
    return await this.placeBet(bets[0].betType, bets[0].amount, currentEpoch);
  }
  
  // Payout claiming
  async claimPayouts(epoch: bigint): Promise<string> {
    const signer = this.walletService.getSigner();
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(signer.address);
    const [treasuryPDA] = this.pdaService.getTreasuryPDA();
    const [epochOutcomePDA] = this.pdaService.getEpochOutcomePDA(epoch);
    const [betBatchPDA] = this.pdaService.getBetBatchPDA(signer.address, epoch);
    const [bonusStatePDA] = this.pdaService.getBonusStatePDA();
    
    // Get token accounts
    const playerTokenAccount = await this.getAssociatedTokenAddress(
      signer.address,
      process.env.CRAP_TOKEN_MINT!
    );
    const treasuryTokenAccount = await this.getAssociatedTokenAddress(
      treasuryPDA,
      process.env.CRAP_TOKEN_MINT!
    );
    
    const instruction = getClaimEpochPayoutsUnifiedInstruction({
      player: signer,
      playerState: playerStatePDA,
      treasury: treasuryPDA,
      treasuryTokenAccount,
      playerTokenAccount,
      epochOutcome: epochOutcomePDA,
      betBatch: betBatchPDA,
      bonusState: bonusStatePDA,
      tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
      mint: address(process.env.CRAP_TOKEN_MINT!)
    });
    
    return await this.sendTransaction([instruction], signer);
  }
  
  // Cleanup operations
  async cleanupBetBatch(epoch: bigint): Promise<string> {
    const signer = this.walletService.getSigner();
    const [betBatchPDA] = this.pdaService.getBetBatchPDA(signer.address, epoch);
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(signer.address);
    
    const instruction = getCleanupBetBatchInstruction({
      betBatch: betBatchPDA,
      playerState: playerStatePDA,
      player: signer
    });
    
    return await this.sendTransaction([instruction], signer);
  }
  
  // Core transaction building and sending
  private async sendTransaction(
    instructions: IInstruction[],
    signer: TransactionSigner
  ): Promise<string> {
    return await transact(async (wallet) => {
      // Get latest blockhash
      const { value: latestBlockhash } = await this.rpc
        .getLatestBlockhash({ commitment: 'confirmed' })
        .send();
      
      // Build transaction message
      const message = pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(signer, tx),
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstructions(instructions, tx)
      );
      
      // Sign and send
      const result = await wallet.signAndSendTransactions({
        transactions: [message],
        options: {
          minContextSlot: latestBlockhash.context.slot,
          commitment: 'confirmed'
        }
      });
      
      if (!result.signatures[0]) {
        throw new Error('Transaction failed');
      }
      
      return result.signatures[0];
    });
  }
  
  private async getAssociatedTokenAddress(
    owner: Address,
    mint: string
  ): Promise<Address> {
    // Implement ATA derivation
    const [ata] = findProgramAddressSync(
      [
        address(owner).bytes,
        address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA').bytes,
        address(mint).bytes
      ],
      address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL')
    );
    return ata;
  }
}
```

### 4. Game State Service with Real Craps Program Integration

```typescript
// src/services/core/CrapsGameStateService.ts
import {
  fetchGlobalGameState,
  fetchScalablePlayerState,
  fetchBetBatch,
  fetchEpochOutcome,
  fetchRngState,
  fetchBonusState,
  type GlobalGameState,
  type BetBatch,
  type ScalablePlayerState,
  type EpochOutcome,
  type RngState,
  type BonusState
} from '@/clients/crapspinocchio';
import { CrapsBetEncoder } from './CrapsTransactionService';

export class GameStateService {
  private subscriptions = new Map<string, () => void>();
  
  async subscribeToGameState(
    callback: (state: GlobalGameState) => void
  ): Promise<() => void> {
    const [globalStatePDA] = await this.findProgramAddress([
      Buffer.from('global_game_state')
    ]);
    
    const subscription = this.rpc
      .accountSubscribe(globalStatePDA, {
        commitment: 'confirmed',
        encoding: 'base64'
      })
      .subscribe({
        next: (notification) => {
          const data = Buffer.from(notification.value.data[0], 'base64');
          const serializer = getGlobalGameStateAccountDataSerializer();
          const [gameState] = serializer.deserialize(data);
          callback(gameState);
        },
        error: (err) => {
          console.error('Game state subscription error:', err);
        }
      });
    
    const unsubscribe = () => subscription.unsubscribe();
    this.subscriptions.set('gameState', unsubscribe);
    
    return unsubscribe;
  }
  
  async fetchPlayerState(playerAddress: string): Promise<ScalablePlayerState> {
    const [playerStatePDA] = await this.findProgramAddress([
      Buffer.from('scalable_player'),
      Buffer.from(address(playerAddress))
    ]);
    
    const accountInfo = await this.rpc
      .getAccountInfo(playerStatePDA, { encoding: 'base64' })
      .send();
    
    if (!accountInfo.value?.data) {
      throw new Error('Player not initialized');
    }
    
    const data = Buffer.from(accountInfo.value.data[0], 'base64');
    const serializer = getScalablePlayerStateAccountDataSerializer();
    const [playerState] = serializer.deserialize(data);
    
    return playerState;
  }
  
  async fetchActiveBets(
    playerAddress: string,
    epoch: number
  ): Promise<BetBatch | null> {
    const [betBatchPDA] = await this.findProgramAddress([
      Buffer.from('bet_batch'),
      Buffer.from(address(playerAddress)),
      Buffer.from(new Uint8Array(new BigUint64Array([BigInt(epoch)]).buffer))
    ]);
    
    try {
      const accountInfo = await this.rpc
        .getAccountInfo(betBatchPDA, { encoding: 'base64' })
        .send();
      
      if (!accountInfo.value?.data) {
        return null;
      }
      
      const data = Buffer.from(accountInfo.value.data[0], 'base64');
      const serializer = getBetBatchAccountDataSerializer();
      const [betBatch] = serializer.deserialize(data);
      
      return betBatch;
    } catch {
      return null;
    }
  }
  
  decodeBets(encodedBets: number[]): Array<{ type: number; amount: number }> {
    const amounts = [1, 5, 10, 25, 50, 100, 250, 500, 1000, 2500, 5000, 10000];
    
    return encodedBets
      .filter(bet => bet !== 0)
      .map(encodedBet => {
        const betType = (encodedBet >> 10) & 0x3F;
        const amountIndex = encodedBet & 0x3FF;
        return {
          type: betType,
          amount: amounts[amountIndex] || 0
        };
      });
  }
}
```

### 5. Redux Store Setup

```typescript
// src/state/store.ts
import { configureStore } from '@reduxjs/toolkit';
import {
  persistStore,
  persistReducer,
  FLUSH,
  REHYDRATE,
  PAUSE,
  PERSIST,
  PURGE,
  REGISTER
} from 'redux-persist';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { gameSlice } from './slices/gameSlice';
import { walletSlice } from './slices/walletSlice';
import { uiSlice } from './slices/uiSlice';

const persistConfig = {
  key: 'craps-pinocchio',
  storage: AsyncStorage,
  whitelist: ['wallet'], // Only persist wallet data
  blacklist: ['game', 'ui'] // Don't persist volatile data
};

const rootReducer = {
  game: gameSlice.reducer,
  wallet: persistReducer(persistConfig, walletSlice.reducer),
  ui: uiSlice.reducer
};

export const store = configureStore({
  reducer: rootReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
        ignoredPaths: ['wallet.signer']
      }
    })
});

export const persistor = persistStore(store);
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
```

### 6. Game Slice Implementation

```typescript
// src/state/slices/gameSlice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type { GlobalGameState, BetBatch } from '@/clients/crapspinocchio';

interface GameState {
  epoch: number;
  phase: 'come_out' | 'point' | 'ended';
  point: number;
  dice: {
    die1: number;
    die2: number;
    total: number;
  };
  activeBets: Array<{
    id: string;
    type: number;
    amount: number;
    status: 'pending' | 'confirmed' | 'won' | 'lost';
    signature?: string;
  }>;
  balance: bigint;
  lastUpdate: number;
}

const initialState: GameState = {
  epoch: 0,
  phase: 'come_out',
  point: 0,
  dice: { die1: 0, die2: 0, total: 0 },
  activeBets: [],
  balance: 0n,
  lastUpdate: Date.now()
};

export const gameSlice = createSlice({
  name: 'game',
  initialState,
  reducers: {
    updateFromChain: (state, action: PayloadAction<GlobalGameState>) => {
      const gameState = action.payload;
      state.epoch = gameState.epoch;
      state.phase = gameState.phase === 0 ? 'come_out' : 
                    gameState.phase === 1 ? 'point' : 'ended';
      state.point = gameState.point;
      state.dice = {
        die1: gameState.die1,
        die2: gameState.die2,
        total: gameState.die1 + gameState.die2
      };
      state.lastUpdate = Date.now();
    },
    
    placeBetOptimistic: (state, action: PayloadAction<{
      id: string;
      type: number;
      amount: number;
    }>) => {
      state.activeBets.push({
        ...action.payload,
        status: 'pending'
      });
      state.balance -= BigInt(action.payload.amount);
    },
    
    confirmBet: (state, action: PayloadAction<{
      id: string;
      signature: string;
    }>) => {
      const bet = state.activeBets.find(b => b.id === action.payload.id);
      if (bet) {
        bet.status = 'confirmed';
        bet.signature = action.payload.signature;
      }
    },
    
    revertBet: (state, action: PayloadAction<{ id: string }>) => {
      const betIndex = state.activeBets.findIndex(b => b.id === action.payload.id);
      if (betIndex !== -1) {
        const bet = state.activeBets[betIndex];
        if (bet.status === 'pending') {
          state.balance += BigInt(bet.amount);
          state.activeBets.splice(betIndex, 1);
        }
      }
    },
    
    updateBalance: (state, action: PayloadAction<bigint>) => {
      state.balance = action.payload;
    },
    
    clearCompletedBets: (state) => {
      state.activeBets = state.activeBets.filter(
        bet => bet.status === 'pending' || bet.status === 'confirmed'
      );
    }
  }
});

export const {
  updateFromChain,
  placeBetOptimistic,
  confirmBet,
  revertBet,
  updateBalance,
  clearCompletedBets
} = gameSlice.actions;
```

## UI Component Implementation

### 1. Main Game Screen

```typescript
// src/screens/GameScreen.tsx
import React, { useEffect, useCallback } from 'react';
import {
  View,
  StyleSheet,
  ScrollView,
  StatusBar
} from 'react-native';
import { useAppDispatch, useAppSelector } from '@/hooks/redux';
import { CrapsTable } from '@/components/game/CrapsTable';
import { DiceDisplay } from '@/components/game/DiceDisplay';
import { BettingControls } from '@/components/game/BettingControls';
import { GameInfo } from '@/components/game/GameInfo';
import { useGameSubscription } from '@/hooks/useGameSubscription';
import { usePlaceBet } from '@/hooks/usePlaceBet';

export const GameScreen: React.FC = () => {
  const dispatch = useAppDispatch();
  const gameState = useAppSelector(state => state.game);
  const { subscribeToGame } = useGameSubscription();
  const { placeBet, isPlacing } = usePlaceBet();
  
  useEffect(() => {
    const unsubscribe = subscribeToGame();
    return () => unsubscribe();
  }, []);
  
  const handleBetPlacement = useCallback(async (
    betType: number,
    amount: number
  ) => {
    await placeBet(betType, amount);
  }, [placeBet]);
  
  return (
    <View style={styles.container}>
      <StatusBar backgroundColor="#0B5D1E" barStyle="light-content" />
      
      <GameInfo
        epoch={gameState.epoch}
        phase={gameState.phase}
        point={gameState.point}
        balance={gameState.balance}
      />
      
      <DiceDisplay
        die1={gameState.dice.die1}
        die2={gameState.dice.die2}
        isRolling={false}
      />
      
      <ScrollView style={styles.tableContainer}>
        <CrapsTable
          onBetSelect={handleBetPlacement}
          activeBets={gameState.activeBets}
          phase={gameState.phase}
          point={gameState.point}
          disabled={isPlacing}
        />
      </ScrollView>
      
      <BettingControls
        selectedAmount={100}
        onAmountChange={() => {}}
        onPlaceBet={() => {}}
        disabled={isPlacing}
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#0B5D1E'
  },
  tableContainer: {
    flex: 1
  }
});
```

### 2. Craps Table Component

```typescript
// src/components/game/CrapsTable.tsx
import React, { useCallback } from 'react';
import {
  View,
  TouchableOpacity,
  Text,
  StyleSheet,
  Dimensions
} from 'react-native';
import Svg, { Rect, Text as SvgText, G } from 'react-native-svg';
import { BET_TYPES } from '@/constants/betTypes';

const { width: SCREEN_WIDTH } = Dimensions.get('window');
const TABLE_WIDTH = SCREEN_WIDTH - 20;
const TABLE_HEIGHT = TABLE_WIDTH * 1.5;

interface Props {
  onBetSelect: (betType: number, amount: number) => void;
  activeBets: Array<{ type: number; amount: number }>;
  phase: string;
  point: number;
  disabled: boolean;
}

export const CrapsTable: React.FC<Props> = ({
  onBetSelect,
  activeBets,
  phase,
  point,
  disabled
}) => {
  const renderBettingArea = useCallback((
    betType: number,
    x: number,
    y: number,
    width: number,
    height: number,
    label: string
  ) => {
    const isActive = activeBets.some(bet => bet.type === betType);
    const betAmount = activeBets
      .filter(bet => bet.type === betType)
      .reduce((sum, bet) => sum + bet.amount, 0);
    
    return (
      <G key={betType}>
        <Rect
          x={x}
          y={y}
          width={width}
          height={height}
          fill={isActive ? '#FFD700' : '#2E7D32'}
          stroke="#FFFFFF"
          strokeWidth={2}
          opacity={disabled ? 0.5 : 1}
          onPress={() => !disabled && onBetSelect(betType, 100)}
        />
        <SvgText
          x={x + width / 2}
          y={y + height / 2 - 10}
          fill="#FFFFFF"
          fontSize={14}
          fontWeight="bold"
          textAnchor="middle"
        >
          {label}
        </SvgText>
        {isActive && (
          <SvgText
            x={x + width / 2}
            y={y + height / 2 + 10}
            fill="#FFFFFF"
            fontSize={12}
            textAnchor="middle"
          >
            ${betAmount}
          </SvgText>
        )}
      </G>
    );
  }, [activeBets, disabled, onBetSelect]);
  
  return (
    <View style={styles.container}>
      <Svg width={TABLE_WIDTH} height={TABLE_HEIGHT}>
        {/* Pass Line */}
        {renderBettingArea(
          BET_TYPES.PASS_LINE,
          10,
          TABLE_HEIGHT - 60,
          TABLE_WIDTH - 20,
          50,
          'PASS LINE'
        )}
        
        {/* Don't Pass */}
        {renderBettingArea(
          BET_TYPES.DONT_PASS,
          10,
          TABLE_HEIGHT - 120,
          TABLE_WIDTH - 20,
          50,
          "DON'T PASS"
        )}
        
        {/* Field */}
        {renderBettingArea(
          BET_TYPES.FIELD,
          10,
          TABLE_HEIGHT / 2 - 30,
          TABLE_WIDTH - 20,
          60,
          'FIELD'
        )}
        
        {/* Place Bets */}
        {[4, 5, 6, 8, 9, 10].map((number, index) => {
          const betType = BET_TYPES[`PLACE_${number}`];
          const x = 10 + (index * (TABLE_WIDTH - 20) / 6);
          return renderBettingArea(
            betType,
            x,
            100,
            (TABLE_WIDTH - 20) / 6 - 5,
            40,
            `${number}`
          );
        })}
        
        {/* Come */}
        {renderBettingArea(
          BET_TYPES.COME,
          10,
          200,
          (TABLE_WIDTH - 20) / 2 - 5,
          50,
          'COME'
        )}
        
        {/* Don't Come */}
        {renderBettingArea(
          BET_TYPES.DONT_COME,
          TABLE_WIDTH / 2 + 5,
          200,
          (TABLE_WIDTH - 20) / 2 - 5,
          50,
          "DON'T COME"
        )}
      </Svg>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    padding: 10
  }
});
```

### 3. Custom Hooks

```typescript
// src/hooks/usePlaceBet.ts
import { useCallback, useState } from 'react';
import { useAppDispatch, useAppSelector } from './redux';
import { ServiceRegistry } from '@/services/core/ServiceRegistry';
import {
  placeBetOptimistic,
  confirmBet,
  revertBet
} from '@/state/slices/gameSlice';
import { showNotification } from '@/state/slices/uiSlice';

export const usePlaceBet = () => {
  const dispatch = useAppDispatch();
  const gameState = useAppSelector(state => state.game);
  const [isPlacing, setIsPlacing] = useState(false);
  
  const placeBet = useCallback(async (
    betType: number,
    amount: number
  ) => {
    const betId = crypto.randomUUID();
    
    try {
      setIsPlacing(true);
      
      // Optimistic update
      dispatch(placeBetOptimistic({ id: betId, type: betType, amount }));
      
      // Place bet on chain
      const services = ServiceRegistry.getInstance();
      const signature = await services.transaction.placeBet(
        betType,
        amount,
        gameState.epoch
      );
      
      // Confirm bet
      dispatch(confirmBet({ id: betId, signature }));
      
      // Show success
      dispatch(showNotification({
        type: 'success',
        message: 'Bet placed successfully!',
        duration: 3000
      }));
      
    } catch (error) {
      // Revert optimistic update
      dispatch(revertBet({ id: betId }));
      
      // Handle error
      const services = ServiceRegistry.getInstance();
      const userError = services.errorHandler.parseError(error);
      
      dispatch(showNotification({
        type: 'error',
        message: userError.message,
        duration: 5000
      }));
    } finally {
      setIsPlacing(false);
    }
  }, [dispatch, gameState.epoch]);
  
  return { placeBet, isPlacing };
};
```

```typescript
// src/hooks/useGameSubscription.ts
import { useEffect, useCallback } from 'react';
import { useAppDispatch } from './redux';
import { ServiceRegistry } from '@/services/core/ServiceRegistry';
import { updateFromChain, updateBalance } from '@/state/slices/gameSlice';

export const useGameSubscription = () => {
  const dispatch = useAppDispatch();
  
  const subscribeToGame = useCallback(() => {
    const services = ServiceRegistry.getInstance();
    let unsubscribeGame: (() => void) | null = null;
    let unsubscribePlayer: (() => void) | null = null;
    
    const setup = async () => {
      // Subscribe to global game state
      unsubscribeGame = await services.gameState.subscribeToGameState(
        (gameState) => {
          dispatch(updateFromChain(gameState));
        }
      );
      
      // Subscribe to player state if connected
      if (services.wallet.isConnected()) {
        const playerAddress = services.wallet.getSigner().address;
        const playerState = await services.gameState.fetchPlayerState(
          playerAddress.toString()
        );
        dispatch(updateBalance(playerState.balance));
      }
    };
    
    setup().catch(console.error);
    
    // Return cleanup function
    return () => {
      unsubscribeGame?.();
      unsubscribePlayer?.();
    };
  }, [dispatch]);
  
  return { subscribeToGame };
};
```

## Token Integration

### 1. Token Creation Service

```typescript
// src/services/token/TokenCreationService.ts
import { TokenMillService } from '@/modules/token-mill/services/tokenMillService';
import { PumpfunService } from '@/modules/pump-fun/services/pumpfunService';
import { RaydiumService } from '@/modules/raydium/services/raydiumService';

export class TokenCreationService {
  private tokenMill: TokenMillService;
  private pumpfun: PumpfunService;
  private raydium: RaydiumService;
  
  async deployCRAPToken(treasuryWallet: TransactionSigner) {
    // Create main game token
    const crapToken = await this.tokenMill.createToken({
      name: "Craps Casino Token",
      symbol: "CRAP",
      decimals: 9,
      totalSupply: 1_000_000_000_000, // 1 trillion
      metadata: {
        description: "Official token for Craps-Pinocchio casino",
        image: "https://arweave.net/crap-token-logo",
        extensions: {
          website: "https://craps-pinocchio.com",
          twitter: "@crapscasino"
        }
      }
    });
    
    // Create liquidity pool
    const pool = await this.raydium.createCLMM({
      tokenA: crapToken.mint,
      tokenB: "So11111111111111111111111111111111111111112", // SOL
      binStep: 10, // 0.1% bins
      initialPrice: 0.001, // 1 CRAP = 0.001 SOL
      activeBin: 0
    });
    
    return { token: crapToken, pool };
  }
  
  async deployDICEToken() {
    // Create bonus token on pump.fun
    const diceToken = await this.pumpfun.createToken({
      name: "Lucky Dice",
      symbol: "DICE",
      description: "Bonus rewards for epic wins!",
      imageUri: "ipfs://dice-token-image",
      twitter: "@luckydicetoken",
      telegram: "t.me/luckydice",
      website: "https://luckydice.casino"
    });
    
    // Initial buy to set price
    await this.pumpfun.buyToken({
      mint: diceToken.mint,
      amount: 5, // 5 SOL
      slippage: 0.5 // 0.5%
    });
    
    return diceToken;
  }
}
```

## Error Handling

### 1. Comprehensive Error Handler

```typescript
// src/services/core/ErrorHandler.ts
export interface UserError {
  title: string;
  message: string;
  action: 'RETRY' | 'DISMISS' | 'SHOW_DEPOSIT' | 'ADJUST_BET' | 'RETRY_LATER';
  details?: any;
}

export class ErrorHandler {
  parseError(error: any): UserError {
    console.log('Parsing error:', error);
    
    // Transaction errors
    if (error.message?.includes('Transaction cancelled')) {
      return {
        title: 'Transaction Cancelled',
        message: 'You cancelled the transaction',
        action: 'DISMISS'
      };
    }
    
    // Parse program errors from logs
    if (error.logs && Array.isArray(error.logs)) {
      const errorLog = error.logs.find((log: string) =>
        log.includes('Error:') || 
        log.includes('failed') ||
        log.includes('InstructionError')
      );
      
      if (errorLog) {
        return this.parseProgramError(errorLog);
      }
    }
    
    // Network errors
    if (error.message?.includes('Network request failed') ||
        error.message?.includes('fetch failed')) {
      return {
        title: 'Connection Error',
        message: 'Please check your internet connection',
        action: 'RETRY'
      };
    }
    
    // RPC errors
    if (error.message?.includes('429') || 
        error.message?.includes('rate limit')) {
      return {
        title: 'Too Many Requests',
        message: 'Please wait a moment and try again',
        action: 'RETRY_LATER'
      };
    }
    
    // Default
    return {
      title: 'Transaction Failed',
      message: error.message || 'Something went wrong',
      action: 'RETRY',
      details: error
    };
  }
  
  private parseProgramError(errorLog: string): UserError {
    // Pinocchio custom errors
    if (errorLog.includes('InvalidBetAmount')) {
      return {
        title: 'Invalid Bet Amount',
        message: 'Bet must be between 1 and 100,000 CRAP',
        action: 'ADJUST_BET'
      };
    }
    
    if (errorLog.includes('InsufficientBalance')) {
      return {
        title: 'Insufficient Balance',
        message: 'Not enough CRAP tokens to place this bet',
        action: 'SHOW_DEPOSIT'
      };
    }
    
    if (errorLog.includes('BettingClosed')) {
      return {
        title: 'Betting Closed',
        message: 'Wait for the next round to place bets',
        action: 'RETRY_LATER'
      };
    }
    
    if (errorLog.includes('GamePaused')) {
      return {
        title: 'Game Paused',
        message: 'The game is temporarily unavailable',
        action: 'RETRY_LATER'
      };
    }
    
    if (errorLog.includes('InvalidBetType')) {
      return {
        title: 'Invalid Bet',
        message: 'This bet type is not available in the current phase',
        action: 'DISMISS'
      };
    }
    
    // Generic program error
    return {
      title: 'Transaction Failed',
      message: 'The program rejected this transaction',
      action: 'RETRY',
      details: errorLog
    };
  }
}
```

## Build and Deploy

### 1. Production Build Configuration

```javascript
// android/app/build.gradle
android {
  defaultConfig {
    minSdkVersion 21
    targetSdkVersion 33
  }
  
  signingConfigs {
    release {
      storeFile file(MYAPP_RELEASE_STORE_FILE)
      storePassword MYAPP_RELEASE_STORE_PASSWORD
      keyAlias MYAPP_RELEASE_KEY_ALIAS
      keyPassword MYAPP_RELEASE_KEY_PASSWORD
    }
  }
  
  buildTypes {
    release {
      signingConfig signingConfigs.release
      minifyEnabled true
      proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
    }
  }
}
```

### 2. Environment Configuration

```typescript
// src/config/environment.ts
interface Environment {
  RPC_ENDPOINT: string;
  HELIUS_API_KEY: string;
  PROGRAM_ID: string;
  CRAP_TOKEN_MINT: string;
  DICE_TOKEN_MINT: string;
  TREASURY_ADDRESS: string;
}

const environments: Record<string, Environment> = {
  development: {
    RPC_ENDPOINT: 'https://api.devnet.solana.com',
    HELIUS_API_KEY: process.env.HELIUS_API_KEY_DEV!,
    PROGRAM_ID: 'DevCrapsProgramID...',
    CRAP_TOKEN_MINT: 'DevCrapTokenMint...',
    DICE_TOKEN_MINT: 'DevDiceTokenMint...',
    TREASURY_ADDRESS: 'DevTreasuryAddress...'
  },
  production: {
    RPC_ENDPOINT: `https://mainnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`,
    HELIUS_API_KEY: process.env.HELIUS_API_KEY!,
    PROGRAM_ID: CRAPS_PINOCCHIO_PROGRAM_ADDRESS,
    CRAP_TOKEN_MINT: 'CrapTokenMintAddress...',
    DICE_TOKEN_MINT: 'DiceTokenMintAddress...',
    TREASURY_ADDRESS: 'TreasuryAddress...'
  }
};

export const config = environments[process.env.NODE_ENV || 'development'];
```

## Testing Strategy

### 1. Unit Tests

```typescript
// src/services/__tests__/TransactionService.test.ts
import { TransactionService } from '../core/TransactionService';
import { mockRpc, mockWallet } from '@/test/mocks';

describe('TransactionService', () => {
  let service: TransactionService;
  
  beforeEach(() => {
    service = new TransactionService(mockRpc, mockWallet);
  });
  
  it('should encode bets correctly', () => {
    const betType = 1; // Pass Line
    const amountIndex = 5; // 100 CRAP
    const encoded = service['encodeBet'](betType, amountIndex);
    
    expect(encoded).toBe((1 << 10) | 5); // 0000010000000101
  });
  
  it('should build place bet instruction', async () => {
    const instruction = await service['buildPlaceBetInstruction'](
      mockWallet.address,
      1, // Pass Line
      100, // 100 CRAP
      12345 // Epoch
    );
    
    expect(instruction.programAddress).toBe(CRAPS_PINOCCHIO_PROGRAM_ADDRESS);
    expect(instruction.accounts).toHaveLength(5);
  });
});
```

### 2. Integration Tests

```typescript
// src/screens/__tests__/GameScreen.integration.test.tsx
import React from 'react';
import { render, fireEvent, waitFor } from '@testing-library/react-native';
import { Provider } from 'react-redux';
import { GameScreen } from '../GameScreen';
import { setupStore } from '@/test/utils';

describe('GameScreen Integration', () => {
  it('should place bet optimistically and confirm', async () => {
    const { store } = setupStore();
    const { getByText, getByTestId } = render(
      <Provider store={store}>
        <GameScreen />
      </Provider>
    );
    
    // Select Pass Line bet
    fireEvent.press(getByText('PASS LINE'));
    
    // Place bet
    fireEvent.press(getByTestId('place-bet-button'));
    
    // Check optimistic update
    expect(store.getState().game.activeBets).toHaveLength(1);
    expect(store.getState().game.activeBets[0].status).toBe('pending');
    
    // Wait for confirmation
    await waitFor(() => {
      expect(store.getState().game.activeBets[0].status).toBe('confirmed');
    });
  });
});
```

## Performance Optimization

### 1. Bundle Size Optimization

```javascript
// metro.config.js - Additional optimization
module.exports = {
  transformer: {
    minifierPath: 'metro-minify-terser',
    minifierConfig: {
      keep_fnames: true,
      mangle: {
        keep_fnames: true,
      },
    },
  },
  resolver: {
    // Exclude unnecessary node modules
    blacklistRE: /node_modules\/.*\/node_modules\/(react-native|@solana\/web3\.js)\/.*/,
  },
};
```

### 2. Memory Management

```typescript
// src/hooks/useCleanup.ts
import { useEffect, useRef } from 'react';
import { useAppDispatch } from './redux';
import { clearCompletedBets } from '@/state/slices/gameSlice';

export const useCleanup = () => {
  const dispatch = useAppDispatch();
  const cleanupInterval = useRef<NodeJS.Timeout>();
  
  useEffect(() => {
    // Clean up completed bets every 5 minutes
    cleanupInterval.current = setInterval(() => {
      dispatch(clearCompletedBets());
    }, 5 * 60 * 1000);
    
    return () => {
      if (cleanupInterval.current) {
        clearInterval(cleanupInterval.current);
      }
    };
  }, [dispatch]);
};
```

## Conclusion

This implementation guide provides a complete foundation for building the Craps-Pinocchio mobile app. The architecture leverages:

1. **Anza Kit** for efficient blockchain operations
2. **Codama-generated clients** for type-safe program interaction
3. **Mobile Wallet Adapter** for seamless wallet integration
4. **Redux Toolkit** for predictable state management
5. **Comprehensive error handling** for better UX
6. **Token integration** with multiple DEX options
7. **Performance optimizations** for mobile networks

The modular structure allows for easy extension and maintenance while providing a solid foundation for a production-ready gaming application.