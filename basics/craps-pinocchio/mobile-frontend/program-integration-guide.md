# Craps-Pinocchio Program Integration Guide

## Overview

This guide provides detailed instructions for integrating the mobile frontend with the actual craps-pinocchio program at `2yxPAyKVGMz6trfp8caRMWCoY5psMq6H1r4cuynLrvoX`. It covers all 32 instructions, account structures, and proper usage patterns.

## Program Architecture Deep Dive

### Account Structure Analysis

Based on the IDL at `/home/r/Coding/pinocchio_template/idl/craps_pinocchio.json`, the program uses these core accounts:

```typescript
// Account sizes and purposes
const ACCOUNT_SIZES = {
  GlobalGameState: 192,    // Main game state
  ScalablePlayerState: 128, // Player data
  BetBatch: 296,           // Packed bet storage (16 bets max)
  Treasury: 152,           // Token vault
  EpochOutcome: 64,        // Dice results per epoch
  RngState: 384,           // Random number generation
  BonusState: 40,          // Bonus bet tracking
  Favorites: 267           // Player preferences
} as const;
```

### Complete Instruction Set

The program provides 32 instructions (discriminants 0-31):

```typescript
// All available instructions from the IDL
export const INSTRUCTIONS = {
  // System setup (0-2)
  InitializeSystem: 0,
  InitializeCriticalPDAs: 1,
  InitializePlayer: 2,
  
  // Player management (3)
  ClosePlayerAccount: 3,
  
  // Token operations (4-7)
  DepositV2: 4,
  WithdrawV2: 5,
  DepositWithAutoClaimV2: 6,
  WithdrawWithAutoClaimV2: 7,
  
  // Betting (8)
  PlaceBet: 8,
  
  // Game flow (9-13)
  SecureAutoRoll: 9,
  CollectBlockHash: 10,
  FinalizeRng: 11,
  StartBettingPhase: 12,
  SettleRealizableBets: 13,
  
  // Payouts (14)
  ClaimEpochPayoutsUnified: 14,
  
  // Cleanup (15-17)
  CleanupBetBatch: 15,
  CleanupOldBetBatch: 16,
  CleanupOldEpochOutcome: 17,
  
  // Authority management (18-26)
  UpdateAuthority: 18,
  UpdateRngAuthority: 19,
  UpdateAdminAuthority: 20,
  UpdateEmergencyAuthority: 21,
  ExecuteAuthorityTransfer: 22,
  EmergencyShutdown: 23,
  ResumeOperations: 24,
  EmergencyPause: 25,
  ResumeGame: 26,
  EnableSecureRng: 27,
  
  // Tournament integration (28-29)
  UpdatePlayerTournament: 28,
  ClearPlayerTournament: 29,
  
  // Treasury management (30-31)
  UpdateTreasuryAuthority: 30,
  UpdateTreasuryParameters: 31
} as const;
```

## Critical Integration Points

### 1. Account Initialization Sequence

Players must initialize their account before any other operations:

```typescript
// src/services/initialization/PlayerInitService.ts
import { 
  getInitializePlayerInstruction,
  fetchScalablePlayerState 
} from '@/clients/crapspinocchio';

export class PlayerInitService {
  async ensurePlayerInitialized(playerAddress: Address): Promise<boolean> {
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(playerAddress);
    
    try {
      // Check if player account exists
      await fetchScalablePlayerState(this.rpc, playerStatePDA);
      return true;
    } catch (error) {
      // Account doesn't exist, need to initialize
      await this.initializePlayer(playerAddress);
      return false;
    }
  }
  
  private async initializePlayer(playerAddress: Address): Promise<string> {
    const [playerStatePDA] = this.pdaService.getPlayerStatePDA(playerAddress);
    const [globalGameStatePDA] = this.pdaService.getGlobalGameStatePDA();
    
    const instruction = getInitializePlayerInstruction({
      playerState: playerStatePDA,
      player: this.wallet.getSigner(),
      globalGameState: globalGameStatePDA,
      systemProgram: PROGRAM_CONSTANTS.SYSTEM_PROGRAM
    });
    
    return await this.sendTransaction([instruction]);
  }
}
```

### 2. Proper Bet Encoding Implementation

The program uses sophisticated bet encoding that must be implemented correctly:

