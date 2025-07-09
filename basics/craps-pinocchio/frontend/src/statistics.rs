use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::game::{GameState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub player: Pubkey,
    pub sessions: Vec<Session>,
    pub lifetime: LifetimeStats,
    pub current_session: Session,
    pub hot_numbers: HotNumbers,
    pub bet_analysis: BetAnalysis,
    pub current_balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub starting_balance: u64,
    pub ending_balance: Option<u64>,
    pub bets_placed: u32,
    pub bets_won: u32,
    pub total_wagered: u64,
    pub total_won: u64,
    pub largest_bet: u64,
    pub largest_win: u64,
    pub longest_win_streak: u32,
    pub longest_loss_streak: u32,
    pub current_streak: i32, // positive for wins, negative for losses
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifetimeStats {
    pub total_sessions: u32,
    pub total_bets: u64,
    pub total_wins: u64,
    pub total_wagered: u64,
    pub total_profit: i64,
    pub win_rate: f64,
    pub average_bet: f64,
    pub roi: f64, // Return on Investment
    pub best_session_profit: i64,
    pub worst_session_loss: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotNumbers {
    pub rolls: HashMap<u8, u32>,     // dice total -> count
    pub last_100: Vec<u8>,          // last 100 rolls
    pub current_temperature: HashMap<u8, Temperature>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Temperature {
    Cold,
    Cool,
    Neutral,
    Warm,
    Hot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetAnalysis {
    pub by_type: HashMap<String, BetTypeStats>,
    pub by_amount: HashMap<String, u32>, // amount range -> count
    pub hourly_distribution: [u32; 24],
    pub profitability_by_type: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetTypeStats {
    pub count: u32,
    pub total_wagered: u64,
    pub total_won: u64,
    pub win_rate: f64,
    pub profit: i64,
}

impl Statistics {
    pub fn new(player: Pubkey) -> Self {
        Self {
            player,
            sessions: Vec::new(),
            lifetime: LifetimeStats::default(),
            current_session: Session::new(0),
            hot_numbers: HotNumbers::default(),
            bet_analysis: BetAnalysis::default(),
            current_balance: 0,
        }
    }
    
    pub fn load(player: Pubkey) -> Result<Self> {
        let stats_path = Self::stats_path(&player)?;
        
        if stats_path.exists() {
            let contents = std::fs::read_to_string(&stats_path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::new(player))
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let stats_path = Self::stats_path(&self.player)?;
        let contents = serde_json::to_string_pretty(self)?;
        
        if let Some(parent) = stats_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(&stats_path, contents)?;
        Ok(())
    }
    
    pub fn record_bet(&mut self, amount: u64) {
        self.current_session.bets_placed += 1;
        self.current_session.total_wagered += amount;
        
        if amount > self.current_session.largest_bet {
            self.current_session.largest_bet = amount;
        }
        
        self.lifetime.total_bets += 1;
        self.lifetime.total_wagered += amount;
        self.update_averages();
    }
    
    pub fn record_win(&mut self, amount: u64) {
        self.current_session.bets_won += 1;
        self.current_session.total_won += amount;
        
        if amount > self.current_session.largest_win {
            self.current_session.largest_win = amount;
        }
        
        // Update streak
        if self.current_session.current_streak >= 0 {
            self.current_session.current_streak += 1;
            if self.current_session.current_streak as u32 > self.current_session.longest_win_streak {
                self.current_session.longest_win_streak = self.current_session.current_streak as u32;
            }
        } else {
            self.current_session.current_streak = 1;
        }
        
        self.lifetime.total_wins += 1;
        self.update_averages();
    }
    
    pub fn record_loss(&mut self) {
        // Update streak
        if self.current_session.current_streak <= 0 {
            self.current_session.current_streak -= 1;
            if (-self.current_session.current_streak) as u32 > self.current_session.longest_loss_streak {
                self.current_session.longest_loss_streak = (-self.current_session.current_streak) as u32;
            }
        } else {
            self.current_session.current_streak = -1;
        }
        
        self.update_averages();
    }
    
    pub fn record_withdrawal(&mut self, amount: u64) {
        self.current_balance = self.current_balance.saturating_sub(amount);
    }
    
    pub fn record_roll(&mut self, total: u8) {
        *self.hot_numbers.rolls.entry(total).or_insert(0) += 1;
        
        self.hot_numbers.last_100.push(total);
        if self.hot_numbers.last_100.len() > 100 {
            self.hot_numbers.last_100.remove(0);
        }
        
        self.update_temperatures();
    }
    
    pub fn end_session(&mut self) {
        self.current_session.end_time = Some(Local::now());
        self.current_session.ending_balance = Some(self.current_balance);
        
        // Calculate session profit/loss
        let session_profit = self.current_session.ending_balance.unwrap() as i64 
            - self.current_session.starting_balance as i64;
        
        if session_profit > self.lifetime.best_session_profit {
            self.lifetime.best_session_profit = session_profit;
        }
        if session_profit < self.lifetime.worst_session_loss {
            self.lifetime.worst_session_loss = session_profit;
        }
        
        self.sessions.push(self.current_session.clone());
        self.lifetime.total_sessions += 1;
        
        // Start new session
        self.current_session = Session::new(self.current_balance);
    }
    
    pub fn refresh(&mut self, game_state: &GameState) {
        // Update any statistics based on current game state
        if let Some(total) = game_state.dice_total() {
            self.record_roll(total);
        }
    }
    
    pub fn export(&self, path: &Path) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
    
    pub fn get_win_rate(&self) -> f64 {
        if self.current_session.bets_placed == 0 {
            0.0
        } else {
            self.current_session.bets_won as f64 / self.current_session.bets_placed as f64
        }
    }
    
    pub fn get_profit(&self) -> i64 {
        self.current_session.total_won as i64 - self.current_session.total_wagered as i64
    }
    
    pub fn get_roi(&self) -> f64 {
        if self.current_session.total_wagered == 0 {
            0.0
        } else {
            self.get_profit() as f64 / self.current_session.total_wagered as f64 * 100.0
        }
    }
    
    pub fn get_hot_numbers(&self, count: usize) -> Vec<(u8, u32)> {
        let mut numbers: Vec<(u8, u32)> = self.hot_numbers.rolls.iter()
            .map(|(&k, &v)| (k, v))
            .collect();
        numbers.sort_by(|a, b| b.1.cmp(&a.1));
        numbers.truncate(count);
        numbers
    }
    
    fn update_averages(&mut self) {
        if self.lifetime.total_bets > 0 {
            self.lifetime.average_bet = self.lifetime.total_wagered as f64 / self.lifetime.total_bets as f64;
            self.lifetime.win_rate = self.lifetime.total_wins as f64 / self.lifetime.total_bets as f64;
        }
        
        self.lifetime.total_profit = self.current_balance as i64 - self.sessions.first()
            .map(|s| s.starting_balance as i64)
            .unwrap_or(0);
        
        if self.lifetime.total_wagered > 0 {
            self.lifetime.roi = self.lifetime.total_profit as f64 / self.lifetime.total_wagered as f64 * 100.0;
        }
    }
    
    fn update_temperatures(&mut self) {
        // Calculate expected frequency for each number
        let expected_frequencies = [
            (2, 1.0/36.0), (3, 2.0/36.0), (4, 3.0/36.0), (5, 4.0/36.0),
            (6, 5.0/36.0), (7, 6.0/36.0), (8, 5.0/36.0), (9, 4.0/36.0),
            (10, 3.0/36.0), (11, 2.0/36.0), (12, 1.0/36.0),
        ];
        
        let total_rolls = self.hot_numbers.last_100.len() as f64;
        
        for (num, expected) in expected_frequencies {
            let actual = self.hot_numbers.last_100.iter()
                .filter(|&&x| x == num)
                .count() as f64 / total_rolls.max(1.0);
            
            let ratio = actual / expected;
            
            let temp = match ratio {
                r if r < 0.5 => Temperature::Cold,
                r if r < 0.8 => Temperature::Cool,
                r if r < 1.2 => Temperature::Neutral,
                r if r < 1.5 => Temperature::Warm,
                _ => Temperature::Hot,
            };
            
            self.hot_numbers.current_temperature.insert(num, temp);
        }
    }
    
    fn stats_path(player: &Pubkey) -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
        Ok(data_dir.join("craps-tui").join("stats").join(format!("{}.json", player)))
    }
}

impl Default for LifetimeStats {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            total_bets: 0,
            total_wins: 0,
            total_wagered: 0,
            total_profit: 0,
            win_rate: 0.0,
            average_bet: 0.0,
            roi: 0.0,
            best_session_profit: 0,
            worst_session_loss: 0,
        }
    }
}

impl Default for HotNumbers {
    fn default() -> Self {
        Self {
            rolls: HashMap::new(),
            last_100: Vec::new(),
            current_temperature: HashMap::new(),
        }
    }
}

impl Default for BetAnalysis {
    fn default() -> Self {
        Self {
            by_type: HashMap::new(),
            by_amount: HashMap::new(),
            hourly_distribution: [0; 24],
            profitability_by_type: HashMap::new(),
        }
    }
}

impl Session {
    fn new(starting_balance: u64) -> Self {
        Self {
            start_time: Local::now(),
            end_time: None,
            starting_balance,
            ending_balance: None,
            bets_placed: 0,
            bets_won: 0,
            total_wagered: 0,
            total_won: 0,
            largest_bet: 0,
            largest_win: 0,
            longest_win_streak: 0,
            longest_loss_streak: 0,
            current_streak: 0,
        }
    }
}