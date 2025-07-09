use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
};
use std::path::PathBuf;

use crate::{
    config::Config,
    game::{BetOutcome, BetType, GameState, PlacedBet},
    hotkeys::{HotkeyManager},
    rpc::CrapsRpcClient,
    statistics::Statistics,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Dashboard,
    Betting,
    History,
    Statistics,
    Settings,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BettingFocus {
    BetType,
    Amount,
    Confirm,
}

pub struct App {
    pub mode: AppMode,
    pub game_state: GameState,
    pub statistics: Statistics,
    pub config: Config,
    pub rpc_client: CrapsRpcClient,
    pub keypair: Keypair,
    pub hotkey_manager: HotkeyManager,
    
    // UI State
    pub selected_bet_type: BetType,
    pub bet_amount: String,
    pub betting_focus: BettingFocus,
    pub message: Option<(String, MessageType)>,
    pub show_confirm_dialog: bool,
    
    // Game state
    pub active_bets: Vec<PlacedBet>,
    pub bet_history: Vec<BetOutcome>,
    
    // Quick bet amounts for professional play
    pub quick_bets: Vec<u64>,
    pub last_bets: Vec<PlacedBet>,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Success,
    Error,
    Info,
    Warning,
}

impl App {
    pub async fn new(rpc_url: &str, keypair_path: Option<String>, devnet: bool) -> Result<Self> {
        let config = Config::load()?;
        let keypair = Self::load_keypair(keypair_path)?;
        let rpc_client = CrapsRpcClient::new(rpc_url, keypair.pubkey(), devnet)?;
        
        // Initialize game state
        let game_state = rpc_client.fetch_game_state().await?;
        let statistics = Statistics::load(keypair.pubkey())?;
        
        Ok(Self {
            mode: AppMode::Dashboard,
            game_state,
            statistics,
            config,
            rpc_client,
            keypair,
            hotkey_manager: HotkeyManager::default(),
            selected_bet_type: BetType::Pass,
            bet_amount: String::from("100"),
            betting_focus: BettingFocus::BetType,
            message: None,
            show_confirm_dialog: false,
            active_bets: Vec::new(),
            bet_history: Vec::new(),
            quick_bets: vec![100, 500, 1000, 5000, 10000],
            last_bets: Vec::new(),
        })
    }
    
    fn load_keypair(path: Option<String>) -> Result<Keypair> {
        let path = path.map(PathBuf::from).unwrap_or_else(|| {
            dirs::home_dir()
                .expect("Failed to get home directory")
                .join(".config/solana/id.json")
        });
        
        let keypair_str = std::fs::read_to_string(path)?;
        let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_str)?;
        Ok(Keypair::from_bytes(&keypair_bytes)?)
    }
    
    pub async fn on_key(&mut self, key: KeyEvent) -> Result<()> {
        // Global hotkeys
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Dashboard;
                self.show_confirm_dialog = false;
                return Ok(());
            }
            KeyCode::F(1) => {
                self.mode = AppMode::Help;
                return Ok(());
            }
            KeyCode::Tab => {
                self.cycle_mode();
                return Ok(());
            }
            _ => {}
        }
        
        // Mode-specific handling
        match self.mode {
            AppMode::Dashboard => self.handle_dashboard_keys(key).await?,
            AppMode::Betting => self.handle_betting_keys(key).await?,
            AppMode::History => self.handle_history_keys(key).await?,
            AppMode::Statistics => self.handle_statistics_keys(key).await?,
            AppMode::Settings => self.handle_settings_keys(key).await?,
            AppMode::Help => {} // Just display, ESC to exit
        }
        
        Ok(())
    }
    
    async fn handle_dashboard_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('b') | KeyCode::Char('B') => self.mode = AppMode::Betting,
            KeyCode::Char('h') | KeyCode::Char('H') => self.mode = AppMode::History,
            KeyCode::Char('s') | KeyCode::Char('S') => self.mode = AppMode::Statistics,
            KeyCode::Char('c') | KeyCode::Char('C') => self.mode = AppMode::Settings,
            
            // Quick actions
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.repeat_last_bet().await?;
                }
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.withdraw_winnings().await?;
            }
            _ => {}
        }
        Ok(())
    }
    
    async fn handle_betting_keys(&mut self, key: KeyEvent) -> Result<()> {
        if self.show_confirm_dialog {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.place_bet().await?;
                    self.show_confirm_dialog = false;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.show_confirm_dialog = false;
                }
                _ => {}
            }
            return Ok(());
        }
        
        match self.betting_focus {
            BettingFocus::BetType => {
                match key.code {
                    // Direct bet type selection
                    KeyCode::Char('p') => self.selected_bet_type = BetType::Pass,
                    KeyCode::Char('d') => self.selected_bet_type = BetType::DontPass,
                    KeyCode::Char('f') => self.selected_bet_type = BetType::Field,
                    KeyCode::Char('c') => self.selected_bet_type = BetType::Come,
                    KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.selected_bet_type = BetType::Odds(Box::new(self.selected_bet_type.clone()));
                    }
                    
                    // Navigation
                    KeyCode::Up | KeyCode::Char('k') => self.prev_bet_type(),
                    KeyCode::Down | KeyCode::Char('j') => self.next_bet_type(),
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
                        self.betting_focus = BettingFocus::Amount;
                    }
                    
                    // Quick number bets
                    KeyCode::Char(c) if c.is_digit(10) => {
                        if let Some(num) = c.to_digit(10) {
                            if num >= 2 {
                                self.selected_bet_type = BetType::Number(num as u8);
                            }
                        }
                    }
                    _ => {}
                }
            }
            BettingFocus::Amount => {
                match key.code {
                    KeyCode::Char(c) if c.is_digit(10) => {
                        self.bet_amount.push(c);
                    }
                    KeyCode::Backspace => {
                        self.bet_amount.pop();
                    }
                    KeyCode::Char('.') => {
                        if !self.bet_amount.contains('.') {
                            self.bet_amount.push('.');
                        }
                    }
                    
                    // Quick amounts
                    KeyCode::F(2) => self.bet_amount = "100".to_string(),
                    KeyCode::F(3) => self.bet_amount = "500".to_string(),
                    KeyCode::F(4) => self.bet_amount = "1000".to_string(),
                    KeyCode::F(5) => self.bet_amount = "5000".to_string(),
                    
                    // Navigation
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.betting_focus = BettingFocus::BetType;
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab | KeyCode::Enter => {
                        self.show_confirm_dialog = true;
                    }
                    _ => {}
                }
            }
            BettingFocus::Confirm => {
                // Handled above
            }
        }
        Ok(())
    }
    
    async fn handle_history_keys(&mut self, _key: KeyEvent) -> Result<()> {
        // History is read-only, navigation handled by UI component
        Ok(())
    }
    
    async fn handle_statistics_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.statistics.refresh(&self.game_state);
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                // Export statistics
                self.export_statistics()?;
            }
            _ => {}
        }
        Ok(())
    }
    
    async fn handle_settings_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.config.save()?;
                self.set_message("Settings saved", MessageType::Success);
            }
            _ => {}
        }
        Ok(())
    }
    
    pub async fn tick(&mut self) -> Result<()> {
        // Update game state periodically
        if let Ok(new_state) = self.rpc_client.fetch_game_state().await {
            if new_state.epoch != self.game_state.epoch {
                // New epoch, check for wins
                self.check_wins().await?;
            }
            self.game_state = new_state;
        }
        
        // Clear old messages
        if let Some((_, _)) = &self.message {
            // Messages auto-clear after a few seconds (handled by UI)
        }
        
        Ok(())
    }
    
    pub fn should_quit(&self) -> bool {
        // Add any cleanup checks here
        true
    }
    
    // Helper methods
    fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            AppMode::Dashboard => AppMode::Betting,
            AppMode::Betting => AppMode::History,
            AppMode::History => AppMode::Statistics,
            AppMode::Statistics => AppMode::Settings,
            AppMode::Settings => AppMode::Dashboard,
            AppMode::Help => AppMode::Dashboard,
        };
    }
    
    fn next_bet_type(&mut self) {
        // Implement cycling through bet types
    }
    
    fn prev_bet_type(&mut self) {
        // Implement cycling through bet types
    }
    
    async fn place_bet(&mut self) -> Result<()> {
        let amount = self.bet_amount.parse::<f64>()
            .unwrap_or(0.0) * 1_000_000_000.0; // Convert to lamports
        
        match self.rpc_client.place_bet(self.selected_bet_type.clone(), amount as u64).await {
            Ok(sig) => {
                self.last_bets.push(PlacedBet {
                    bet_type: self.selected_bet_type.clone(),
                    amount: amount as u64,
                    epoch: self.game_state.epoch,
                    signature: sig,
                });
                self.statistics.record_bet(amount as u64);
                self.set_message("Bet placed successfully", MessageType::Success);
            }
            Err(e) => {
                self.set_message(&format!("Failed to place bet: {}", e), MessageType::Error);
            }
        }
        
        Ok(())
    }
    
    async fn repeat_last_bet(&mut self) -> Result<()> {
        if let Some(last_bet) = self.last_bets.last() {
            self.selected_bet_type = last_bet.bet_type.clone();
            self.bet_amount = (last_bet.amount as f64 / 1_000_000_000.0).to_string();
            self.place_bet().await?;
        } else {
            self.set_message("No previous bet to repeat", MessageType::Warning);
        }
        Ok(())
    }
    
    async fn withdraw_winnings(&mut self) -> Result<()> {
        match self.rpc_client.withdraw_winnings().await {
            Ok(amount) => {
                self.statistics.record_withdrawal(amount);
                self.set_message(
                    &format!("Withdrew {} CRAP", amount as f64 / 1_000_000_000.0),
                    MessageType::Success
                );
            }
            Err(e) => {
                self.set_message(&format!("Withdrawal failed: {}", e), MessageType::Error);
            }
        }
        Ok(())
    }
    
    async fn check_wins(&mut self) -> Result<()> {
        // Check and claim any wins from the previous epoch
        if let Ok(winnings) = self.rpc_client.check_and_claim_wins(self.game_state.epoch - 1).await {
            if winnings > 0 {
                self.statistics.record_win(winnings);
                self.set_message(
                    &format!("Won {} CRAP!", winnings as f64 / 1_000_000_000.0),
                    MessageType::Success
                );
            }
        }
        Ok(())
    }
    
    fn export_statistics(&mut self) -> Result<()> {
        let path = dirs::home_dir()
            .unwrap()
            .join("craps_statistics.json");
        self.statistics.export(&path)?;
        self.set_message("Statistics exported", MessageType::Success);
        Ok(())
    }
    
    fn set_message(&mut self, msg: &str, msg_type: MessageType) {
        self.message = Some((msg.to_string(), msg_type));
    }
}