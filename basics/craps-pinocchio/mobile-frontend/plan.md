# Craps-Pinocchio Mobile App Development Plan

## Executive Summary

This comprehensive plan outlines the development of a fully-featured Android mobile application for the Craps-Pinocchio Solana program. The app leverages the Solana App Kit, Anza Kit, and Codama-generated TypeScript clients to deliver a production-ready on-chain craps casino experience with modern mobile UX patterns.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Technology Stack](#technology-stack)
3. [Architecture](#architecture)
4. [UI/UX Design Specifications](#uiux-design-specifications)
5. [Solana App Kit Module Integration](#solana-app-kit-module-integration)
6. [Implementation Guide](#implementation-guide)
7. [Token Economics](#token-economics)
8. [Development Roadmap](#development-roadmap)
9. [Testing & Deployment](#testing--deployment)

## Project Overview

### Objectives
- Build a React Native Android app for the Craps-Pinocchio Solana program
- Leverage Solana Mobile Stack (SMS) with Mobile Wallet Adapter (MWA)
- Integrate Codama-generated TypeScript clients from `/clients/crapspinocchio`
- Implement token creation and liquidity features using Solana App Kit modules
- Create engaging UI for all 64 bet types with real-time blockchain synchronization

### Technology Stack
- **Framework**: React Native 0.76.9 (from solana-app-kit)
- **Blockchain SDK**: Anza Kit (@solana/kit) + Solana Kite (@helius-dev/kite)
- **Program Client**: Codama-generated TypeScript client (`/clients/crapspinocchio`)
- **Wallet Integration**: @solana-mobile/mobile-wallet-adapter-protocol-web3js
- **State Management**: Redux Toolkit with Redux Persist
- **UI Components**: React Native Paper + Custom Components
- **Token Features**: Token Mill, Pump.fun, Raydium, Meteora modules
- **RPC Provider**: Helius with mobile optimizations

## Architecture

### Application Layers

```
┌─────────────────────────────────────────────────┐
│                  UI Layer                       │
│  (React Native Components & Screens)            │
├─────────────────────────────────────────────────┤
│              Navigation Layer                   │
│  (React Navigation + Deep Linking)              │
├─────────────────────────────────────────────────┤
│            State Management Layer               │
│    (Redux Toolkit + Redux Persist)              │
├─────────────────────────────────────────────────┤
│             Service Layer                       │
│  (Game Logic, Wallet, Transaction Services)     │
├─────────────────────────────────────────────────┤
│           Blockchain Layer                      │
│  (Anza Kit, Codama Clients, Pinocchio Program) │
└─────────────────────────────────────────────────┘
```

### Core Modules Structure

```
mobile-frontend/
├── src/
│   ├── modules/
│   │   ├── craps-game/          # Main game module
│   │   │   ├── components/      # Table, dice, betting UI
│   │   │   ├── services/        # Game logic, transactions
│   │   │   └── hooks/           # Game state management
│   │   ├── wallet-integration/  # MWA implementation
│   │   ├── token-creation/      # CRAP token setup
│   │   └── liquidity/           # DEX integration
│   └── shared/
│       ├── services/            # Blockchain services
│       ├── hooks/               # Common hooks
│       └── utils/               # Helpers
└── clients/
    └── crapspinocchio/          # Codama-generated clients
```

## UI/UX Design Specifications

### Design System

```typescript
// Color Palette
const colors = {
  primary: {
    tableGreen: '#0B5D1E',
    gold: '#FFD700',
    red: '#DC2626'
  },
  chips: {
    5: '#DC2626',    // Red
    10: '#1E40AF',   // Blue
    25: '#059669',   // Green
    50: '#EA580C',   // Orange
    100: '#000000',  // Black
    500: '#7C3AED'   // Purple
  },
  ui: {
    background: '#111827',
    card: '#1F2937',
    text: '#F9FAFB',
    textSecondary: '#9CA3AF'
  }
};

// Typography
const typography = {
  fontFamily: 'SF Pro Display',
  sizes: {
    xs: 12,
    sm: 14,
    base: 16,
    lg: 18,
    xl: 20,
    '2xl': 24,
    '3xl': 30
  }
};
```

### Screen Specifications

#### 1. Main Game Table Screen
```
┌─────────────────────────────────────────────────┐
│ ◀ Menu        CRAPS        Balance ▼            │
│        Epoch #1234      100 CRAP                │
├─────────────────────────────────────────────────┤
│ ┌───────────────────────────────┐               │
│ │  COME OUT ROLL - Place Bets / 
           POINT ROLL              │
│ ├───────────────────────────────┤               │
│ │        SPECIAL BETS           │               │
│ │   (only displays on comeout)  │               │
│ ├───────────────────────────────┤               │
│ │  YES  │  NEXT  │  NO          │               │
│ │ (opens popup for 2-12)        │               │
│ ├─────────────┴───────────────┤               │
│ │         PASS / COME           │               │
│ │ (toggles on game state) │               │
│ ├───────────────────────────────┤               │
│ │   DON'T PASS / DON'T COME     │               │
│ │ (toggles on game state)       │               │
│ └───────────────────────────────┘               │
│ ├───────────────────────────────┤               │
│ │  FIELD  │  HARDWAYS           │               │
│ │ (hard opens popup for 4,6,8,12)    │               │
│                                   │
│  Current Dice: [ ⚃ ][ ⚄ ]        │
│  Total: 9                         │
│                                   │
│ ┌───────────────────────────────┐ │
│ │ Active Bets (3)         $45   │ │
│ │ • Pass Line        $20   ✓    │ │
│ │ • Field (9)        $10   ✓    │ │
│ │ • Come (6)         $15   ⏳   │ │
│ └───────────────────────────────┘ │
│                                   │
│ [1] [5] [10] [25] [50] [100]      │
│      Select Chip Amount           │
│                                   │
│    [ NEXT ROLL IN ... ]        │
└─────────────────────────────────────────────────┘
```

**Technical Specifications:**
- Interactive SVG table with 44x44px minimum touch targets
- 3D dice rendering with physics-based animation
- Real-time epoch and game phase indicators
- Chip selector with haptic feedback
- Animated bet placement and resolution

#### 2. Wallet Connection Screen
```
┌─────────────────────────────────────────────────┐
│                                 │
│     Welcome to Craps!           │
│                                 │
│    ┌───────────────────┐        │
│    │   [Dice Logo]     │        │
│    │    ⚀     ⚅       │        │
│    └───────────────────┘        │
│                                 │
│  Connect your wallet to start   │
│                                 │
│  ┌─────────────────────────┐    │
│  │ 🟣 Phantom             │    │
│  │ Connect with Phantom   │    │
│  └─────────────────────────┘    │
│                                 │
│  ┌─────────────────────────┐    │
│  │ 🔵 Solflare           │    │
│  │ Connect with Solflare  │    │
│  └─────────────────────────┘    │
│                                 │
│  ┌─────────────────────────┐    │
│  │ 📧 Email Login        │    │
│  │ Continue with email    │    │
│  └─────────────────────────┘    │
│                                 │
│   First time? Learn the rules   │
│                                 │
└─────────────────────────────────────────────────┘
```

#### 3. Bet Placement Modal
```
┌─────────────────────────────────┐
│      Place Your Bet      [X]    │
├─────────────────────────────────┤
│                                 │
│  Bet Type: PASS LINE            │
│  Current Odds: 1:1              │
│                                 │
│  ┌─────────────────────────┐    │
│  │      Bet Amount         │    │
│  │  ┌─────┐  ┌─────┐      │    │
│  │  │  -  │  │ 25  │  │ + ││    │
│  │  └─────┘  └─────┘  └───┘│    │
│  │                         │    │
│  │  Min: 1   Max: 10,000   │    │
│  └─────────────────────────┘    │
│                                 │
│  Quick Select:                  │
│  [1] [5] [10] [25] [50] [100]  │
│  [250] [500] [1K] [MAX]        │
│                                 │
│  ┌─────────────────────────┐    │
│  │ Balance: 1,000 CRAP     │    │
│  │ After bet: 975 CRAP     │    │
│  └─────────────────────────┘    │
│                                 │
│  [ Cancel ]    [ Place Bet ]    │
│                                 │
└─────────────────────────────────┘
```

### Animation Specifications

```typescript
// Dice roll animation
const diceAnimation = {
  duration: 2000, // 2 seconds
  physics: {
    gravity: 9.8,
    bounciness: 0.6,
    friction: 0.3
  },
  rotation: {
    x: Math.random() * 720,
    y: Math.random() * 720,
    z: Math.random() * 360
  }
};

// Chip placement animation
const chipPlacement = {
  duration: 300,
  scale: [0, 1.2, 1],
  opacity: [0, 1],
  haptic: 'impactLight'
};

// Win celebration
const winAnimation = {
  confetti: {
    count: 100,
    spread: 70,
    colors: ['#FFD700', '#FFA500', '#FF6347']
  },
  sound: 'win_chime.mp3',
  duration: 3000
};
```

## Token Creation & Liquidity Setup

### 1. Token Creation with Kite

```typescript
// modules/craps-game/services/tokenSetupService.ts
import { createToken, createMetadata, uploadMetadata } from '@helius-dev/kite';
import { createSolanaRpc, address, generateKeyPairSigner } from '@solana/kit';
import { transact, Web3MobileWallet } from '@solana-mobile/mobile-wallet-adapter-protocol-web3js';

export class CRAPTokenService {
  private rpc: ReturnType<typeof createSolanaRpc>;
  private kite: ReturnType<typeof connect>;

  constructor() {
    this.rpc = createSolanaRpc(process.env.RPC_ENDPOINT!);
    this.kite = connect(process.env.RPC_ENDPOINT!);
  }

  async deployMainToken() {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Create token metadata
      const metadata = {
        name: "Craps Casino Token",
        symbol: "CRAP",
        description: "Official token for Craps-Pinocchio casino gaming",
        image: "https://arweave.net/craps-token-logo",
        external_url: "https://craps-pinocchio.com",
        attributes: [
          { trait_type: "Token Type", value: "Gaming" },
          { trait_type: "Casino", value: "Craps-Pinocchio" },
          { trait_type: "Decimals", value: "9" }
        ]
      };

      // Upload metadata to IPFS
      const metadataUri = await uploadMetadata(metadata);

      // Create token mint
      const tokenMint = await createToken({
        name: "Craps Casino Token",
        symbol: "CRAP",
        decimals: 9,
        initialSupply: 1_000_000_000, // 1 billion tokens
        metadataUri,
        wallet: auth.publicKey,
        connection: this.kite.connection
      });

      return {
        mint: tokenMint.mint,
        signature: tokenMint.signature,
        metadataUri
      };
    });
  }

  async deployBonusToken() {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Create DICE bonus token metadata
      const metadata = {
        name: "Lucky Dice",
        symbol: "DICE",
        description: "Bonus rewards for epic wins in Craps-Pinocchio",
        image: "https://arweave.net/dice-token-logo",
        external_url: "https://craps-pinocchio.com/dice",
        attributes: [
          { trait_type: "Token Type", value: "Bonus" },
          { trait_type: "Casino", value: "Craps-Pinocchio" },
          { trait_type: "Decimals", value: "6" }
        ]
      };

      const metadataUri = await uploadMetadata(metadata);

      // Create bonus token with smaller decimals
      const bonusToken = await createToken({
        name: "Lucky Dice",
        symbol: "DICE",
        decimals: 6,
        initialSupply: 100_000_000, // 100 million bonus tokens
        metadataUri,
        wallet: auth.publicKey,
        connection: this.kite.connection
      });

      return {
        mint: bonusToken.mint,
        signature: bonusToken.signature,
        metadataUri
      };
    });
  }

  async distributeWinBonus(playerAddress: string, winAmount: bigint) {
    // Calculate bonus based on win size (10% of winnings in DICE)
    const bonusAmount = Number(winAmount) * 0.1;
    
    // All rewards paid in CRAP tokens only
    // No bonus tokens - simplified single-token economy
    });
  }
}
```

### 2. Liquidity Pool Creation with Raydium

```typescript
// modules/craps-game/services/liquidityService.ts
import { RaydiumService } from '@/modules/raydium/services/raydiumService';
import { createAssociatedTokenAccount, getAssociatedTokenAddress } from '@solana/spl-token';
import { transact, Web3MobileWallet } from '@solana-mobile/mobile-wallet-adapter-protocol-web3js';

export class CrapsLiquidityService {
  private raydium: RaydiumService;
  
  constructor() {
    this.raydium = new RaydiumService();
  }

  async createCRAPSOLPool(crapTokenMint: string) {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Create standard AMM pool on Raydium
      const poolResult = await this.raydium.createAndLaunchToken(
        {
          name: "CRAP",
          symbol: "CRAP",
          decimals: 9,
          description: "Craps Casino Token",
          imageUri: "https://arweave.net/craps-token-logo",
          twitter: "@crapscasino",
          website: "https://craps-pinocchio.com"
        },
        auth.publicKey,
        auth.sendTransaction,
        {
          statusCallback: (status) => console.log('Pool creation:', status)
        },
        {
          mode: 'standard',
          tokenSupply: "400000000", // 400M tokens for liquidity
          solRaised: "400", // 400 SOL initial liquidity
          lpTokensPercent: 100, // Keep all LP tokens
          createOnly: false,
          initialBuyAmount: "0"
        }
      );

      return poolResult;
    });
  }

  async createDICESOLPool(diceTokenMint: string) {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Create smaller pool for DICE token
      const poolResult = await this.raydium.createAndLaunchToken(
        {
          name: "DICE",
          symbol: "DICE", 
          decimals: 6,
          description: "Lucky Dice Bonus Token",
          imageUri: "https://arweave.net/dice-token-logo",
          twitter: "@crapscasino",
          website: "https://craps-pinocchio.com/dice"
        },
        auth.publicKey,
        auth.sendTransaction,
        {
          statusCallback: (status) => console.log('DICE pool creation:', status)
        },
        {
          mode: 'standard',
          tokenSupply: "50000000", // 50M DICE tokens
          solRaised: "50", // 50 SOL initial liquidity
          lpTokensPercent: 100,
          createOnly: false,
          initialBuyAmount: "0"
        }
      );

      return poolResult;
    });
  }

  async addLiquidityToPool(
    poolId: string,
    tokenAmount: number,
    solAmount: number
  ) {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Add additional liquidity to existing pool
      const result = await this.raydium.addLiquidity({
        poolId,
        tokenAmount,
        solAmount,
        slippage: 0.5, // 0.5% slippage tolerance
        wallet: auth.publicKey
      });

      return result;
    });
  }

  async manageLiquidityBasedOnVolume(poolId: string) {
    const poolStats = await this.raydium.getPoolStatistics(poolId);
    
    if (poolStats.volume24h > 1000000) { // High volume
      // Add more liquidity to reduce slippage
      await this.addLiquidityToPool(
        poolId,
        poolStats.volume24h * 0.1, // 10% of volume in tokens
        poolStats.volume24h * 0.0001 // Equivalent SOL
      );
    }
    
    // Monitor price impact and adjust accordingly
    if (poolStats.priceImpact > 0.05) { // 5% price impact
      await this.rebalancePool(poolId);
    }
  }

  private async rebalancePool(poolId: string) {
    // Implement pool rebalancing logic
    const poolInfo = await this.raydium.getPoolInfo(poolId);
    
    // Calculate optimal token ratios
    const optimalRatio = await this.calculateOptimalRatio(poolInfo);
    
    // Rebalance if needed
    if (Math.abs(poolInfo.currentRatio - optimalRatio) > 0.1) {
      await this.raydium.rebalancePool({
        poolId,
        targetRatio: optimalRatio,
        maxSlippage: 0.02 // 2% max slippage
      });
    }
  }
}
```

### 3. Token Distribution Strategy

```typescript
// modules/craps-game/services/tokenDistributionService.ts
export class TokenDistributionService {
  private readonly TOKEN_ALLOCATION = {
    gameplay: 0.40,      // 40% for gameplay rewards
    liquidity: 0.30,     // 30% for DEX liquidity  
    treasury: 0.15,      // 15% for house treasury
    staking: 0.10,       // 10% for staking rewards
    team: 0.05          // 5% for team (vested)
  };

  async distributeTokens(
    tokenMint: string,
    totalSupply: bigint,
    treasuryWallet: string
  ) {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Create distribution accounts
      const distributions = await Promise.all([
        this.createDistributionAccount('gameplay', totalSupply),
        this.createDistributionAccount('liquidity', totalSupply),
        this.createDistributionAccount('treasury', totalSupply),
        this.createDistributionAccount('staking', totalSupply),
        this.createDistributionAccount('team', totalSupply)
      ]);

      // Transfer tokens to each distribution account
      for (const dist of distributions) {
        await this.kite.transferTokens({
          mint: tokenMint,
          recipient: dist.account,
          amount: Number(dist.amount),
          decimals: 9
        });
      }

      return distributions;
    });
  }

  private async createDistributionAccount(
    type: string,
    totalSupply: bigint
  ) {
    const allocation = this.TOKEN_ALLOCATION[type];
    const amount = totalSupply * BigInt(Math.floor(allocation * 100)) / 100n;
    
    // Create token account for this distribution
    const account = await createAssociatedTokenAccount(
      this.kite.connection,
      this.treasuryWallet,
      this.tokenMint,
      this.treasuryWallet.publicKey
    );

    return {
      type,
      account: account.toBase58(),
      amount,
      allocation
    };
  }
}
```

## Implementation Guide

### Detailed Integration Documentation

This plan is supported by comprehensive implementation guides:

1. **[implementation-guide.md](./implementation-guide.md)** - Complete code implementation with working examples
2. **[constants.md](./constants.md)** - All constants, bet types, and configuration values
3. **[program-integration-guide.md](./program-integration-guide.md)** - Detailed craps-pinocchio program integration

### 1. Wallet Integration with MWA

```typescript
// modules/wallet-integration/services/mwaService.ts
import { transact, Web3MobileWallet } from '@solana-mobile/mobile-wallet-adapter-protocol-web3js';
import { address } from '@solana/kit';

export class MobileWalletService {
  private authToken: string | null = null;
  
  async connect(): Promise<string> {
    return await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      return auth.publicKey;
    });
  }

  private async authorizeSession(wallet: Web3MobileWallet) {
    // Try reauthorization first for better UX
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
    const auth = await wallet.authorize({
      cluster: 'mainnet-beta',
      identity: {
        name: 'Craps Pinocchio',
        uri: 'https://craps-pinocchio.com',
        icon: 'data:image/png;base64,...'
      }
    });
    
    this.authToken = auth.auth_token;
    return auth;
  }
}
```

### 2. Transaction Building with Codama Clients

```typescript
// modules/craps-game/services/transactionService.ts
import { 
  pipe, 
  createTransactionMessage,
  setTransactionMessageLifetimeUsingBlockhash,
  appendTransactionMessageInstructions,
  address,
  lamports
} from '@solana/kit';
import {
  getPlaceBetInstruction,
  getDepositWithAutoClaimV2Instruction,
  getClaimEpochPayoutsUnifiedInstruction,
  getGlobalGameStateAccountDataSerializer,
  type GlobalGameState
} from '@/clients/crapspinocchio';

export class CrapsTransactionService {
  private rpc: ReturnType<typeof createSolanaRpc>;
  
  async placeBet(
    playerAddress: string,
    betType: number,
    amount: number,
    epoch: number
  ) {
    await transact(async (wallet: Web3MobileWallet) => {
      const auth = await this.authorizeSession(wallet);
      
      // Derive PDAs
      const [playerStatePDA] = await this.deriveAddress([
        Buffer.from('scalable_player'),
        address(playerAddress).toBuffer()
      ]);
      
      const [betBatchPDA] = await this.deriveAddress([
        Buffer.from('bet_batch'),
        address(playerAddress).toBuffer(),
        new BN(epoch).toArrayLike(Buffer, 'le', 8)
      ]);
      
      // Build instruction using Codama client
      const placeBetIx = getPlaceBetInstruction({
        betBatch: address(betBatchPDA),
        playerState: address(playerStatePDA),
        globalGameState: address(this.globalGameStatePDA),
        player: auth.publicKey,
        systemProgram: address('11111111111111111111111111111111')
      }, {
        bets: [{
          encodedBet: this.encodeBet(betType, amount)
        }]
      });
      
      // Build transaction message
      const { value: latestBlockhash } = await this.rpc
        .getLatestBlockhash({ commitment: 'confirmed' })
        .send();
      
      const message = pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstructions([placeBetIx], tx)
      );
      
      // Sign and send
      const result = await wallet.signAndSendTransactions({
        transactions: [message]
      });
      
      return result[0];
    });
  }
  
  private encodeBet(betType: number, amountIndex: number): number {
    // Pinocchio 16-bit bet encoding
    return (betType << 10) | (amountIndex & 0x3FF);
  }
  
  private getAmountIndex(amount: number): number {
    // Non-linear encoding for amounts 1-100,000
    if (amount <= 10) return amount - 1;
    if (amount <= 100) return Math.floor(10 + (amount - 10) / 1.1);
    if (amount <= 1000) return Math.floor(90 + (amount - 100) / 10);
    if (amount <= 10000) return Math.floor(180 + (amount - 1000) / 100);
    return Math.floor(270 + (amount - 10000) / 1000);
  }
}
```

### 3. Real-time Game State Synchronization

```typescript
// modules/craps-game/services/gameStateService.ts
import { createSolanaRpc } from '@solana/kit';
import { 
  getGlobalGameStateAccountDataSerializer,
  getBetBatchAccountDataSerializer,
  getScalablePlayerStateAccountDataSerializer
} from '@/clients/crapspinocchio';

export class GameStateService {
  private rpc: ReturnType<typeof createSolanaRpc>;
  private subscriptions = new Map<string, () => void>();
  
  subscribeToGameState(callback: (state: GlobalGameState) => void) {
    const subscription = this.rpc
      .accountSubscribe(
        this.globalGameStatePDA,
        { commitment: 'confirmed', encoding: 'base64' }
      )
      .subscribe({
        next: (notification) => {
          // Deserialize with Codama serializer
          const data = Buffer.from(notification.value.data[0], 'base64');
          const serializer = getGlobalGameStateAccountDataSerializer();
          const [gameState] = serializer.deserialize(data);
          
          // Convert byte arrays to numbers
          const processedState = {
            gameEpoch: new BN(gameState.gameEpoch).toNumber(),
            currentDice: gameState.currentDice,
            currentDie1: gameState.currentDie1,
            currentDie2: gameState.currentDie2,
            currentPoint: gameState.currentPoint,
            gamePhase: gameState.gamePhase === 0 ? 'come_out' : 'point',
            nextRollSlot: new BN(gameState.nextRollSlot).toString(),
            paused: gameState.paused === 1
          };
          
          callback(processedState);
        },
        error: (error) => {
          console.error('Game state subscription error:', error);
          this.reconnect();
        }
      });
    
    this.subscriptions.set('gameState', () => subscription.unsubscribe());
  }
  
  async fetchPlayerBets(playerAddress: string, epoch: number) {
    const [betBatchPDA] = await this.deriveAddress([
      Buffer.from('bet_batch'),
      address(playerAddress).toBuffer(),
      new BN(epoch).toArrayLike(Buffer, 'le', 8)
    ]);
    
    const accountInfo = await this.rpc
      .getAccountInfo(betBatchPDA, { encoding: 'base64' })
      .send();
    
    if (!accountInfo.value?.data) return null;
    
    const data = Buffer.from(accountInfo.value.data[0], 'base64');
    const serializer = getBetBatchAccountDataSerializer();
    const [betBatch] = serializer.deserialize(data);
    
    // Decode packed bets
    const bets = [];
    for (let i = 0; i < betBatch.betCount; i++) {
      const encodedBet = (betBatch.packedBets[i * 2] << 8) | betBatch.packedBets[i * 2 + 1];
      const betType = (encodedBet >> 10) & 0x3F;
      const amountIndex = encodedBet & 0x3FF;
      
      bets.push({
        type: betType,
        amount: this.getAmountFromIndex(amountIndex),
        resolved: (betBatch.resolvedMask[Math.floor(i / 8)] & (1 << (i % 8))) !== 0,
        realizable: (betBatch.realizableMask[Math.floor(i / 8)] & (1 << (i % 8))) !== 0,
        settled: (betBatch.settledMask[Math.floor(i / 8)] & (1 << (i % 8))) !== 0,
        payout: new BN(betBatch.individualPayouts.slice(i * 8, (i + 1) * 8)).toNumber()
      });
    }
    
    return { epoch, bets };
  }
}
```

### 4. Error Handling for Pinocchio Programs

```typescript
// modules/craps-game/utils/errorHandler.ts
export class PinocchioErrorHandler {
  static parse(error: any): UserFriendlyError {
    // Parse program errors from transaction logs
    if (error.logs) {
      for (const log of error.logs) {
        // Pinocchio error patterns
        if (log.includes('InvalidBetAmount')) {
          return {
            title: 'Invalid Bet',
            message: 'Bet amount must be between 1 and 100,000 CRAP',
            action: 'ADJUST_BET'
          };
        }
        
        if (log.includes('InsufficientBalance')) {
          return {
            title: 'Insufficient Balance',
            message: 'Not enough CRAP tokens. Please deposit more.',
            action: 'SHOW_DEPOSIT'
          };
        }
        
        if (log.includes('BettingWindowClosed')) {
          return {
            title: 'Betting Closed',
            message: 'Betting window has closed. Wait for next round.',
            action: 'WAIT'
          };
        }
        
        if (log.includes('GamePaused')) {
          return {
            title: 'Game Paused',
            message: 'The game is temporarily paused.',
            action: 'RETRY_LATER'
          };
        }
        
        if (log.includes('MaxBetsReached')) {
          return {
            title: 'Too Many Bets',
            message: 'Maximum 16 bets per batch reached.',
            action: 'PLACE_FEWER_BETS'
          };
        }
      }
    }
    
    // MWA errors
    if (error.message?.includes('User declined')) {
      return {
        title: 'Cancelled',
        message: 'Transaction cancelled',
        action: 'DISMISS'
      };
    }
    
    // Network errors
    if (error.message?.includes('Network request failed')) {
      return {
        title: 'Connection Error',
        message: 'Check your internet connection',
        action: 'RETRY',
        retry: true
      };
    }
    
    return {
      title: 'Transaction Failed',
      message: 'Something went wrong. Please try again.',
      action: 'RETRY'
    };
  }
}
```

### 5. State Management with Redux

```typescript
// modules/craps-game/state/gameSlice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type { GlobalGameState, BetBatch } from '@/clients/crapspinocchio';

interface GameState {
  // Game state from blockchain
  epoch: number;
  phase: 'come_out' | 'point';
  point: number;
  dice: {
    die1: number;
    die2: number;
    total: number;
  };
  nextRollSlot: string;
  paused: boolean;
  
  // Player state
  balance: bigint;
  initialized: boolean;
  totalWagered: bigint;
  totalWon: bigint;
  
  // Betting state
  activeBets: BetInfo[];
  pendingBets: PendingBet[];
  betHistory: BetBatch[];
}

const gameSlice = createSlice({
  name: 'game',
  initialState: {
    epoch: 0,
    phase: 'come_out',
    point: 0,
    dice: { die1: 0, die2: 0, total: 0 },
    nextRollSlot: '0',
    paused: false,
    balance: 0n,
    initialized: false,
    totalWagered: 0n,
    totalWon: 0n,
    activeBets: [],
    pendingBets: [],
    betHistory: []
  } as GameState,
  reducers: {
    updateGameState: (state, action: PayloadAction<Partial<GlobalGameState>>) => {
      const { gameEpoch, currentDie1, currentDie2, currentPoint, gamePhase } = action.payload;
      
      if (gameEpoch) state.epoch = gameEpoch;
      if (currentDie1 !== undefined && currentDie2 !== undefined) {
        state.dice = {
          die1: currentDie1,
          die2: currentDie2,
          total: currentDie1 + currentDie2
        };
      }
      if (currentPoint !== undefined) state.point = currentPoint;
      if (gamePhase !== undefined) state.phase = gamePhase === 0 ? 'come_out' : 'point';
    },
    
    placeBetOptimistic: (state, action: PayloadAction<PendingBet>) => {
      state.pendingBets.push(action.payload);
      state.balance -= BigInt(action.payload.amount);
    },
    
    confirmBet: (state, action: PayloadAction<{ betId: string; signature: string }>) => {
      const index = state.pendingBets.findIndex(b => b.id === action.payload.betId);
      if (index !== -1) {
        const [bet] = state.pendingBets.splice(index, 1);
        state.activeBets.push({
          ...bet,
          signature: action.payload.signature,
          status: 'confirmed'
        });
      }
    },
    
    revertBet: (state, action: PayloadAction<string>) => {
      const index = state.pendingBets.findIndex(b => b.id === action.payload);
      if (index !== -1) {
        const [bet] = state.pendingBets.splice(index, 1);
        state.balance += BigInt(bet.amount);
      }
    },
    
    updateBalance: (state, action: PayloadAction<string>) => {
      state.balance = BigInt(action.payload);
    },
    
    settleBets: (state, action: PayloadAction<BetSettlement[]>) => {
      for (const settlement of action.payload) {
        const betIndex = state.activeBets.findIndex(b => b.id === settlement.betId);
        if (betIndex !== -1) {
          state.activeBets[betIndex].status = settlement.won ? 'won' : 'lost';
          state.activeBets[betIndex].payout = settlement.payout;
          
          if (settlement.won) {
            state.balance += BigInt(settlement.payout);
            state.totalWon += BigInt(settlement.payout);
          }
        }
      }
    }
  }
});

export const {
  updateGameState,
  placeBetOptimistic,
  confirmBet,
  revertBet,
  updateBalance,
  settleBets
} = gameSlice.actions;

export default gameSlice.reducer;
```

## Token Economics

### Token Distribution

```typescript
const TOKEN_ALLOCATION = {
  gameplay: 400_000_000,    // 40% - Player rewards & gameplay mining
  liquidity: 300_000_000,   // 30% - DEX liquidity (Raydium pools)
  treasury: 150_000_000,    // 15% - House treasury & operations
  staking: 100_000_000,     // 10% - Staking rewards pool
  team: 50_000_000          // 5% - Team allocation (vested)
};

const GAMEPLAY_INCENTIVES = {
  winMultiplier: 1.0,       // Base payout per game rules
  diceBonus: 0.1,           // 10% bonus in DICE tokens for big wins
  stakingBoost: 0.05,       // 5% extra for staked players
  referralBonus: 0.005,     // 0.5% referral rewards
  // House edge is implied in game rules, not specified as percentage
};
```

### Single Token System

```typescript
// CRAP Token - Main and only gaming token
const CRAP_TOKEN = {
  name: "Craps Casino Token",
  symbol: "CRAP",
  decimals: 9,
  totalSupply: 1_000_000_000_000, // 1 trillion tokens
  use: "Primary gaming currency and rewards",
  initialPrice: 0.000001 // 1 CRAP = 0.000001 SOL (1M CRAP per SOL)
};

// Token Allocation
const TOKEN_ALLOCATION = {
  TEAM_INVESTORS_TREASURY: 500_000_000_000,  // 50% for team/investors/treasury (dev keypair)
  LIQUIDITY: 100_000_000_000,                // 10% for DEX liquidity pool  
  COMMUNITY_REWARDS: 400_000_000_000         // 40% for community rewards and airdrops
};
```

### Liquidity Strategy

```typescript
const LIQUIDITY_STRATEGY = {
  crapSolPool: {
    platform: 'Raydium',
    initialLiquidity: {
      crapTokens: 100_000_000_000,  // 100B CRAP (10% of total supply)
      solAmount: 100,               // 100 SOL
      ratio: 1_000_000_000         // 1B CRAP per SOL
    },
    feeStructure: {
      tradingFee: 0.0025,          // 0.25% trading fee
      protocolFee: 0.0003          // 0.03% protocol fee
    }
  }
};
```

### Staking Rewards

```typescript
const STAKING_TIERS = [
  {
    minStake: 1_000,
    name: 'Bronze',
    benefits: {
      cashback: 0.001,      // 0.1% cashback on losses
      bonusMultiplier: 1.05, // 5% win bonus
      freeRollsDaily: 1
    }
  },
  {
    minStake: 10_000,
    name: 'Silver',
    benefits: {
      cashback: 0.0025,     // 0.25% cashback
      bonusMultiplier: 1.10, // 10% win bonus
      freeRollsDaily: 3
    }
  },
  {
    minStake: 100_000,
    name: 'Gold',
    benefits: {
      cashback: 0.005,      // 0.5% cashback
      bonusMultiplier: 1.20, // 20% win bonus
      freeRollsDaily: 5
    }
  },
  {
    minStake: 1_000_000,
    name: 'Diamond',
    benefits: {
      cashback: 0.0075,     // 0.75% cashback
      bonusMultiplier: 1.30, // 30% win bonus
      freeRollsDaily: 10,
      vipSupport: true,
      customLimits: true
    }
  }
];
```

### Liquidity Mining Program

```typescript
const LIQUIDITY_MINING = {
  totalRewards: 100_000_000,     // 100M DICE tokens over 1 year
  duration: 365 * 24 * 60 * 60,  // 1 year program
  distribution: 'linear',         // Linear vesting
  
  pools: [
    {
      pair: 'CRAP/SOL',
      platform: 'Raydium',
      weight: 0.7,              // 70% of rewards
      minLiquidity: 1000,       // 1000 SOL minimum
      lockPeriod: 0             // No lock period
    },
    {
      pair: 'DICE/SOL', 
      platform: 'Raydium',
      weight: 0.3,              // 30% of rewards
      minLiquidity: 100,        // 100 SOL minimum
      lockPeriod: 0             // No lock period
    }
  ],
  
  // Bonus multipliers for long-term LPs
  loyaltyMultipliers: {
    '7days': 1.0,     // Base rate
    '30days': 1.1,    // 10% bonus
    '90days': 1.25,   // 25% bonus
    '180days': 1.5    // 50% bonus
  }
};
```

## Development Roadmap

### Phase 1: Foundation (Weeks 1-2)

1. **Environment Setup**
   - Fork solana-app-kit repository
   - Configure React Native for Android
   - Set up Anza Kit and Helius RPC
   - Integrate Codama clients from `/clients/crapspinocchio`

2. **Core Infrastructure**
   ```bash
   # Install dependencies
   npm install @solana/kit @helius-dev/kite \
     @solana-mobile/mobile-wallet-adapter-protocol-web3js \
     @reduxjs/toolkit react-redux redux-persist \
     react-native-reanimated react-native-gesture-handler
   ```

3. **MWA Integration**
   - Implement wallet connection flow
   - Set up transaction signing
   - Test with Phantom and Solflare

### Phase 2: Game Core (Weeks 3-4)

1. **UI Components**
   - Craps table with SVG rendering
   - 3D dice with physics engine
   - Betting interface for 64 bet types
   - Real-time game state display

2. **Blockchain Integration**
   - Connect all Codama instructions
   - Implement bet encoding/decoding
   - Real-time state synchronization
   - Error handling and retry logic

3. **Player Features**
   - Account initialization
   - Deposit/withdrawal flows
   - Bet history tracking
   - Win/loss statistics

### Phase 3: Token Features (Weeks 5-6)

1. **CRAP Token Launch**
   - Deploy using Kite's createToken function
   - Create metadata and upload to IPFS
   - Set up initial token distribution
   - Create Raydium CRAP/SOL pool

2. **DICE Bonus Token**
   - Deploy DICE token using Kite
   - Create separate Raydium DICE/SOL pool
   - Implement bonus distribution logic
   - Set up liquidity mining rewards

3. **DeFi Integration**
   - Integrate Raydium swap functionality
   - Add liquidity provision features
   - Implement staking rewards system
   - Create LP token management

### Phase 4: Polish & Launch (Weeks 7-8)

1. **Performance Optimization**
   - Bundle size < 50MB
   - 60 FPS animations
   - Offline mode support
   - Battery optimization

2. **Security Audit**
   - Transaction validation
   - Rate limiting
   - Anti-fraud measures
   - Penetration testing

3. **Launch Preparation**
   - Beta testing program
   - Google Play Store + Solana dApp Store submission
   - Marketing materials
   - Documentation

## Testing & Deployment

### Testing Strategy

```typescript
// Unit tests for bet encoding
describe('BetEncoder', () => {
  it('should encode and decode bets correctly', () => {
    const betType = 5; // Pass line
    const amount = 100;
    const encoded = BetEncoder.encode(betType, amount);
    const decoded = BetEncoder.decode(encoded);
    
    expect(decoded.betType).toBe(betType);
    expect(BetEncoder.getAmountFromIndex(decoded.amountIndex)).toBe(amount);
  });
});

// Integration tests for MWA
describe('MWA Integration', () => {
  it('should connect to wallet and sign transaction', async () => {
    const service = new MobileWalletService();
    const publicKey = await service.connect();
    expect(publicKey).toBeTruthy();
    
    const tx = await service.signAndSend(testTransaction);
    expect(tx).toMatch(/^[A-Za-z0-9]{88}$/);
  });
});

// E2E tests
describe('Complete Game Flow', () => {
  it('should place bet and receive payout', async () => {
    await device.launchApp();
    await element(by.id('connect-wallet')).tap();
    await element(by.id('phantom-wallet')).tap();
    
    // Place bet
    await element(by.id('pass-line')).tap();
    await element(by.id('bet-25')).tap();
    await element(by.id('place-bet')).tap();
    
    // Wait for roll
    await waitFor(element(by.id('dice-result')))
      .toBeVisible()
      .withTimeout(5000);
    
    // Check payout
    const balance = await element(by.id('balance')).getText();
    expect(parseInt(balance)).toBeGreaterThan(975);
  });
});
```

### Deployment Configuration

```javascript
// metro.config.js
module.exports = {
  resolver: {
    extraNodeModules: {
      crypto: require.resolve('react-native-crypto'),
      stream: require.resolve('stream-browserify'),
      buffer: require.resolve('buffer')
    }
  },
  transformer: {
    minifierConfig: {
      keep_fnames: true,
      mangle: {
        keep_fnames: true
      }
    }
  }
};

// Build configuration
const buildConfig = {
  android: {
    minSdkVersion: 23,
    targetSdkVersion: 34,
    enableProguardInReleaseBuilds: true,
    enableShrinkResourcesInReleaseBuilds: true
  },
  optimization: {
    useBytecode: true,
    enableHermes: true
  }
};
```

### Performance Metrics

- **App Size**: < 50MB APK
- **Launch Time**: < 2 seconds
- **Transaction Speed**: < 3 seconds confirmation
- **Memory Usage**: < 200MB peak
- **Battery Impact**: < 5% per hour active use
- **Network Usage**: < 10MB per hour

## Conclusion

This comprehensive plan provides everything needed to build a production-ready Craps mobile game on Solana. By leveraging:

1. **Solana App Kit modules** for token creation and DeFi features
2. **Anza Kit** for optimal performance and smaller bundles
3. **Codama-generated clients** for type-safe program interaction
4. **Mobile Wallet Adapter** for seamless wallet integration
5. **Pinocchio program architecture** for efficient on-chain gaming

The app will showcase the full potential of Solana for real-money mobile gaming with instant settlements, provably fair dice rolls, and integrated DeFi features.
