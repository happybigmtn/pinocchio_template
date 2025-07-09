//! Constants for the craps-pinocchio program

// ===== PDA SEEDS =====
pub const CRAPS_PINOCCHIO_SEED: &[u8] = b"craps_pinocchio";
pub const GLOBAL_GAME_STATE_SEED: &[u8] = b"global_game_state";
pub const TREASURY_SEED: &[u8] = b"treasury";
pub const AUTO_ROLL_TIMER_SEED: &[u8] = b"auto_roll_timer";
pub const BONUS_STATE_SEED: &[u8] = b"bonus_state";
pub const SCALABLE_PLAYER_SEED: &[u8] = b"scalable_player";
pub const BET_BATCH_SEED: &[u8] = b"bet_batch";
pub const RATE_LIMIT_SEED: &[u8] = b"rate_limit";
pub const EPOCH_OUTCOME_SEED: &[u8] = b"epoch_outcome";
pub const RNG_STATE_SEED: &[u8] = b"rng_state";
pub const AUTHORITY_CONFIG_SEED: &[u8] = b"authority_config";
pub const PLAYER_EPOCH_TRACKER_SEED: &[u8] = b"player_epoch_tracker";

// ===== BET TYPE CONSTANTS =====
// Core game bets
pub const BET_PASS: u8 = 0;
pub const BET_DONT_PASS: u8 = 1;
pub const BET_COME: u8 = 2;
pub const BET_DONT_COME: u8 = 3;
pub const BET_FIELD: u8 = 4;

// YES bets - number will be rolled before 7
pub const BET_YES_2: u8 = 5;
pub const BET_YES_3: u8 = 6;
pub const BET_YES_4: u8 = 7;
pub const BET_YES_5: u8 = 8;
pub const BET_YES_6: u8 = 9;
pub const BET_YES_8: u8 = 10;
pub const BET_YES_9: u8 = 11;
pub const BET_YES_10: u8 = 12;
pub const BET_YES_11: u8 = 13;
pub const BET_YES_12: u8 = 14;

// NO bets - 7 will be rolled before the number
pub const BET_NO_2: u8 = 15;
pub const BET_NO_3: u8 = 16;
pub const BET_NO_4: u8 = 17;
pub const BET_NO_5: u8 = 18;
pub const BET_NO_6: u8 = 19;
pub const BET_NO_8: u8 = 20;
pub const BET_NO_9: u8 = 21;
pub const BET_NO_10: u8 = 22;
pub const BET_NO_11: u8 = 23;
pub const BET_NO_12: u8 = 24;

// Hard ways bets
pub const BET_HARD4: u8 = 25;
pub const BET_HARD6: u8 = 26;
pub const BET_HARD8: u8 = 27;
pub const BET_HARD10: u8 = 28;

// Odds bets
pub const BET_ODDS_PASS: u8 = 29;
pub const BET_ODDS_DONT_PASS: u8 = 30;
pub const BET_ODDS_COME: u8 = 31;
pub const BET_ODDS_DONT_COME: u8 = 32;

// Complex/special bets
pub const BET_HOT_ROLLER: u8 = 33;
pub const BET_FIRE: u8 = 34;
pub const BET_TWICE_HARD: u8 = 35;
pub const BET_RIDE_LINE: u8 = 36;
pub const BET_MUGGSY: u8 = 37;
pub const BET_BONUS_SMALL: u8 = 38;
pub const BET_BONUS_TALL: u8 = 39;
pub const BET_BONUS_SMALL_TALL: u8 = 40;
pub const BET_REPLAY: u8 = 41;
pub const BET_DIFFERENT_DOUBLES: u8 = 42;

// NEXT bets - one-roll bets
pub const BET_NEXT_2: u8 = 43;
pub const BET_NEXT_3: u8 = 44;
pub const BET_NEXT_4: u8 = 45;
pub const BET_NEXT_5: u8 = 46;
pub const BET_NEXT_6: u8 = 47;
pub const BET_NEXT_7: u8 = 48;
pub const BET_NEXT_8: u8 = 49;
pub const BET_NEXT_9: u8 = 50;
pub const BET_NEXT_10: u8 = 51;
pub const BET_NEXT_11: u8 = 52;
pub const BET_NEXT_12: u8 = 53;

