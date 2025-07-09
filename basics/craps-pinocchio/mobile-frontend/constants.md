# Craps-Pinocchio Constants and Configuration

## Program Constants

```typescript
// src/constants/crapsConstants.ts
import { CRAPS_PINOCCHIO_PROGRAM_ADDRESS } from '@/clients/crapspinocchio';

// Program addresses
export const PROGRAM_CONSTANTS = {
  PROGRAM_ID: CRAPS_PINOCCHIO_PROGRAM_ADDRESS, // '2yxPAyKVGMz6trfp8caRMWCoY5psMq6H1r4cuynLrvoX'
  TOKEN_PROGRAM: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  ASSOCIATED_TOKEN_PROGRAM: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
  SYSTEM_PROGRAM: '11111111111111111111111111111111'
} as const;

// Token configuration
export const TOKEN_CONFIG = {
  CRAP_TOKEN_MINT: process.env.CRAP_TOKEN_MINT!,
  DICE_TOKEN_MINT: process.env.DICE_TOKEN_MINT!,
  DECIMALS: 9,
  MAX_SUPPLY: 1_000_000_000_000, // 1 trillion
} as const;

// Game phases from the program
export enum GamePhase {
  COME_OUT = 0,
  POINT = 1,
  ENDED = 2
}

// RNG phases for timing
export enum RngPhase {
  BETTING = 0,
  COLLECTION = 1, 
  FINALIZED = 2
}

// All 64 bet types supported by the craps program
export const BET_TYPES = {
  // Core game bets (0-4)
  PASS: 0,
  DONT_PASS: 1,
  COME: 2,
  DONT_COME: 3,
  FIELD: 4,
  
  // YES bets - number before 7 (5-14)
  YES_2: 5,
  YES_3: 6,
  YES_4: 7,
  YES_5: 8,
  YES_6: 9,
  YES_8: 10,
  YES_9: 11,
  YES_10: 12,
  YES_11: 13,
  YES_12: 14,
  
  // NO bets - 7 before number (15-24)
  NO_2: 15,
  NO_3: 16,
  NO_4: 17,
  NO_5: 18,
  NO_6: 19,
  NO_8: 20,
  NO_9: 21,
  NO_10: 22,
  NO_11: 23,
  NO_12: 24,
  
  // Hard ways (25-28)
  HARD4: 25,
  HARD6: 26,
  HARD8: 27,
  HARD10: 28,
  
  // Odds bets - require base bet (29-32)
  ODDS_PASS: 29,
  ODDS_DONT_PASS: 30,
  ODDS_COME: 31,
  ODDS_DONT_COME: 32,
  
  // Special/bonus bets (33-42)
  HOT_ROLLER: 33,
  FIRE: 34,
  TWICE_HARD: 35,
  RIDE_LINE: 36,
  MUGGSY: 37,
  BONUS_SMALL: 38,
  BONUS_TALL: 39,
  BONUS_SMALL_TALL: 40,
  REPLAY: 41,
  DIFFERENT_DOUBLES: 42,
  
  // One-roll NEXT bets (43-53)
  NEXT_2: 43,
  NEXT_3: 44,
  NEXT_4: 45,
  NEXT_5: 46,
  NEXT_6: 47,
  NEXT_7: 48,
  NEXT_8: 49,
  NEXT_9: 50,
  NEXT_10: 51,
  NEXT_11: 52,
  NEXT_12: 53,
  
  // Repeater bets (54-63)
  REPEATER_2: 54,
  REPEATER_3: 55,
  REPEATER_4: 56,
  REPEATER_5: 57,
  REPEATER_6: 58,
  REPEATER_8: 59,
  REPEATER_9: 60,
  REPEATER_10: 61,
  REPEATER_11: 62,
  REPEATER_12: 63
} as const;

// Bet type information for UI
export const BET_INFO = {
  [BET_TYPES.PASS]: {
    name: 'Pass Line',
    description: 'Wins on 7/11, loses on 2/3/12, pushes on point',
    odds: '1:1',
    houseEdge: 1.36,
    category: 'main',
    availablePhases: [GamePhase.COME_OUT]
  },
  [BET_TYPES.DONT_PASS]: {
    name: "Don't Pass",
    description: 'Opposite of Pass Line, 12 is push',
    odds: '1:1',
    houseEdge: 1.4,
    category: 'main',
    availablePhases: [GamePhase.COME_OUT]
  },
  [BET_TYPES.COME]: {
    name: 'Come',
    description: 'Like Pass Line but after point established',
    odds: '1:1',
    houseEdge: 1.36,
    category: 'main',
    availablePhases: [GamePhase.POINT]
  },
  [BET_TYPES.DONT_COME]: {
    name: "Don't Come",
    description: 'Like Don\'t Pass but after point established',
    odds: '1:1',
    houseEdge: 1.4,
    category: 'main',
    availablePhases: [GamePhase.POINT]
  },
  [BET_TYPES.FIELD]: {
    name: 'Field',
    description: 'One roll bet on 2,3,4,9,10,11,12',
    odds: '1:1 (2:1 on 2,12)',
    houseEdge: 2.78,
    category: 'one-roll',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.YES_4]: {
    name: 'Yes 4',
    description: '4 rolled before 7',
    odds: '9:5',
    houseEdge: 6.67,
    category: 'place',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.YES_5]: {
    name: 'Yes 5',
    description: '5 rolled before 7',
    odds: '7:5',
    houseEdge: 4.0,
    category: 'place',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.YES_6]: {
    name: 'Yes 6',
    description: '6 rolled before 7',
    odds: '7:6',
    houseEdge: 1.52,
    category: 'place',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.YES_8]: {
    name: 'Yes 8',
    description: '8 rolled before 7',
    odds: '7:6',
    houseEdge: 1.52,
    category: 'place',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.YES_9]: {
    name: 'Yes 9',
    description: '9 rolled before 7',
    odds: '7:5',
    houseEdge: 4.0,
    category: 'place',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.YES_10]: {
    name: 'Yes 10',
    description: '10 rolled before 7',
    odds: '9:5',
    houseEdge: 6.67,
    category: 'place',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.HARD4]: {
    name: 'Hard 4',
    description: 'Two 2s before 7 or easy 4',
    odds: '7:1',
    houseEdge: 11.11,
    category: 'hardway',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.HARD6]: {
    name: 'Hard 6',
    description: 'Two 3s before 7 or easy 6',
    odds: '9:1',
    houseEdge: 9.09,
    category: 'hardway',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.HARD8]: {
    name: 'Hard 8',
    description: 'Two 4s before 7 or easy 8',
    odds: '9:1',
    houseEdge: 9.09,
    category: 'hardway',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.HARD10]: {
    name: 'Hard 10',
    description: 'Two 5s before 7 or easy 10',
    odds: '7:1',
    houseEdge: 11.11,
    category: 'hardway',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.NEXT_7]: {
    name: 'Next Roll 7',
    description: 'Next roll is 7',
    odds: '4:1',
    houseEdge: 16.67,
    category: 'next-roll',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.NEXT_11]: {
    name: 'Next Roll 11',
    description: 'Next roll is 11',
    odds: '15:1',
    houseEdge: 11.11,
    category: 'next-roll',
    availablePhases: [GamePhase.COME_OUT, GamePhase.POINT]
  },
  [BET_TYPES.HOT_ROLLER]: {
    name: 'Hot Roller',
    description: 'Progressive bonus for consecutive points',
    odds: 'Progressive',
    houseEdge: 5.56,
    category: 'bonus',
    availablePhases: [GamePhase.COME_OUT]
  },
  [BET_TYPES.FIRE]: {
    name: 'Fire Bet',
    description: 'Bonus for making unique points',
    odds: 'Progressive',
    houseEdge: 20.61,
    category: 'bonus',
    availablePhases: [GamePhase.COME_OUT]
  }
} as const;

// Bet categories for UI organization
export const BET_CATEGORIES = {
  main: {
    name: 'Main Bets',
    color: '#059669',
    bets: [BET_TYPES.PASS, BET_TYPES.DONT_PASS, BET_TYPES.COME, BET_TYPES.DONT_COME]
  },
  place: {
    name: 'Place Bets',
    color: '#DC2626',
    bets: [BET_TYPES.YES_4, BET_TYPES.YES_5, BET_TYPES.YES_6, BET_TYPES.YES_8, BET_TYPES.YES_9, BET_TYPES.YES_10]
  },
  hardway: {
    name: 'Hard Ways',
    color: '#EA580C',
    bets: [BET_TYPES.HARD4, BET_TYPES.HARD6, BET_TYPES.HARD8, BET_TYPES.HARD10]
  },
  'one-roll': {
    name: 'One Roll',
    color: '#7C3AED',
    bets: [BET_TYPES.FIELD, BET_TYPES.NEXT_2, BET_TYPES.NEXT_3, BET_TYPES.NEXT_7, BET_TYPES.NEXT_11, BET_TYPES.NEXT_12]
  },
  bonus: {
    name: 'Bonus Bets',
    color: '#FFD700',
    bets: [BET_TYPES.HOT_ROLLER, BET_TYPES.FIRE, BET_TYPES.BONUS_SMALL, BET_TYPES.BONUS_TALL]
  }
} as const;

// Game timing constants
export const GAME_TIMING = {
  BETTING_WINDOW_SLOTS: 40,      // 40 slots for betting
  COLLECTION_WINDOW_SLOTS: 10,   // 10 slots for hash collection
  AUTO_ROLL_INTERVAL_SLOTS: 50,  // Roll every 50 slots
  SLOT_TIME_MS: 400,             // ~400ms per slot
  
  // UI timeouts
  DICE_ANIMATION_MS: 2000,       // 2 second dice roll animation
  BET_FEEDBACK_MS: 500,          // 500ms for bet placement feedback
  NOTIFICATION_MS: 3000,         // 3 second notifications
  RECONNECT_DELAY_MS: 5000       // 5 second reconnect delay
} as const;

// Error codes from the program
export const CRAPS_ERRORS = {
  // System errors (0-99)
  INVALID_PDA: 17,
  UNAUTHORIZED_ACTION: 9,
  ACCOUNT_ALREADY_EXISTS: 8,
  ACCOUNT_NOT_FOUND: 6,
  
  // Game errors (100-199)  
  INVALID_BET_AMOUNT: 107,
  INVALID_BET_KIND: 108,
  INVALID_EPOCH: 111,
  BETTING_WINDOW_CLOSED: 117,
  INSUFFICIENT_BALANCE: 129,
  GAME_PAUSED: 105,
  MAX_BETS_REACHED: 142,
  
  // RNG errors (130+)
  INVALID_RNG_PHASE: 132,
  BETTING_STILL_OPEN: 143,
  HASH_COLLECTION_INCOMPLETE: 144
} as const;

// Error messages for user display
export const ERROR_MESSAGES = {
  [CRAPS_ERRORS.INVALID_BET_AMOUNT]: {
    title: 'Invalid Bet Amount',
    message: 'Bet amount must be between 1 and 100,000 CRAP tokens',
    action: 'ADJUST_BET'
  },
  [CRAPS_ERRORS.INVALID_BET_KIND]: {
    title: 'Invalid Bet Type',
    message: 'This bet type is not available in the current game phase',
    action: 'SELECT_DIFFERENT_BET'
  },
  [CRAPS_ERRORS.BETTING_WINDOW_CLOSED]: {
    title: 'Betting Closed',
    message: 'The betting window has closed. Wait for the next round.',
    action: 'WAIT_FOR_NEXT_ROUND'
  },
  [CRAPS_ERRORS.INSUFFICIENT_BALANCE]: {
    title: 'Insufficient Balance',
    message: 'You do not have enough CRAP tokens for this bet',
    action: 'DEPOSIT_MORE'
  },
  [CRAPS_ERRORS.GAME_PAUSED]: {
    title: 'Game Paused',
    message: 'The game is temporarily paused by the authority',
    action: 'WAIT_FOR_RESUME'
  },
  [CRAPS_ERRORS.MAX_BETS_REACHED]: {
    title: 'Maximum Bets',
    message: 'You can only place up to 16 bets per round',
    action: 'REDUCE_BETS'
  }
} as const;

// UI layout constants
export const UI_CONSTANTS = {
  // Table dimensions
  TABLE_ASPECT_RATIO: 1.5,      // Height = Width * 1.5
  MIN_TOUCH_TARGET: 44,         // 44px minimum touch target
  TABLE_PADDING: 10,            // 10px padding around table
  
  // Chip values and colors
  CHIP_VALUES: [1, 5, 10, 25, 50, 100, 250, 500, 1000, 2500, 5000, 10000],
  CHIP_COLORS: {
    1: '#FFFFFF',     // White
    5: '#DC2626',     // Red  
    10: '#1E40AF',    // Blue
    25: '#059669',    // Green
    50: '#EA580C',    // Orange
    100: '#000000',   // Black
    250: '#7C3AED',   // Purple
    500: '#F59E0B',   // Yellow
    1000: '#EC4899',  // Pink
    2500: '#06B6D4',  // Cyan
    5000: '#8B5CF6',  // Violet
    10000: '#FFD700'  // Gold
  },
  
  // Animation durations
  ANIMATIONS: {
    BET_PLACEMENT: 300,
    CHIP_DROP: 400,
    DICE_ROLL: 2000,
    WIN_CELEBRATION: 3000,
    LOSS_FADE: 1000,
    NOTIFICATION: 300
  }
} as const;

// Staking tiers and benefits
export const STAKING_TIERS = [
  {
    minStake: 1_000,
    name: 'Bronze',
    color: '#CD7F32',
    benefits: {
      cashbackPercent: 0.001,    // 0.1% cashback on losses
      bonusMultiplier: 1.05,     // 5% bonus on wins
      freeRollsDaily: 1,
      prioritySupport: false
    }
  },
  {
    minStake: 10_000,
    name: 'Silver',
    color: '#C0C0C0',
    benefits: {
      cashbackPercent: 0.0025,   // 0.25% cashback
      bonusMultiplier: 1.10,     // 10% bonus on wins
      freeRollsDaily: 3,
      prioritySupport: false
    }
  },
  {
    minStake: 100_000,
    name: 'Gold',
    color: '#FFD700',
    benefits: {
      cashbackPercent: 0.005,    // 0.5% cashback
      bonusMultiplier: 1.20,     // 20% bonus on wins
      freeRollsDaily: 5,
      prioritySupport: true
    }
  },
  {
    minStake: 1_000_000,
    name: 'Diamond',
    color: '#B9F2FF',
    benefits: {
      cashbackPercent: 0.0075,   // 0.75% cashback
      bonusMultiplier: 1.30,     // 30% bonus on wins
      freeRollsDaily: 10,
      prioritySupport: true,
      customLimits: true
    }
  }
] as const;

// Token economics
export const TOKEN_ECONOMICS = {
  TOTAL_SUPPLY: 1_000_000_000_000, // 1 trillion tokens
  
  ALLOCATION: {
    GAMEPLAY: 400_000_000_000,      // 40% for player rewards
    LIQUIDITY: 100_000_000_000,     // 10% for DEX liquidity  
    BELIEVERS: 250_000_000_000,     // 25% for early supporters
    AIRDROP: 100_000_000_000,       // 10% for community
    TEAM: 150_000_000_000           // 15% for team (vested)
  },
  
  FEES: {
    HOUSE_EDGE: 0.025,              // 2.5% house edge
    WITHDRAWAL_FEE: 0.01,           // 1% withdrawal fee
    REFERRAL_BONUS: 0.005,          // 0.5% referral bonus
    STAKING_APY: 0.12               // 12% staking APY
  },
  
  BONUS_DISTRIBUTION: {
    WIN_MULTIPLIER: 1.0,            // Base payout
    DICE_BONUS_PERCENT: 0.1,        // 10% bonus in DICE tokens
    BIG_WIN_THRESHOLD: 1000,        // 1000+ CRAP wins get DICE
    JACKPOT_CONTRIBUTION: 0.001     // 0.1% to progressive jackpot
  }
} as const;

// Development environment configuration
export const ENV_CONFIG = {
  DEVELOPMENT: {
    RPC_ENDPOINT: 'https://api.devnet.solana.com',
    ENABLE_LOGS: true,
    MOCK_TRANSACTIONS: false,
    BYPASS_WALLET_CHECKS: false
  },
  
  STAGING: {
    RPC_ENDPOINT: `https://mainnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`,
    ENABLE_LOGS: true,
    MOCK_TRANSACTIONS: false,
    BYPASS_WALLET_CHECKS: false
  },
  
  PRODUCTION: {
    RPC_ENDPOINT: `https://mainnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`,
    ENABLE_LOGS: false,
    MOCK_TRANSACTIONS: false,
    BYPASS_WALLET_CHECKS: false
  }
} as const;

