//! Event emission for off-chain tracking
//! 
//! This module provides event structures and emission functions for tracking
//! game activity off-chain. Events are emitted as program logs that can be
//! parsed by indexers and analytics systems.

use pinocchio_log::log;
use bytemuck::{Pod, Zeroable};

/// Event discriminator to identify event types in logs
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    BetPlaced = 1,
    DiceRolled = 2,
    PayoutClaimed = 3,
    PlayerInitialized = 4,
    DepositMade = 5,
    WithdrawalMade = 6,
    EpochStarted = 7,
    EpochEnded = 8,
    EmergencyAction = 9,
    AuthorityChanged = 10,
}

/// Base event header included in all events
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct EventHeader {
    /// Event type discriminator
    pub event_type: u8,
    /// Unix timestamp
    pub timestamp: [u8; 8],
    /// Current epoch
    pub epoch: [u8; 8],
    /// Slot number
    pub slot: [u8; 8],
}

/// Bet placed event
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BetPlacedEvent {
    /// Event header
    pub header: EventHeader,
    /// Player public key
    pub player: [u8; 32],
    /// Bet type (0-63)
    pub bet_type: u8,
    /// Bet amount in lamports
    pub amount: [u8; 8],
    /// Batch index
    pub batch_index: [u8; 2],
    /// Padding
    pub _padding: [u8; 5],
}

/// Dice rolled event
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct DiceRolledEvent {
    /// Event header
    pub header: EventHeader,
    /// First die value (1-6)
    pub die1: u8,
    /// Second die value (1-6)
    pub die2: u8,
    /// Total (2-12)
    pub total: u8,
    /// Game phase after roll
    pub new_phase: u8,
    /// Point value (0 if come out)
    pub point: u8,
    /// Whether this was a seven out
    pub seven_out: u8,
    /// Padding
    pub _padding: [u8; 2],
}

/// Payout claimed event
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PayoutClaimedEvent {
    /// Event header
    pub header: EventHeader,
    /// Player public key
    pub player: [u8; 32],
    /// Total payout amount
    pub payout_amount: [u8; 8],
    /// Number of winning bets
    pub winning_bets: [u8; 2],
    /// Number of losing bets
    pub losing_bets: [u8; 2],
    /// Epoch claimed for
    pub claim_epoch: [u8; 8],
    /// Padding
    pub _padding: [u8; 4],
}

/// Deposit event
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct DepositEvent {
    /// Event header
    pub header: EventHeader,
    /// Player public key
    pub player: [u8; 32],
    /// Deposit amount
    pub amount: [u8; 8],
    /// New balance
    pub new_balance: [u8; 8],
    /// Whether auto-claim was triggered
    pub auto_claimed: u8,
    /// Padding
    pub _padding: [u8; 7],
}

/// Withdrawal event
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WithdrawalEvent {
    /// Event header
    pub header: EventHeader,
    /// Player public key
    pub player: [u8; 32],
    /// Withdrawal amount
    pub amount: [u8; 8],
    /// New balance
    pub new_balance: [u8; 8],
    /// Whether auto-claim was triggered
    pub auto_claimed: u8,
    /// Padding
    pub _padding: [u8; 7],
}

/// Emergency action event
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct EmergencyActionEvent {
    /// Event header
    pub header: EventHeader,
    /// Authority that triggered action
    pub authority: [u8; 32],
    /// Action type (0=shutdown, 1=resume, 2=pause_game, 3=resume_game)
    pub action: u8,
    /// Previous state
    pub prev_state: u8,
    /// New state
    pub new_state: u8,
    /// Padding
    pub _padding: [u8; 5],
}

/// Helper macro to emit events
#[macro_export]
macro_rules! emit_event {
    ($event:expr) => {
        {
            let bytes = bytemuck::bytes_of(&$event);
            let event_str = bs58::encode(bytes).into_string();
            pinocchio_log::log!("EVENT:{}", event_str.as_str());
        }
    };
}

/// Emit a bet placed event
pub fn emit_bet_placed(
    player: &[u8; 32],
    bet_type: u8,
    amount: u64,
    batch_index: u16,
    epoch: u64,
    slot: u64,
    timestamp: i64,
) {
    let event = BetPlacedEvent {
        header: EventHeader {
            event_type: EventType::BetPlaced as u8,
            timestamp: timestamp.to_le_bytes(),
            epoch: epoch.to_le_bytes(),
            slot: slot.to_le_bytes(),
        },
        player: *player,
        bet_type,
        amount: amount.to_le_bytes(),
        batch_index: batch_index.to_le_bytes(),
        _padding: [0; 5],
    };
    
    emit_event!(event);
}