// Repeater bets
pub const BET_REPEATER_2: u8 = 54;
pub const BET_REPEATER_3: u8 = 55;
pub const BET_REPEATER_4: u8 = 56;
pub const BET_REPEATER_5: u8 = 57;
pub const BET_REPEATER_6: u8 = 58;
pub const BET_REPEATER_8: u8 = 59;
pub const BET_REPEATER_9: u8 = 60;
pub const BET_REPEATER_10: u8 = 61;
pub const BET_REPEATER_11: u8 = 62;
pub const BET_REPEATER_12: u8 = 63;

// ===== GAME PHASE CONSTANTS =====
pub const PHASE_COME_OUT: u8 = 0;
pub const PHASE_POINT: u8 = 1;

// ===== GAME CONSTANTS =====
pub const TOKEN_DECIMALS: u64 = 1_000_000_000; // 9 decimals for $CRAP tokens
pub const DICE_SIDES: u8 = 6;
pub const DICE_MIN_VALUE: u8 = 1;
pub const DICE_MIN_SUM: u8 = 2;
pub const DICE_MAX_SUM: u8 = 12;
pub const NATURAL_SEVEN: u8 = 7;
pub const NATURAL_ELEVEN: u8 = 11;
pub const CRAPS_TWO: u8 = 2;
pub const CRAPS_THREE: u8 = 3;
pub const CRAPS_TWELVE: u8 = 12;

// Valid point numbers
pub const VALID_POINTS: [u8; 6] = [4, 5, 6, 8, 9, 10];

// ===== LIMITS =====
pub const MAX_BET_AMOUNT: u64 = 10_000_000_000_000u64; // 10k tokens max bet
pub const MIN_BET_AMOUNT: u64 = 1_000_000; // 0.001 tokens (6 decimals)
pub const MAX_DEPOSIT_AMOUNT: u64 = 1_000_000_000_000_000; // 1M tokens max deposit
pub const MAX_WITHDRAWAL_AMOUNT: u64 = 100_000_000_000_000; // 100k tokens max withdrawal per transaction
pub const DAILY_WITHDRAWAL_LIMIT: u64 = 1_000_000_000_000_000; // 1M tokens max withdrawal per day
pub const DAILY_CLAIM_SLOTS: u64 = 172_800; // 24 hours worth of slots
pub const MAX_AUTO_CLAIM_LOOKBACK: u64 = 50; // Look back up to 50 epochs for auto-claims

// ===== RNG CONSTANTS =====
pub const MAX_BLOCK_HASHES: u8 = 15;  // Increased from 10 to allow buffer
pub const AUTO_ROLL_INTERVAL: u64 = 50; // ~20-25 seconds with 0.4-0.5s slots
pub const BETTING_WINDOW_SLOTS: u64 = 40;
pub const REQUIRED_BLOCK_HASHES: u8 = 10;  // Increased from 5 to 10 for better security
pub const SLOTS_PER_ROLL: u64 = 60; // Time between dice rolls

// ===== TREASURY CONSTANTS =====
pub const TREASURY_SAFETY_MULTIPLIER: u64 = 2;
pub const MIN_TREASURY_BALANCE: u64 = 100_000_000_000; // 100 tokens
pub const TREASURY_RESERVE_PERCENTAGE: u8 = 5;
pub const HOUSE_COMMISSION_BPS: u16 = 250; // 2.5% = 250 basis points

// ===== CIRCUIT BREAKER CONSTANTS =====
pub const MAX_PAYOUT_RATIO: u8 = 80; // Max 80% of treasury can be paid out
pub const MAX_SINGLE_PAYOUT: u64 = 50_000_000_000_000; // 50k tokens max single payout
pub const MAX_HOURLY_PAYOUTS: u64 = 500_000_000_000_000; // 500k tokens max hourly payouts
pub const MAX_DEPOSITS_PER_HOUR: u64 = 1_000_000_000_000_000; // 1M tokens max hourly deposits
pub const MAX_WITHDRAWALS_PER_HOUR: u64 = 200_000_000_000_000; // 200k tokens max hourly withdrawals
pub const HOURLY_SLOTS: u64 = 7200; // 1 hour = 3600 seconds / 0.5s per slot
pub const EMERGENCY_RESERVE_RATIO: u8 = 20; // 20% of treasury reserved for emergencies
pub const LIQUIDITY_THRESHOLD: u8 = 90; // Trigger warnings at 90% treasury utilization

// ===== BATCH CONSTANTS =====
pub const MAX_BETS_PER_BATCH: usize = 16;
pub const MAX_ACTIVE_BATCHES: usize = 10;

// ===== FINANCIAL CONSTANTS =====
pub const BASIS_POINTS_DIVISOR: u64 = 10000;
pub const PERCENTAGE_MAX: u8 = 100;