```typescript
// src/services/betting/BetEncodingService.ts
export class BetEncodingService {
  // Exact amount encoding from the program
  private static readonly AMOUNT_ENCODING_TABLE = [
    // Range 1-100: Direct mapping (100 values, indices 0-99)
    ...Array.from({ length: 100 }, (_, i) => i + 1),
    
    // Range 101-500: 5 CRAP increments (80 values, indices 100-179)  
    ...Array.from({ length: 80 }, (_, i) => 101 + i * 5),
    
    // Range 501-1500: 10 CRAP increments (100 values, indices 180-279)
    ...Array.from({ length: 100 }, (_, i) => 501 + i * 10),
    
    // Range 1501-5000: 25 CRAP increments (140 values, indices 280-419)
    ...Array.from({ length: 140 }, (_, i) => 1501 + i * 25),
    
    // Range 5001-10000: 50 CRAP increments (100 values, indices 420-519)
    ...Array.from({ length: 100 }, (_, i) => 5001 + i * 50),
    
    // Range 10001-20000: 100 CRAP increments (100 values, indices 520-619)
    ...Array.from({ length: 100 }, (_, i) => 10001 + i * 100),
    
    // Range 20001-40000: 250 CRAP increments (80 values, indices 620-699)
    ...Array.from({ length: 80 }, (_, i) => 20001 + i * 250),
    
    // Range 40001-60000: 500 CRAP increments (40 values, indices 700-739)
    ...Array.from({ length: 40 }, (_, i) => 40001 + i * 500),
    
    // Range 60001-80000: 1000 CRAP increments (20 values, indices 740-759)
    ...Array.from({ length: 20 }, (_, i) => 60001 + i * 1000),
    
    // Range 80001-100000: 2500 CRAP increments (8 values, indices 760-767)
    ...Array.from({ length: 8 }, (_, i) => 80001 + i * 2500)
  ];
  
  static encodeAmount(amount: number): number {
    const index = this.AMOUNT_ENCODING_TABLE.findIndex(val => val === amount);
    if (index === -1) {
      throw new Error(`Invalid bet amount: ${amount}. Must match encoding table values.`);
    }
    return index;
  }
  
  static decodeAmount(index: number): number {
    if (index < 0 || index >= this.AMOUNT_ENCODING_TABLE.length) {
      throw new Error(`Invalid amount index: ${index}`);
    }
    return this.AMOUNT_ENCODING_TABLE[index];
  }
  
  static getValidAmounts(): number[] {
    return [...this.AMOUNT_ENCODING_TABLE];
  }
  
  static findClosestValidAmount(desiredAmount: number): number {
    let closest = this.AMOUNT_ENCODING_TABLE[0];
    let minDiff = Math.abs(desiredAmount - closest);
    
    for (const amount of this.AMOUNT_ENCODING_TABLE) {
      const diff = Math.abs(desiredAmount - amount);
      if (diff < minDiff) {
        minDiff = diff;
        closest = amount;
      }
    }
    
    return closest;
  }
  
  // Full bet encoding (16-bit)
  static encodeBet(betType: number, amount: number): number {
    if (betType < 0 || betType > 63) {
      throw new Error(`Invalid bet type: ${betType}. Must be 0-63.`);
    }
    
    const amountIndex = this.encodeAmount(amount);
    
    if (amountIndex > 1023) {
      throw new Error(`Amount index overflow: ${amountIndex}`);
    }
    
    // 16-bit encoding: 6 bits for bet type, 10 bits for amount index
    return (betType << 10) | (amountIndex & 0x3FF);
  }
  
  static decodeBet(encodedBet: number): { betType: number; amount: number } {
    const betType = (encodedBet >> 10) & 0x3F; // Extract top 6 bits
    const amountIndex = encodedBet & 0x3FF;     // Extract bottom 10 bits
    
    return {
      betType,
      amount: this.decodeAmount(amountIndex)
    };
  }
}
```

### 3. BetBatch Account Management

The BetBatch account can hold up to 16 bets and uses bit masks for state tracking:

```typescript
// src/services/betting/BetBatchService.ts
import { fetchBetBatch, type BetBatch } from '@/clients/crapspinocchio';

export class BetBatchService {
  async loadPlayerBets(
    playerAddress: Address, 
    epoch: bigint
  ): Promise<DecodedBetBatch | null> {
    const [betBatchPDA] = this.pdaService.getBetBatchPDA(playerAddress, epoch);
    
    try {
      const betBatchAccount = await fetchBetBatch(this.rpc, betBatchPDA);
      return this.decodeBetBatch(betBatchAccount.data);
    } catch (error) {
      return null; // No bets for this epoch
    }
  }
  
  private decodeBetBatch(betBatch: BetBatch): DecodedBetBatch {
    const bets: DecodedBet[] = [];
    
    // Extract packed bets (32 bytes = 16 bets * 2 bytes each)
    for (let i = 0; i < betBatch.betCount; i++) {
      const byteIndex = i * 2;
      const encodedBet = (betBatch.packedBets[byteIndex] << 8) | 
                        betBatch.packedBets[byteIndex + 1];
      
      if (encodedBet === 0) continue; // Skip empty slots
      
      const { betType, amount } = BetEncodingService.decodeBet(encodedBet);
      
      // Check bit masks for bet status
      const bitPosition = i;
      const byteIndex2 = Math.floor(bitPosition / 8);
      const bitMask = 1 << (bitPosition % 8);
      
      const isResolved = (betBatch.resolvedMask[byteIndex2] & bitMask) !== 0;
      const isRealizable = (betBatch.realizableMask[byteIndex2] & bitMask) !== 0;
      const isSettled = (betBatch.settledMask[byteIndex2] & bitMask) !== 0;
      const isWinning = (betBatch.winningMask[byteIndex2] & bitMask) !== 0;
      
      // Extract individual payout (8 bytes per bet)
      const payoutStartIndex = i * 8;
      const payoutBytes = betBatch.individualPayouts.slice(
        payoutStartIndex, 
        payoutStartIndex + 8
      );
      const payout = this.bytesToBigInt(payoutBytes);
      
      bets.push({
        index: i,
        betType,
        amount,
        encodedBet,
        status: {
          resolved: isResolved,
          realizable: isRealizable,
          settled: isSettled,
          winning: isWinning
        },
        payout: Number(payout)
      });
    }
    
    return {
      epoch: this.bytesToBigInt(betBatch.epoch),
      player: this.bytesToAddress(betBatch.player),
      betCount: betBatch.betCount,
      totalAmount: this.bytesToBigInt(betBatch.totalAmount),
      payoutTotal: this.bytesToBigInt(betBatch.payoutTotal),
      bets
    };
  }
  
  private bytesToBigInt(bytes: number[]): bigint {
    let result = 0n;
    for (let i = 0; i < bytes.length; i++) {
      result |= BigInt(bytes[i]) << BigInt(i * 8);
    }
    return result;
  }
  
  private bytesToAddress(bytes: number[]): Address {
    return address(Buffer.from(bytes).toString('base64'));
  }
}
```

### 4. Real-time Game State Monitoring

Critical for mobile UX - players need real-time updates:

```typescript
// src/services/realtime/GameStateMonitor.ts
import { 
  fetchGlobalGameState,
  fetchRngState,
  type GlobalGameState,
  type RngState 
} from '@/clients/crapspinocchio';

export class GameStateMonitor {
  private gameStateSubscription?: () => void;
  private rngStateSubscription?: () => void;
  private callbacks: Map<string, (data: any) => void> = new Map();
  
  async startMonitoring(): Promise<void> {
    await Promise.all([
      this.subscribeToGameState(),
      this.subscribeToRngState()
    ]);
  }
  
  private async subscribeToGameState(): Promise<void> {
    const [globalGameStatePDA] = this.pdaService.getGlobalGameStatePDA();
    
    this.gameStateSubscription = this.rpc
      .accountSubscribe(globalGameStatePDA, {
        commitment: 'confirmed',
        encoding: 'base64'
      })
      .subscribe({
        next: async (notification) => {
          try {
            // Fetch and deserialize the account
            const gameState = await fetchGlobalGameState(this.rpc, globalGameStatePDA);
            const processedState = this.processGameState(gameState.data);
            
            // Notify all subscribers
            this.notifyCallbacks('gameState', processedState);
          } catch (error) {
            console.error('Error processing game state update:', error);
          }
        },
        error: (error) => {
          console.error('Game state subscription error:', error);
          this.reconnectAfterDelay();
        }
      });
  }
  
  private async subscribeToRngState(): Promise<void> {
    const [rngStatePDA] = this.pdaService.getRngStatePDA();
    
    this.rngStateSubscription = this.rpc
      .accountSubscribe(rngStatePDA, {
        commitment: 'confirmed',
        encoding: 'base64'
      })
      .subscribe({
        next: async (notification) => {
          try {
            const rngState = await fetchRngState(this.rpc, rngStatePDA);
            const processedRng = this.processRngState(rngState.data);
            
            this.notifyCallbacks('rngState', processedRng);
          } catch (error) {
            console.error('Error processing RNG state update:', error);
          }
        }
      });
  }
  
  private processGameState(raw: GlobalGameState): ProcessedGameState {
    return {
      epoch: this.bytesToNumber(raw.gameEpoch),
      dice: {
        total: raw.currentDice,
        die1: raw.currentDie1,
        die2: raw.currentDie2
      },
      point: raw.currentPoint,
      phase: raw.gamePhase === 0 ? 'come_out' : raw.gamePhase === 1 ? 'point' : 'ended',
      nextRollSlot: this.bytesToNumber(raw.nextRollSlot),
      epochStartSlot: this.bytesToNumber(raw.epochStartSlot),
      totalActiveBets: this.bytesToNumber(raw.totalActiveBets),
      rollCount: this.bytesToNumber(raw.epochRollCount),
      paused: raw.paused === 1,
      useSecureRng: raw.useSecureRng === 1,
      treasury: this.bytesToAddress(raw.treasury),
      crapTokenMint: this.bytesToAddress(raw.crapTokenMint)
    };
  }
  
  private processRngState(raw: RngState): ProcessedRngState {
    return {
      epoch: this.bytesToNumber(raw.epoch),
      phase: raw.phase, // 0=betting, 1=collection, 2=finalized
      hashCount: raw.hashCount,
      bettingStartSlot: this.bytesToNumber(raw.bettingStartSlot),
      collectionStartSlot: this.bytesToNumber(raw.collectionStartSlot),
      finalizationSlot: this.bytesToNumber(raw.finalizationSlot),
      finalValue: this.bytesToNumber(raw.finalValue),
      timeUntilNextPhase: this.calculateTimeUntilNextPhase(raw)
    };
  }
  
  private calculateTimeUntilNextPhase(rngState: RngState): number {
    const currentSlot = Date.now() / 400; // Rough slot estimation
    
    switch (rngState.phase) {
      case 0: // Betting phase
        const bettingEnd = this.bytesToNumber(rngState.bettingStartSlot) + 40;
        return Math.max(0, (bettingEnd - currentSlot) * 400);
        
      case 1: // Collection phase  
        const collectionEnd = this.bytesToNumber(rngState.collectionStartSlot) + 10;
        return Math.max(0, (collectionEnd - currentSlot) * 400);
        
      case 2: // Finalized
        return 0;
        
      default:
        return 0;
    }
  }
  
  // Subscription management
  onGameStateUpdate(callback: (state: ProcessedGameState) => void): () => void {
    const id = Math.random().toString(36);
    this.callbacks.set(`gameState_${id}`, callback);
    
    return () => this.callbacks.delete(`gameState_${id}`);
  }
  
  onRngStateUpdate(callback: (state: ProcessedRngState) => void): () => void {
    const id = Math.random().toString(36);
    this.callbacks.set(`rngState_${id}`, callback);
    
    return () => this.callbacks.delete(`rngState_${id}`);
  }
  
  private notifyCallbacks(type: string, data: any): void {
    for (const [key, callback] of this.callbacks.entries()) {
      if (key.startsWith(type)) {
        try {
          callback(data);
        } catch (error) {
          console.error(`Callback error for ${key}:`, error);
        }
      }
    }
  }
  
  cleanup(): void {
    this.gameStateSubscription?.();
    this.rngStateSubscription?.();
    this.callbacks.clear();
  }
}
```

### 5. Transaction Timing and Validation

Critical for proper betting window management:

```typescript
// src/services/timing/BettingWindowService.ts
export class BettingWindowService {
  async validateBettingWindow(): Promise<BettingWindowStatus> {
    const [rngStatePDA] = this.pdaService.getRngStatePDA();
    const rngState = await fetchRngState(this.rpc, rngStatePDA);
    
    const currentSlot = await this.getCurrentSlot();
    const bettingStartSlot = this.bytesToNumber(rngState.data.bettingStartSlot);
    const collectionStartSlot = this.bytesToNumber(rngState.data.collectionStartSlot);
    
    // Calculate window status
    const bettingEndSlot = bettingStartSlot + GAME_TIMING.BETTING_WINDOW_SLOTS;
    const bettingTimeRemaining = Math.max(0, (bettingEndSlot - currentSlot) * GAME_TIMING.SLOT_TIME_MS);
    
    return {
      isOpen: rngState.data.phase === RngPhase.BETTING && currentSlot < bettingEndSlot,
      timeRemainingMs: bettingTimeRemaining,
      currentPhase: rngState.data.phase,
      nextPhaseIn: this.calculateNextPhaseTime(rngState.data, currentSlot)
    };
  }
  
  async waitForBettingWindow(): Promise<void> {
    return new Promise((resolve) => {
      const checkWindow = async () => {
        const status = await this.validateBettingWindow();
        
        if (status.isOpen) {
          resolve();
        } else {
          // Wait for next phase
          setTimeout(checkWindow, Math.min(status.nextPhaseIn, 5000));
        }
      };
      
      checkWindow();
    });
  }
  
  private async getCurrentSlot(): Promise<number> {
    const response = await this.rpc.getSlot({ commitment: 'confirmed' }).send();
    return response;
  }
}
```

### 6. Error Handling for Program-Specific Errors

Handle the exact error codes from the craps program:

```typescript
// src/services/error/CrapsErrorHandler.ts
export class CrapsErrorHandler {
  static parseTransactionError(error: any): UserFriendlyError {
    // Check for program errors in logs
    if (error.logs && Array.isArray(error.logs)) {
      for (const log of error.logs) {
        // Extract error codes from Pinocchio program logs
        const errorMatch = log.match(/Program log: Error: (\d+)/);
        if (errorMatch) {
          const errorCode = parseInt(errorMatch[1]);
          return this.mapErrorCode(errorCode);
        }
        
        // Check for specific error patterns
        if (log.includes('insufficient balance')) {
          return ERROR_MESSAGES[CRAPS_ERRORS.INSUFFICIENT_BALANCE];
        }
        
        if (log.includes('betting window closed')) {
          return ERROR_MESSAGES[CRAPS_ERRORS.BETTING_WINDOW_CLOSED];
        }
        
        if (log.includes('invalid bet amount')) {
          return ERROR_MESSAGES[CRAPS_ERRORS.INVALID_BET_AMOUNT];
        }
        
        if (log.includes('game paused')) {
          return ERROR_MESSAGES[CRAPS_ERRORS.GAME_PAUSED];
        }
      }
    }
    
    // Handle MWA-specific errors
    if (error.message?.includes('User rejected')) {
      return {
        title: 'Transaction Cancelled',
        message: 'You cancelled the transaction',
        action: 'DISMISS'
      };
    }
    
    // Network and RPC errors
    if (error.message?.includes('timeout') || error.message?.includes('network')) {
      return {
        title: 'Network Error',
        message: 'Please check your connection and try again',
        action: 'RETRY'
      };
    }
    
    // Default fallback
    return {
      title: 'Transaction Failed',
      message: 'An unexpected error occurred',
      action: 'RETRY'
    };
  }
  
  private static mapErrorCode(code: number): UserFriendlyError {
    return ERROR_MESSAGES[code] || {
      title: 'Program Error',
      message: `Program error code: ${code}`,
      action: 'CONTACT_SUPPORT'
    };
  }
}
```

## Critical Implementation Checklist

### ✅ Account Management
- [ ] Player account initialization check on app launch
- [ ] Proper PDA derivation for all account types
- [ ] Account cleanup for rent recovery
- [ ] Balance tracking and updates

### ✅ Betting System
- [ ] Correct bet encoding (16-bit format)
- [ ] Bet validation against available phases
- [ ] Maximum 16 bets per batch enforcement
- [ ] Proper BetBatch account creation

### ✅ Game Flow
- [ ] Real-time game state monitoring
- [ ] RNG phase tracking for betting windows
- [ ] Epoch management and transitions
- [ ] Dice roll animation sync with on-chain data

### ✅ Token Operations
- [ ] Deposit/withdrawal with auto-claim variants
- [ ] Token account creation and management
- [ ] Balance validation before betting
- [ ] Payout claiming automation

### ✅ Error Handling
- [ ] Program-specific error parsing
- [ ] User-friendly error messages
- [ ] Automatic retry logic for network errors
- [ ] Graceful degradation for RPC issues

