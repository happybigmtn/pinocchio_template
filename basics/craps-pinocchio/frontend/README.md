# Professional Craps TUI Frontend

A sophisticated terminal user interface for the Solana Craps game, designed with professional gamblers in mind. Inspired by Jony Ive's design philosophy, this interface emphasizes minimalism, efficiency, and keyboard-only operation.

## Features

- **Professional Design**: Minimalist aesthetic with high contrast colors and clean typography
- **Keyboard-Only Navigation**: Complete control without touching the mouse
- **Real-Time Statistics**: Track win rates, hot numbers, and session performance
- **Advanced Betting Options**: Support for all craps bet types including repeater bets
- **Session Management**: Automatic tracking of profits, losses, and betting patterns
- **Customizable Settings**: Themes, hotkeys, and betting preferences

## Installation

```bash
cd frontend
cargo build --release
```

## Configuration

Before running, update the program IDs in `src/rpc.rs`:
- Replace `YourDevnetProgramIDHere` with your devnet program ID
- Replace `YourMainnetProgramIDHere` with your mainnet program ID

## Usage

### Running the Application

```bash
# Connect to devnet (default)
cargo run --release

# Connect to mainnet
cargo run --release -- --mainnet

# Use custom RPC endpoint
cargo run --release -- --rpc-url https://api.mainnet-beta.solana.com

# Use custom keypair
cargo run --release -- --keypair ~/.config/solana/my-keypair.json
```

### Key Navigation

#### Global Shortcuts
- `TAB` - Cycle through modes
- `ESC` - Back / Cancel
- `F1` / `?` - Show help
- `Q` - Quit application

#### Dashboard Mode
- `B` - Switch to betting
- `H` - View history
- `S` - View statistics
- `C` - Settings
- `Ctrl+R` - Repeat last bet
- `Ctrl+W` - Withdraw winnings

#### Betting Mode
- `P` - Pass Line bet
- `D` - Don't Pass bet
- `F` - Field bet
- `C` - Come bet
- `4-10` - Number bets
- `Ctrl+O` - Odds bet
- `F2-F5` - Quick bet amounts (100, 500, 1k, 5k)
- `Enter` - Place bet
- `Ctrl+R` - Repeat last bet
- `Ctrl+D` - Double bet
- `Ctrl+H` - Half bet

#### Statistics Mode
- `R` - Refresh statistics
- `E` - Export to file

## Design Philosophy

This interface follows key design principles:

1. **Simplicity**: Every element serves a purpose
2. **Efficiency**: Common actions require minimal keystrokes
3. **Professional**: Designed for serious players who value speed and precision
4. **Accessibility**: High contrast colors and clear typography
5. **Performance**: Optimized for fast updates and minimal latency

## Architecture

- `main.rs` - Application entry point and terminal setup
- `app.rs` - Core application state and logic
- `ui.rs` - All UI rendering components
- `game.rs` - Game types and bet definitions
- `rpc.rs` - Solana RPC client for blockchain interaction
- `statistics.rs` - Comprehensive statistics tracking
- `config.rs` - User preferences and settings
- `hotkeys.rs` - Keyboard shortcut management

## Customization

Settings are stored in `~/.config/craps-tui/config.toml`:

```toml
[theme]
color_scheme = "Professional"  # Options: Professional, HighContrast, Casino, Matrix
animations_enabled = true
sound_enabled = false

[betting]
default_bet_amount = 100.0
quick_bet_amounts = [100.0, 500.0, 1000.0, 5000.0, 10000.0]
confirm_bets_over = 1000.0
auto_repeat_last_bet = false
martingale_enabled = false

[advanced]
rpc_timeout_seconds = 30
auto_claim_wins = true
```

## Statistics Tracking

The application tracks comprehensive statistics:

- **Session Stats**: Current profit/loss, win rate, streak tracking
- **Lifetime Stats**: Total wagered, ROI, best/worst sessions
- **Hot Numbers**: Roll frequency analysis with temperature indicators
- **Bet Analysis**: Performance by bet type and amount

Statistics are automatically saved to `~/.local/share/craps-tui/stats/`.

## Contributing

This frontend is designed to be extended. Key areas for contribution:

1. Additional bet types and strategies
2. Advanced statistics visualizations
3. Multi-table support
4. Tournament mode
5. Mobile/touch support

## License

This project is licensed under the same terms as the Solana Craps program.