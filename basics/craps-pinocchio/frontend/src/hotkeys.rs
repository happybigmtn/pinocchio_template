use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    // Navigation
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    Confirm,
    Cancel,
    
    // Modes
    SwitchToDashboard,
    SwitchToBetting,
    SwitchToHistory,
    SwitchToStatistics,
    SwitchToSettings,
    ShowHelp,
    
    // Betting
    PlaceBet,
    RepeatLastBet,
    DoubleBet,
    HalfBet,
    MaxBet,
    ClearBet,
    
    // Quick bets
    QuickBet100,
    QuickBet500,
    QuickBet1000,
    QuickBet5000,
    QuickBet10000,
    
    // Bet types
    SelectPass,
    SelectDontPass,
    SelectField,
    SelectCome,
    SelectDontCome,
    SelectNumber(u8),
    SelectHard(u8),
    SelectOdds,
    SelectRepeater,
    
    // Actions
    Withdraw,
    Deposit,
    RefreshStats,
    ExportData,
    
    // System
    Quit,
    ToggleCompactMode,
    ToggleAnimations,
}

pub struct HotkeyManager {
    bindings: HashMap<(KeyCode, KeyModifiers), Action>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        
        // Navigation
        bindings.insert((KeyCode::Up, KeyModifiers::NONE), Action::NavigateUp);
        bindings.insert((KeyCode::Char('k'), KeyModifiers::NONE), Action::NavigateUp);
        bindings.insert((KeyCode::Down, KeyModifiers::NONE), Action::NavigateDown);
        bindings.insert((KeyCode::Char('j'), KeyModifiers::NONE), Action::NavigateDown);
        bindings.insert((KeyCode::Left, KeyModifiers::NONE), Action::NavigateLeft);
        bindings.insert((KeyCode::Char('h'), KeyModifiers::NONE), Action::NavigateLeft);
        bindings.insert((KeyCode::Right, KeyModifiers::NONE), Action::NavigateRight);
        bindings.insert((KeyCode::Char('l'), KeyModifiers::NONE), Action::NavigateRight);
        bindings.insert((KeyCode::Enter, KeyModifiers::NONE), Action::Confirm);
        bindings.insert((KeyCode::Esc, KeyModifiers::NONE), Action::Cancel);
        
        // Mode switching
        bindings.insert((KeyCode::Char('d'), KeyModifiers::NONE), Action::SwitchToDashboard);
        bindings.insert((KeyCode::Char('b'), KeyModifiers::NONE), Action::SwitchToBetting);
        bindings.insert((KeyCode::Char('h'), KeyModifiers::CONTROL), Action::SwitchToHistory);
        bindings.insert((KeyCode::Char('s'), KeyModifiers::NONE), Action::SwitchToStatistics);
        bindings.insert((KeyCode::Char('c'), KeyModifiers::NONE), Action::SwitchToSettings);
        bindings.insert((KeyCode::F(1), KeyModifiers::NONE), Action::ShowHelp);
        bindings.insert((KeyCode::Char('?'), KeyModifiers::NONE), Action::ShowHelp);
        
        // Betting actions
        bindings.insert((KeyCode::Char('r'), KeyModifiers::CONTROL), Action::RepeatLastBet);
        bindings.insert((KeyCode::Char('d'), KeyModifiers::CONTROL), Action::DoubleBet);
        bindings.insert((KeyCode::Char('h'), KeyModifiers::CONTROL), Action::HalfBet);
        bindings.insert((KeyCode::Char('m'), KeyModifiers::CONTROL), Action::MaxBet);
        bindings.insert((KeyCode::Char('c'), KeyModifiers::CONTROL), Action::ClearBet);
        
        // Quick bet amounts
        bindings.insert((KeyCode::F(2), KeyModifiers::NONE), Action::QuickBet100);
        bindings.insert((KeyCode::F(3), KeyModifiers::NONE), Action::QuickBet500);
        bindings.insert((KeyCode::F(4), KeyModifiers::NONE), Action::QuickBet1000);
        bindings.insert((KeyCode::F(5), KeyModifiers::NONE), Action::QuickBet5000);
        bindings.insert((KeyCode::F(6), KeyModifiers::NONE), Action::QuickBet10000);
        
