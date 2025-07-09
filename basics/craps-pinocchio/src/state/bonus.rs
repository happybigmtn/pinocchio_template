use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// Bonus state for tracking complex multi-roll bets
/// Matches the structure from craps-anchor for consistency
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct BonusState {
    /// Bit-packed tracking for space efficiency
    /// Bits 0-4: numbers 2,3,4,5,6 (bit 0 = 2, bit 4 = 6)
    pub small_rolled: u8,
    
    /// Bits 0-4: numbers 8,9,10,11,12 (bit 0 = 8, bit 4 = 12)
    pub tall_rolled: u8,
    
    /// Bits 0-5: doubles 1-1 through 6-6 (bit 0 = 1-1, bit 5 = 6-6)
    pub doubles_rolled: u8,
    
    /// Count of unique pass line points won (for Fire bet)
    pub fire_points: u8,
    
    /// Count of unique point combinations hit (for Hot Roller bet)
    pub hot_roller_count: u8,
    
    /// Total pass line wins (for Ride Line bet, max 255)
    pub ride_line_streak: u8,
    
    /// Hit counts for numbers 2-12 (for Repeater bets)
    pub hits: [u8; 11],
    
    /// Pass wins for points 4,5,6,8,9,10 (for Replay bet)
    pub pass_wins: [u8; 6],
    
    /// Count of each double (for TwiceHard bet)
    pub doubles: [u8; 6],
    
    /// Reserved for future use
    pub _reserved: [u8; 8],
}

impl BonusState {
    pub const LEN: usize = core::mem::size_of::<Self>();
    
    /// Initialize with default values
    pub fn new() -> Self {
        Self {
            small_rolled: 0,
            tall_rolled: 0,
            doubles_rolled: 0,
            fire_points: 0,
            hot_roller_count: 0,
            ride_line_streak: 0,
            hits: [0; 11],
            pass_wins: [0; 6],
            doubles: [0; 6],
            _reserved: [0; 8],
        }
    }
    
    /// Check if a small number (2-6) has been rolled
    pub fn is_small_rolled(&self, num: u8) -> bool {
        if num < 2 || num > 6 {
            return false;
        }
        let bit = num - 2;
        (self.small_rolled & (1 << bit)) != 0
    }
    
    /// Mark a small number as rolled
    pub fn mark_small_rolled(&mut self, num: u8) {
        if num >= 2 && num <= 6 {
            let bit = num - 2;
            self.small_rolled |= 1 << bit;
        }
    }
    
    /// Check if a tall number (8-12) has been rolled
    pub fn is_tall_rolled(&self, num: u8) -> bool {
        if num < 8 || num > 12 {
            return false;
        }
        let bit = num - 8;
        (self.tall_rolled & (1 << bit)) != 0
    }
    
    /// Mark a tall number as rolled
    pub fn mark_tall_rolled(&mut self, num: u8) {
        if num >= 8 && num <= 12 {
            let bit = num - 8;
            self.tall_rolled |= 1 << bit;
        }
    }
    
    /// Check if a specific double has been rolled
    pub fn is_double_rolled(&self, die_value: u8) -> bool {
        if die_value < 1 || die_value > 6 {
            return false;
        }
        let bit = die_value - 1;
        (self.doubles_rolled & (1 << bit)) != 0
    }
    
    /// Mark a double as rolled
    pub fn mark_double_rolled(&mut self, die_value: u8) {
        if die_value >= 1 && die_value <= 6 {
            let bit = die_value - 1;
            self.doubles_rolled |= 1 << bit;
        }
    }
    
    /// Check if all small numbers have been rolled
    pub fn all_small_rolled(&self) -> bool {
        self.small_rolled == 0b11111
    }
    
    /// Check if all tall numbers have been rolled
    pub fn all_tall_rolled(&self) -> bool {
        self.tall_rolled == 0b11111
    }
    
    /// Count how many different doubles have been rolled
    pub fn count_doubles_rolled(&self) -> u8 {
        self.doubles_rolled.count_ones() as u8
    }
    
