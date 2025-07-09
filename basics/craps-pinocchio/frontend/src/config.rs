use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Display preferences
    pub theme: ThemeConfig,
    pub display: DisplayConfig,
    
    // Betting preferences
    pub betting: BettingConfig,
    
    // Hotkeys
    pub hotkeys: HotkeyConfig,
    
    // Advanced settings
    pub advanced: AdvancedConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub color_scheme: ColorScheme,
    pub animations_enabled: bool,
    pub sound_enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorScheme {
    Professional,  // Default minimalist theme
    HighContrast, // For accessibility
    Casino,       // Traditional casino colors
    Matrix,       // Green on black
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub show_statistics_panel: bool,
    pub show_hot_numbers: bool,
    pub show_bet_history: bool,
    pub compact_mode: bool,
    pub dice_animation_speed: u8, // 0-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BettingConfig {
    pub default_bet_amount: f64,
    pub quick_bet_amounts: Vec<f64>,
    pub confirm_bets_over: Option<f64>, // None = always confirm
    pub auto_repeat_last_bet: bool,
    pub martingale_enabled: bool,
    pub stop_loss: Option<f64>,
    pub stop_win: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub place_bet: String,
    pub repeat_last: String,
    pub double_bet: String,
    pub half_bet: String,
    pub max_bet: String,
    pub withdraw: String,
    pub statistics: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    pub rpc_timeout_seconds: u64,
    pub auto_claim_wins: bool,
    pub bet_batch_size: u8,
    pub cache_duration_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeConfig {
                color_scheme: ColorScheme::Professional,
                animations_enabled: true,
                sound_enabled: false,
            },
            display: DisplayConfig {
                show_statistics_panel: true,
                show_hot_numbers: true,
                show_bet_history: true,
                compact_mode: false,
                dice_animation_speed: 5,
            },
            betting: BettingConfig {
                default_bet_amount: 100.0,
                quick_bet_amounts: vec![100.0, 500.0, 1000.0, 5000.0, 10000.0],
                confirm_bets_over: Some(1000.0),
                auto_repeat_last_bet: false,
                martingale_enabled: false,
                stop_loss: None,
                stop_win: None,
            },
            hotkeys: HotkeyConfig {
                place_bet: "Enter".to_string(),
                repeat_last: "Ctrl+R".to_string(),
                double_bet: "Ctrl+D".to_string(),
                half_bet: "Ctrl+H".to_string(),
                max_bet: "Ctrl+M".to_string(),
                withdraw: "Ctrl+W".to_string(),
                statistics: "Ctrl+S".to_string(),
            },
            advanced: AdvancedConfig {
                rpc_timeout_seconds: 30,
                auto_claim_wins: true,
                bet_batch_size: 16,
                cache_duration_seconds: 5,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let contents = toml::to_string_pretty(self)?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(&config_path, contents)?;
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("craps-tui").join("config.toml"))
    }
}