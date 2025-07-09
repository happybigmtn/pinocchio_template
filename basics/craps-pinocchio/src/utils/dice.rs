//! Dice-related utilities for craps game logic

use crate::constants::*;

/// Check if a roll is a seven-out (total equals 7)
pub fn is_seven_out(die1: u8, die2: u8) -> bool {
    calculate_roll_total(die1, die2) == NATURAL_SEVEN
}

/// Check if a roll is craps (2, 3, or 12)
pub fn is_craps(die1: u8, die2: u8) -> bool {
    let total = calculate_roll_total(die1, die2);
    total == CRAPS_TWO || total == CRAPS_THREE || total == CRAPS_TWELVE
}

/// Check if a roll matches the current point
pub fn is_point_made(die1: u8, die2: u8, point: u8) -> bool {
    point != 0 && calculate_roll_total(die1, die2) == point
}

/// Calculate the total of two dice
pub fn calculate_roll_total(die1: u8, die2: u8) -> u8 {
    die1 + die2
}

/// Check if a roll is a natural (7 or 11)
pub fn is_natural(die1: u8, die2: u8) -> bool {
    let total = calculate_roll_total(die1, die2);
    total == NATURAL_SEVEN || total == NATURAL_ELEVEN
}

/// Check if dice form a hard way (both dice show the same value)
pub fn is_hard_way(die1: u8, die2: u8) -> bool {
    die1 == die2 && die1 >= 2 && die1 <= 5
}

/// Check if a specific hard way was rolled
pub fn is_specific_hard_way(die1: u8, die2: u8, target: u8) -> bool {
    die1 == die2 && calculate_roll_total(die1, die2) == target
}

/// Check if a roll is a valid point number (4, 5, 6, 8, 9, 10)
pub fn is_valid_point_roll(die1: u8, die2: u8) -> bool {
    let total = calculate_roll_total(die1, die2);
    VALID_POINTS.contains(&total)
}

/// Get the type of roll for logging/display purposes
pub fn get_roll_type(die1: u8, die2: u8) -> &'static str {
    let total = calculate_roll_total(die1, die2);
    
    match total {
        2 => "Snake Eyes",
        3 => "Ace Deuce",
        7 => {
            if die1 == die2 {
                "Hard Seven" // Impossible but included for completeness
            } else {
                "Natural Seven"
            }
        }
        11 => "Yo-leven",
        12 => "Boxcars",
        4 => {
            if die1 == die2 {
                "Hard Four"
            } else {
                "Easy Four"
            }
        }
        6 => {
            if die1 == die2 {
                "Hard Six"
            } else {
                "Easy Six"
            }
        }
        8 => {
            if die1 == die2 {
                "Hard Eight"
            } else {
                "Easy Eight"
            }
        }
        10 => {
            if die1 == die2 {
                "Hard Ten"
            } else {
                "Easy Ten"
            }
        }
        _ => "Roll"
    }
}

/// Check if a field bet wins (2, 3, 4, 9, 10, 11, 12)
pub fn is_field_winner(die1: u8, die2: u8) -> bool {
    let total = calculate_roll_total(die1, die2);
    matches!(total, 2 | 3 | 4 | 9 | 10 | 11 | 12)
}

/// Get field bet multiplier (2 and 12 typically pay double or triple)
pub fn get_field_multiplier(die1: u8, die2: u8) -> u8 {
    let total = calculate_roll_total(die1, die2);
    match total {
        2 => 2,  // Snake eyes pays double
        12 => 3, // Boxcars pays triple
        _ => 1,  // All other field numbers pay even money
    }
}

/// Check if a big 6/8 bet wins
pub fn is_big_six_eight_winner(die1: u8, die2: u8, target: u8) -> bool {
    let total = calculate_roll_total(die1, die2);
    total == target && (target == 6 || target == 8)
}