    /// Update bonus state based on dice roll
    pub fn update_for_roll(&mut self, dice1: u8, dice2: u8, dice_total: u8, prev_point: u8) {
        // Update hit counts for repeater bets
        if dice_total >= 2 && dice_total <= 12 {
            let idx = (dice_total - 2) as usize;
            self.hits[idx] = self.hits[idx].saturating_add(1);
        }
        
        // Update small/tall/all tracking
        match dice_total {
            2..=6 => self.mark_small_rolled(dice_total),
            8..=12 => self.mark_tall_rolled(dice_total),
            _ => {}
        }
        
        // Track doubles
        if dice1 == dice2 {
            self.mark_double_rolled(dice1);
            if dice1 >= 1 && dice1 <= 6 {
                let idx = (dice1 as usize).saturating_sub(1);
                self.doubles[idx] = self.doubles[idx].saturating_add(1);
            }
        }
        
        // Track pass wins for points
        if prev_point != 0 && dice_total == prev_point {
            match prev_point {
                4 => self.pass_wins[0] = self.pass_wins[0].saturating_add(1),
                5 => self.pass_wins[1] = self.pass_wins[1].saturating_add(1),
                6 => self.pass_wins[2] = self.pass_wins[2].saturating_add(1),
                8 => self.pass_wins[3] = self.pass_wins[3].saturating_add(1),
                9 => self.pass_wins[4] = self.pass_wins[4].saturating_add(1),
                10 => self.pass_wins[5] = self.pass_wins[5].saturating_add(1),
                _ => {}
            }
            
            // Increment fire points and ride line streak
            self.fire_points = self.fire_points.saturating_add(1);
            self.ride_line_streak = self.ride_line_streak.saturating_add(1);
        }
        
        // Update hot roller count for point numbers
        if matches!(dice_total, 4 | 5 | 6 | 8 | 9 | 10) {
            self.hot_roller_count = self.hot_roller_count.saturating_add(1);
        }
    }
    
    /// Reset bonus state on seven-out
    pub fn reset_on_seven_out(&mut self) {
        self.hits = [0; 11];
        self.pass_wins = [0; 6];
        self.doubles = [0; 6];
        self.small_rolled = 0;
        self.tall_rolled = 0;
        self.doubles_rolled = 0;
        self.fire_points = 0;
        self.hot_roller_count = 0;
        self.ride_line_streak = 0;
    }
    
    /// Get hit count for a specific number (for Repeater bets)
    pub fn get_hit_count(&self, num: u8) -> u8 {
        if num >= 2 && num <= 12 {
            self.hits[(num - 2) as usize]
        } else {
            0
        }
    }
    
    /// Get pass win count for a specific point (for Replay bet)
    pub fn get_pass_win_count(&self, point: u8) -> u8 {
        match point {
            4 => self.pass_wins[0],
            5 => self.pass_wins[1],
            6 => self.pass_wins[2],
            8 => self.pass_wins[3],
            9 => self.pass_wins[4],
            10 => self.pass_wins[5],
            _ => 0,
        }
    }
    
    /// Get double count for a specific die value (for TwiceHard bet)
    pub fn get_double_count(&self, die_value: u8) -> u8 {
        if die_value >= 1 && die_value <= 6 {
            self.doubles[(die_value - 1) as usize]
        } else {
            0
        }
    }
    
    /// Check if any double has been rolled at least twice (for TwiceHard bet)
    pub fn has_twice_hard(&self) -> bool {
        self.doubles.iter().any(|&count| count >= 2)
    }
}

impl Default for BonusState {
    fn default() -> Self {
        Self::new()
    }
}

// Point combinations for HOT_ROLLER bet tracking
pub const POINT_COMBOS: &[(u8, &[(u8, u8)])] = &[
    (4, &[(1, 3), (2, 2)]),
    (5, &[(1, 4), (2, 3)]),
    (6, &[(1, 5), (2, 4), (3, 3)]),
    (8, &[(6, 2), (5, 3), (4, 4)]),
    (9, &[(6, 3), (5, 4)]),
    (10, &[(6, 4), (5, 5)]),
];