### ✅ Performance
- [ ] Efficient account subscription management
- [ ] Optimized transaction building
- [ ] Connection pooling and caching
- [ ] Background sync for game state

### ✅ Security
- [ ] Transaction simulation before sending
- [ ] PDA validation for all operations
- [ ] Amount validation against encoding table
- [ ] Secure key storage

## Testing with Actual Program

### Integration Tests

```typescript
// tests/integration/craps-program.test.ts
describe('Craps Program Integration', () => {
  let services: ServiceRegistry;
  let testWallet: TransactionSigner;
  
  beforeAll(async () => {
    services = ServiceRegistry.getInstance();
    testWallet = await generateKeyPairSigner();
  });
  
  test('should initialize player account', async () => {
    const signature = await services.transaction.initializePlayer();
    expect(signature).toMatch(/^[A-Za-z0-9]{88}$/);
    
    // Verify account exists
    const [playerStatePDA] = services.pda.getPlayerStatePDA(testWallet.address);
    const playerState = await fetchScalablePlayerState(services.rpc, playerStatePDA);
    expect(playerState.data.player).toEqual(testWallet.address.bytes);
  });
  
  test('should place bet correctly', async () => {
    // Ensure player is initialized and funded
    await services.transaction.deposit(1000n * BigInt(10**9));
    
    const currentEpoch = await services.gameState.getCurrentEpoch();
    const signature = await services.transaction.placeBet(
      BET_TYPES.PASS,
      100,
      currentEpoch
    );
    
    expect(signature).toBeTruthy();
    
    // Verify bet is recorded
    const bets = await services.gameState.getPlayerBets(testWallet.address, currentEpoch);
    expect(bets?.bets).toHaveLength(1);
    expect(bets?.bets[0].betType).toBe(BET_TYPES.PASS);
    expect(bets?.bets[0].amount).toBe(100);
  });
  
  test('should handle betting window timing', async () => {
    const windowStatus = await services.timing.validateBettingWindow();
    
    if (!windowStatus.isOpen) {
      await services.timing.waitForBettingWindow();
    }
    
    // Should be able to place bet now
    const signature = await services.transaction.placeBet(BET_TYPES.FIELD, 25, currentEpoch);
    expect(signature).toBeTruthy();
  });
  
  test('should decode bet batch correctly', async () => {
    const currentEpoch = await services.gameState.getCurrentEpoch();
    const betBatch = await services.betting.loadPlayerBets(testWallet.address, currentEpoch);
    
    expect(betBatch).toBeTruthy();
    expect(betBatch!.bets.length).toBeGreaterThan(0);
    
    // Verify bet encoding/decoding
    for (const bet of betBatch!.bets) {
      const reencoded = BetEncodingService.encodeBet(bet.betType, bet.amount);
      expect(reencoded).toBe(bet.encodedBet);
    }
  });
});
```

## Deployment Configuration

### Environment Variables

```bash
# .env.production
CRAPS_PROGRAM_ID=2yxPAyKVGMz6trfp8caRMWCoY5psMq6H1r4cuynLrvoX
HELIUS_API_KEY=your_helius_api_key
CRAP_TOKEN_MINT=CrapTokenMintAddress
DICE_TOKEN_MINT=DiceTokenMintAddress
RPC_ENDPOINT=https://mainnet.helius-rpc.com/?api-key=${HELIUS_API_KEY}
ENABLE_DEBUG_LOGS=false
```

### Build Configuration

```typescript
// mobile-frontend/metro.config.js - Updated for program integration
const { getDefaultConfig, mergeConfig } = require('@react-native/metro-config');

const config = {
  resolver: {
    extraNodeModules: {
      crypto: require.resolve('react-native-crypto'),
      stream: require.resolve('readable-stream'),
      buffer: require.resolve('buffer'),
    },
    alias: {
      '@/clients/crapspinocchio': '../../../clients/crapspinocchio',
      '@/constants': './src/constants',
      '@/services': './src/services'
    }
  },
  watchFolders: [
    path.resolve(__dirname, '../../../clients/crapspinocchio')
  ]
};

module.exports = mergeConfig(getDefaultConfig(__dirname), config);
```

This comprehensive integration guide ensures proper connection between the mobile frontend and the actual craps-pinocchio program, with all the specific details needed for production deployment.