/// Calculate the number of ways to roll a specific total
pub fn ways_to_roll(total: u8) -> u8 {
    match total {
        2 | 12 => 1,  // Only one way: 1+1 or 6+6
        3 | 11 => 2,  // Two ways: 1+2, 2+1 or 5+6, 6+5
        4 | 10 => 3,  // Three ways: 1+3, 2+2, 3+1 or 4+6, 5+5, 6+4
        5 | 9 => 4,   // Four ways
        6 | 8 => 5,   // Five ways
        7 => 6,       // Six ways
        _ => 0,       // Invalid total
    }
}

/// Get the true odds for a pass line odds bet based on the point
pub fn get_pass_odds_payout(point: u8) -> (u64, u64) {
    match point {
        4 | 10 => (2, 1),  // Pays 2:1
        5 | 9 => (3, 2),   // Pays 3:2
        6 | 8 => (6, 5),   // Pays 6:5
        _ => (0, 0),       // Invalid point
    }
}

/// Get the true odds for a don't pass odds bet based on the point
pub fn get_dont_pass_odds_payout(point: u8) -> (u64, u64) {
    match point {
        4 | 10 => (1, 2),  // Pays 1:2
        5 | 9 => (2, 3),   // Pays 2:3
        6 | 8 => (5, 6),   // Pays 5:6
        _ => (0, 0),       // Invalid point
    }
}

/// Check if dice values are valid (1-6 each)
pub fn are_dice_valid(die1: u8, die2: u8) -> bool {
    die1 >= DICE_MIN_VALUE && die1 <= DICE_SIDES &&
    die2 >= DICE_MIN_VALUE && die2 <= DICE_SIDES
}

/// Get dice combination as a unique identifier (for tracking purposes)
/// Returns a value where high nibble is die1 and low nibble is die2
pub fn get_dice_combination_id(die1: u8, die2: u8) -> u8 {
    (die1 << 4) | die2
}

/// Extract dice values from combination ID
pub fn extract_dice_from_combination_id(combination_id: u8) -> (u8, u8) {
    let die1 = (combination_id >> 4) & 0x0F;
    let die2 = combination_id & 0x0F;
    (die1, die2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_calculations() {
        assert_eq!(calculate_roll_total(3, 4), 7);
        assert_eq!(calculate_roll_total(6, 6), 12);
        assert_eq!(calculate_roll_total(1, 1), 2);
    }

    #[test]
    fn test_seven_out() {
        assert!(is_seven_out(3, 4));
        assert!(is_seven_out(1, 6));
        assert!(!is_seven_out(3, 3));
    }

    #[test]
    fn test_craps() {
        assert!(is_craps(1, 1)); // 2
        assert!(is_craps(1, 2)); // 3
        assert!(is_craps(6, 6)); // 12
        assert!(!is_craps(3, 4)); // 7
    }

    #[test]
    fn test_natural() {
        assert!(is_natural(3, 4)); // 7
        assert!(is_natural(5, 6)); // 11
        assert!(!is_natural(3, 3)); // 6
    }

    #[test]
    fn test_hard_ways() {
        assert!(is_hard_way(2, 2)); // Hard 4
        assert!(is_hard_way(3, 3)); // Hard 6
        assert!(is_hard_way(4, 4)); // Hard 8
        assert!(is_hard_way(5, 5)); // Hard 10
        assert!(!is_hard_way(2, 3)); // Easy 5
        assert!(!is_hard_way(1, 1)); // Not a hard way (2)
        assert!(!is_hard_way(6, 6)); // Not a hard way (12)
    }

    #[test]
    fn test_field_bet() {
        assert!(is_field_winner(1, 1)); // 2
        assert!(is_field_winner(5, 6)); // 11
        assert!(!is_field_winner(3, 4)); // 7
        assert!(!is_field_winner(2, 3)); // 5
    }

    #[test]
    fn test_point_validation() {
        assert!(is_valid_point_roll(2, 2)); // 4
        assert!(is_valid_point_roll(4, 6)); // 10
        assert!(!is_valid_point_roll(1, 1)); // 2
        assert!(!is_valid_point_roll(3, 4)); // 7
    }

    #[test]
    fn test_dice_combination_id() {
        let id = get_dice_combination_id(3, 4);
        let (die1, die2) = extract_dice_from_combination_id(id);
        assert_eq!(die1, 3);
        assert_eq!(die2, 4);
    }
}