        // Bet type selection
        bindings.insert((KeyCode::Char('p'), KeyModifiers::NONE), Action::SelectPass);
        bindings.insert((KeyCode::Char('P'), KeyModifiers::SHIFT), Action::SelectDontPass);
        bindings.insert((KeyCode::Char('f'), KeyModifiers::NONE), Action::SelectField);
        bindings.insert((KeyCode::Char('c'), KeyModifiers::NONE), Action::SelectCome);
        bindings.insert((KeyCode::Char('C'), KeyModifiers::SHIFT), Action::SelectDontCome);
        bindings.insert((KeyCode::Char('o'), KeyModifiers::CONTROL), Action::SelectOdds);
        bindings.insert((KeyCode::Char('r'), KeyModifiers::NONE), Action::SelectRepeater);
        
        // Number bets
        for num in 2..=12 {
            if num != 7 {
                let key = match num {
                    10 => '0',
                    11 => '-',
                    12 => '=',
                    _ => char::from_digit(num as u32, 10).unwrap(),
                };
                bindings.insert((KeyCode::Char(key), KeyModifiers::NONE), Action::SelectNumber(num));
            }
        }
        
        // Hard ways
        bindings.insert((KeyCode::Char('4'), KeyModifiers::CONTROL), Action::SelectHard(4));
        bindings.insert((KeyCode::Char('6'), KeyModifiers::CONTROL), Action::SelectHard(6));
        bindings.insert((KeyCode::Char('8'), KeyModifiers::CONTROL), Action::SelectHard(8));
        bindings.insert((KeyCode::Char('0'), KeyModifiers::CONTROL), Action::SelectHard(10));
        
        // Other actions
        bindings.insert((KeyCode::Char('w'), KeyModifiers::CONTROL), Action::Withdraw);
        bindings.insert((KeyCode::Char('d'), KeyModifiers::ALT), Action::Deposit);
        bindings.insert((KeyCode::Char('r'), KeyModifiers::ALT), Action::RefreshStats);
        bindings.insert((KeyCode::Char('e'), KeyModifiers::CONTROL), Action::ExportData);
        
        // System
        bindings.insert((KeyCode::Char('q'), KeyModifiers::NONE), Action::Quit);
        bindings.insert((KeyCode::Char('Q'), KeyModifiers::SHIFT), Action::Quit);
        bindings.insert((KeyCode::Char('t'), KeyModifiers::CONTROL), Action::ToggleCompactMode);
        bindings.insert((KeyCode::Char('a'), KeyModifiers::CONTROL), Action::ToggleAnimations);
        
        Self { bindings }
    }
}

impl HotkeyManager {
    pub fn get_action(&self, key: KeyEvent) -> Option<Action> {
        self.bindings.get(&(key.code, key.modifiers)).copied()
    }
    
    pub fn get_hotkey_for_action(&self, action: Action) -> Option<String> {
        self.bindings.iter()
            .find(|(_, &a)| a == action)
            .map(|((code, mods), _)| format_hotkey(*code, *mods))
    }
    
    pub fn customize_binding(&mut self, key: KeyCode, modifiers: KeyModifiers, action: Action) {
        // Remove any existing binding for this action
        self.bindings.retain(|_, &mut a| a != action);
        // Add new binding
        self.bindings.insert((key, modifiers), action);
    }
}

fn format_hotkey(code: KeyCode, modifiers: KeyModifiers) -> String {
    let mut parts = Vec::new();
    
    if modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("Ctrl");
    }
    if modifiers.contains(KeyModifiers::ALT) {
        parts.push("Alt");
    }
    if modifiers.contains(KeyModifiers::SHIFT) {
        parts.push("Shift");
    }
    
    let key_str = match code {
        KeyCode::Char(c) => c.to_uppercase().to_string(),
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        _ => "?".to_string(),
    };
    
    parts.push(&key_str);
    parts.join("+")
}