// Performance and monitoring
export const PERFORMANCE_TARGETS = {
  APP_LAUNCH_TIME_MS: 2000,        // 2 second max launch
  TRANSACTION_TIMEOUT_MS: 30000,   // 30 second transaction timeout
  RPC_TIMEOUT_MS: 10000,           // 10 second RPC timeout
  MAX_MEMORY_MB: 200,              // 200MB memory limit
  MIN_FPS: 30,                     // 30 FPS minimum
  MAX_BUNDLE_SIZE_MB: 50           // 50MB APK size limit
} as const;
```

## Usage Examples

### Bet Type Validation
```typescript
import { BET_TYPES, BET_INFO, GamePhase } from '@/constants/crapsConstants';

function isBetAvailable(betType: number, currentPhase: GamePhase): boolean {
  const betInfo = BET_INFO[betType];
  return betInfo?.availablePhases.includes(currentPhase) ?? false;
}

// Example: Check if Come bet is available
const canPlaceCome = isBetAvailable(BET_TYPES.COME, GamePhase.POINT); // true
const canPlacePassInPoint = isBetAvailable(BET_TYPES.PASS, GamePhase.POINT); // false
```

### Error Handling
```typescript
import { CRAPS_ERRORS, ERROR_MESSAGES } from '@/constants/crapsConstants';

function handleProgramError(error: any): UserErrorMessage {
  // Extract error code from transaction logs
  const errorCode = extractErrorCodeFromLogs(error.logs);
  
  return ERROR_MESSAGES[errorCode] || {
    title: 'Unknown Error',
    message: 'An unexpected error occurred',
    action: 'RETRY'
  };
}
```

### Staking Tier Calculation  
```typescript
import { STAKING_TIERS } from '@/constants/crapsConstants';

function getStakingTier(stakedAmount: number) {
  for (let i = STAKING_TIERS.length - 1; i >= 0; i--) {
    if (stakedAmount >= STAKING_TIERS[i].minStake) {
      return STAKING_TIERS[i];
    }
  }
  return null; // No tier qualification
}

// Example: Get benefits for 50,000 CRAP staked
const tier = getStakingTier(50_000); // Returns Silver tier
const cashback = tier?.benefits.cashbackPercent; // 0.0025 (0.25%)
```