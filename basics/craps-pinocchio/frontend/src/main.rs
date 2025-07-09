mod app;
mod ui;
mod game;
mod rpc;
mod config;
mod statistics;
mod hotkeys;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{io, time::Duration};
use tokio::time::interval;

use crate::app::App;
use crate::ui::draw_ui;

/// Professional Craps Terminal Interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,

    /// Path to keypair file
    #[arg(short, long)]
    keypair: Option<String>,

    /// Enable devnet mode
    #[arg(short, long)]
    devnet: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();
    
    // Initialize logger
    env_logger::init();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app
    let mut app = App::new(&args.rpc_url, args.keypair, args.devnet).await?;
    
    // Run app
    let res = run_app(&mut terminal, &mut app).await;
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }
    
    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let mut ticker = interval(Duration::from_millis(100));
    
    loop {
        terminal.draw(|f| draw_ui(f, app))?;
        
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        if app.should_quit() {
                            return Ok(());
                        }
                    }
                    _ => app.on_key(key).await?,
                }
            }
        }
        
        // Update game state periodically
        tokio::select! {
            _ = ticker.tick() => {
                app.tick().await?;
            }
        }
    }
}