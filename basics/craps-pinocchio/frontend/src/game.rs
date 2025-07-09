use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Signature};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    ComeOut,
    Point,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BetType {
    Pass,
    DontPass,
    Come,
    DontCome,
    Field,
    Number(u8),
    Hard4,
    Hard6,
    Hard8,
    Hard10,
    Odds(Box<BetType>),
    Repeater(u8),
}

impl fmt::Display for BetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetType::Pass => write!(f, "Pass Line"),
            BetType::DontPass => write!(f, "Don't Pass"),
            BetType::Come => write!(f, "Come"),
            BetType::DontCome => write!(f, "Don't Come"),
            BetType::Field => write!(f, "Field"),
            BetType::Number(n) => write!(f, "Number {}", n),
            BetType::Hard4 => write!(f, "Hard 4"),
            BetType::Hard6 => write!(f, "Hard 6"),
            BetType::Hard8 => write!(f, "Hard 8"),
            BetType::Hard10 => write!(f, "Hard 10"),
            BetType::Odds(bet) => write!(f, "Odds on {}", bet),
            BetType::Repeater(n) => write!(f, "Repeater {}", n),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub epoch: u64,
    pub phase: GamePhase,
    pub point: Option<u8>,
    pub die1: Option<u8>,
    pub die2: Option<u8>,
    pub next_roll_slot: u64,
    pub active_bets: u64,
    pub treasury_balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedBet {
    pub bet_type: BetType,
    pub amount: u64,
    pub epoch: u64,
    pub signature: Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetOutcome {
    pub bet: PlacedBet,
    pub won: bool,
    pub payout: u64,
    pub claimed: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            epoch: 0,
            phase: GamePhase::ComeOut,
            point: None,
            die1: None,
            die2: None,
            next_roll_slot: 0,
            active_bets: 0,
            treasury_balance: 0,
        }
    }
    
    pub fn dice_total(&self) -> Option<u8> {
        match (self.die1, self.die2) {
            (Some(d1), Some(d2)) => Some(d1 + d2),
            _ => None,
        }
    }
    
    pub fn is_natural(&self) -> bool {
        matches!(self.dice_total(), Some(7) | Some(11))
    }
    
    pub fn is_craps(&self) -> bool {
        matches!(self.dice_total(), Some(2) | Some(3) | Some(12))
    }
    
    pub fn is_point_number(n: u8) -> bool {
        matches!(n, 4 | 5 | 6 | 8 | 9 | 10)
    }
}

// Bet constants from the program
pub const BET_PASS: u8 = 0;
pub const BET_DONT_PASS: u8 = 1;
pub const BET_COME: u8 = 2;
pub const BET_DONT_COME: u8 = 3;
pub const BET_FIELD: u8 = 4;

pub const BET_YES_4: u8 = 7;
pub const BET_YES_5: u8 = 8;
pub const BET_YES_6: u8 = 9;
pub const BET_YES_8: u8 = 10;
pub const BET_YES_9: u8 = 11;
pub const BET_YES_10: u8 = 12;

pub const BET_HARD4: u8 = 25;
pub const BET_HARD6: u8 = 26;
pub const BET_HARD8: u8 = 27;
pub const BET_HARD10: u8 = 28;

pub const BET_ODDS_PASS: u8 = 29;
pub const BET_ODDS_DONT_PASS: u8 = 30;
pub const BET_ODDS_COME: u8 = 31;
pub const BET_ODDS_DONT_COME: u8 = 32;

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

impl BetType {
    pub fn to_program_bet_type(&self) -> u8 {
        match self {
            BetType::Pass => BET_PASS,
            BetType::DontPass => BET_DONT_PASS,
            BetType::Come => BET_COME,
            BetType::DontCome => BET_DONT_COME,
            BetType::Field => BET_FIELD,
            BetType::Number(4) => BET_YES_4,
            BetType::Number(5) => BET_YES_5,
            BetType::Number(6) => BET_YES_6,
            BetType::Number(8) => BET_YES_8,
            BetType::Number(9) => BET_YES_9,
            BetType::Number(10) => BET_YES_10,
            BetType::Hard4 => BET_HARD4,
            BetType::Hard6 => BET_HARD6,
            BetType::Hard8 => BET_HARD8,
            BetType::Hard10 => BET_HARD10,
            BetType::Odds(bet) => match **bet {
                BetType::Pass => BET_ODDS_PASS,
                BetType::DontPass => BET_ODDS_DONT_PASS,
                BetType::Come => BET_ODDS_COME,
                BetType::DontCome => BET_ODDS_DONT_COME,
                _ => panic!("Invalid odds bet type"),
            },
            BetType::Repeater(2) => BET_REPEATER_2,
            BetType::Repeater(3) => BET_REPEATER_3,
            BetType::Repeater(4) => BET_REPEATER_4,
            BetType::Repeater(5) => BET_REPEATER_5,
            BetType::Repeater(6) => BET_REPEATER_6,
            BetType::Repeater(8) => BET_REPEATER_8,
            BetType::Repeater(9) => BET_REPEATER_9,
            BetType::Repeater(10) => BET_REPEATER_10,
            BetType::Repeater(11) => BET_REPEATER_11,
            BetType::Repeater(12) => BET_REPEATER_12,
            _ => panic!("Invalid bet type"),
        }
    }
}