/// Emit a dice rolled event
pub fn emit_dice_rolled(
    die1: u8,
    die2: u8,
    new_phase: u8,
    point: u8,
    seven_out: bool,
    epoch: u64,
    slot: u64,
    timestamp: i64,
) {
    let event = DiceRolledEvent {
        header: EventHeader {
            event_type: EventType::DiceRolled as u8,
            timestamp: timestamp.to_le_bytes(),
            epoch: epoch.to_le_bytes(),
            slot: slot.to_le_bytes(),
        },
        die1,
        die2,
        total: die1 + die2,
        new_phase,
        point,
        seven_out: if seven_out { 1 } else { 0 },
        _padding: [0; 2],
    };
    
    emit_event!(event);
}

/// Emit a payout claimed event
pub fn emit_payout_claimed(
    player: &[u8; 32],
    payout_amount: u64,
    winning_bets: u16,
    losing_bets: u16,
    claim_epoch: u64,
    current_epoch: u64,
    slot: u64,
    timestamp: i64,
) {
    let event = PayoutClaimedEvent {
        header: EventHeader {
            event_type: EventType::PayoutClaimed as u8,
            timestamp: timestamp.to_le_bytes(),
            epoch: current_epoch.to_le_bytes(),
            slot: slot.to_le_bytes(),
        },
        player: *player,
        payout_amount: payout_amount.to_le_bytes(),
        winning_bets: winning_bets.to_le_bytes(),
        losing_bets: losing_bets.to_le_bytes(),
        claim_epoch: claim_epoch.to_le_bytes(),
        _padding: [0; 4],
    };
    
    emit_event!(event);
}

/// Emit a deposit event
pub fn emit_deposit(
    player: &[u8; 32],
    amount: u64,
    new_balance: u64,
    auto_claimed: bool,
    epoch: u64,
    slot: u64,
    timestamp: i64,
) {
    let event = DepositEvent {
        header: EventHeader {
            event_type: EventType::DepositMade as u8,
            timestamp: timestamp.to_le_bytes(),
            epoch: epoch.to_le_bytes(),
            slot: slot.to_le_bytes(),
        },
        player: *player,
        amount: amount.to_le_bytes(),
        new_balance: new_balance.to_le_bytes(),
        auto_claimed: if auto_claimed { 1 } else { 0 },
        _padding: [0; 7],
    };
    
    emit_event!(event);
}

/// Emit a withdrawal event
pub fn emit_withdrawal(
    player: &[u8; 32],
    amount: u64,
    new_balance: u64,
    auto_claimed: bool,
    epoch: u64,
    slot: u64,
    timestamp: i64,
) {
    let event = WithdrawalEvent {
        header: EventHeader {
            event_type: EventType::WithdrawalMade as u8,
            timestamp: timestamp.to_le_bytes(),
            epoch: epoch.to_le_bytes(),
            slot: slot.to_le_bytes(),
        },
        player: *player,
        amount: amount.to_le_bytes(),
        new_balance: new_balance.to_le_bytes(),
        auto_claimed: if auto_claimed { 1 } else { 0 },
        _padding: [0; 7],
    };
    
    emit_event!(event);
}

/// Emit an emergency action event
pub fn emit_emergency_action(
    authority: &[u8; 32],
    action: u8,
    prev_state: u8,
    new_state: u8,
    epoch: u64,
    slot: u64,
    timestamp: i64,
) {
    let event = EmergencyActionEvent {
        header: EventHeader {
            event_type: EventType::EmergencyAction as u8,
            timestamp: timestamp.to_le_bytes(),
            epoch: epoch.to_le_bytes(),
            slot: slot.to_le_bytes(),
        },
        authority: *authority,
        action,
        prev_state,
        new_state,
        _padding: [0; 5],
    };
    
    emit_event!(event);
}

/// Emit a simple log event for basic tracking
pub fn emit_simple_event(event_type: EventType, message: &str) {
    log!("EVENT_{}:{}", event_type as u8, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_sizes() {
        // Ensure events are reasonably sized for logging (under 256 bytes)
        assert!(core::mem::size_of::<BetPlacedEvent>() < 256);
        assert!(core::mem::size_of::<DiceRolledEvent>() < 256);
        assert!(core::mem::size_of::<PayoutClaimedEvent>() < 256);
        assert!(core::mem::size_of::<DepositEvent>() < 256);
        assert!(core::mem::size_of::<WithdrawalEvent>() < 256);
        assert!(core::mem::size_of::<EmergencyActionEvent>() < 256);
    }

    #[test]
    fn test_event_header() {
        let header = EventHeader {
            event_type: EventType::BetPlaced as u8,
            timestamp: 1234567890i64.to_le_bytes(),
            epoch: 100u64.to_le_bytes(),
            slot: 50000u64.to_le_bytes(),
        };
        
        assert_eq!(header.event_type, 1);
        assert_eq!(i64::from_le_bytes(header.timestamp), 1234567890);
        assert_eq!(u64::from_le_bytes(header.epoch), 100);
        assert_eq!(u64::from_le_bytes(header.slot), 50000);
    }
}