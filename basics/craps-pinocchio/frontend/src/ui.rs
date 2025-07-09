use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, BorderType, Clear, List, ListItem, Paragraph, Row, Table,
    },
    Frame,
};

use crate::{
    app::{App, AppMode, BettingFocus, MessageType},
    game::{BetType, GamePhase},
};

// Professional color scheme - minimalist, high contrast
const BG_COLOR: Color = Color::Rgb(15, 15, 20);        // Deep blue-black
const FG_COLOR: Color = Color::Rgb(230, 230, 230);     // Off-white
const ACCENT_COLOR: Color = Color::Rgb(100, 200, 255); // Professional blue
const WIN_COLOR: Color = Color::Rgb(100, 255, 150);    // Mint green
const LOSS_COLOR: Color = Color::Rgb(255, 100, 100);   // Soft red
const MUTED_COLOR: Color = Color::Rgb(128, 128, 140);  // Muted gray

pub fn draw_ui(f: &mut Frame, app: &App) {
    let size = f.size();
    
    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(size);
    
    draw_header(f, chunks[0], app);
    draw_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
    
    // Draw overlays
    if app.show_confirm_dialog {
        draw_confirm_dialog(f, app);
    }
    
    if let Some((msg, msg_type)) = &app.message {
        draw_message(f, msg, *msg_type);
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);
    
    // Left: Game info
    let game_info = Paragraph::new(format!(
        "Epoch {} | {} | Point: {}",
        app.game_state.epoch,
        match app.game_state.phase {
            GamePhase::ComeOut => "COME OUT",
            GamePhase::Point => "POINT",
        },
        app.game_state.point.map_or("None".to_string(), |p| p.to_string())
    ))
    .style(Style::default().fg(FG_COLOR))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(MUTED_COLOR))
    );
    f.render_widget(game_info, header_chunks[0]);
    
    // Center: Title
    let title = Paragraph::new("PROFESSIONAL CRAPS")
        .style(
            Style::default()
                .fg(ACCENT_COLOR)
                .add_modifier(Modifier::BOLD)
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(MUTED_COLOR))
        );
    f.render_widget(title, header_chunks[1]);
    
    // Right: Balance
    let balance = Paragraph::new(format!(
        "Balance: {} CRAP",
        app.statistics.current_balance as f64 / 1_000_000_000.0
    ))
    .style(Style::default().fg(WIN_COLOR))
    .alignment(Alignment::Right)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(MUTED_COLOR))
    );
    f.render_widget(balance, header_chunks[2]);
}

fn draw_content(f: &mut Frame, area: Rect, app: &App) {
    match app.mode {
        AppMode::Dashboard => draw_dashboard(f, area, app),
        AppMode::Betting => draw_betting_interface(f, area, app),
        AppMode::History => draw_history(f, area, app),
        AppMode::Statistics => draw_statistics(f, area, app),
        AppMode::Settings => draw_settings(f, area, app),
        AppMode::Help => draw_help(f, area),
    }
}

fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);
    
    // Left side: Dice display and game state
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .margin(1)
        .split(chunks[0]);
    
    draw_dice_display(f, left_chunks[0], app);
    draw_active_bets(f, left_chunks[1], app);
    
    // Right side: Quick stats and actions
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .margin(1)
        .split(chunks[1]);
    
    draw_session_stats(f, right_chunks[0], app);
    draw_hot_numbers(f, right_chunks[1], app);
    draw_quick_actions(f, right_chunks[2], app);
}

fn draw_dice_display(f: &mut Frame, area: Rect, app: &App) {
    let dice_art = match (app.game_state.die1, app.game_state.die2) {
        (Some(d1), Some(d2)) => format_dice(d1, d2),
        _ => vec![
            "     Waiting for roll...     ".to_string(),
            "                             ".to_string(),
            "         [ ? ] [ ? ]         ".to_string(),
        ],
    };
    
    let dice_text: Vec<Line> = dice_art
        .into_iter()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(FG_COLOR))))
        .collect();
    
    let dice_display = Paragraph::new(dice_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" DICE ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(ACCENT_COLOR))
        );
    
    f.render_widget(dice_display, area);
}

fn draw_betting_interface(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);
    
    // Left: Bet type selection
    draw_bet_type_selector(f, chunks[0], app);
    
    // Right: Bet details and confirmation
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),  // Bet details
            Constraint::Min(0),      // Bet history
        ])
        .split(chunks[1]);
    
    draw_bet_details(f, right_chunks[0], app);
    draw_recent_bets(f, right_chunks[1], app);
}

fn draw_bet_type_selector(f: &mut Frame, area: Rect, app: &App) {
    let bet_types = vec![
        ("P", "Pass Line", BetType::Pass),
        ("D", "Don't Pass", BetType::DontPass),
        ("F", "Field", BetType::Field),
        ("C", "Come", BetType::Come),
        ("", "─────────", BetType::Pass), // Separator
        ("4", "Four", BetType::Number(4)),
        ("5", "Five", BetType::Number(5)),
        ("6", "Six", BetType::Number(6)),
        ("8", "Eight", BetType::Number(8)),
        ("9", "Nine", BetType::Number(9)),
        ("10", "Ten", BetType::Number(10)),
        ("", "─────────", BetType::Pass), // Separator
        ("H4", "Hard 4", BetType::Hard4),
        ("H6", "Hard 6", BetType::Hard6),
        ("H8", "Hard 8", BetType::Hard8),
        ("H10", "Hard 10", BetType::Hard10),
    ];
    
    let items: Vec<ListItem> = bet_types
        .iter()
        .map(|(key, name, bet_type)| {
            let style = if *bet_type == app.selected_bet_type {
                Style::default()
                    .fg(BG_COLOR)
                    .bg(ACCENT_COLOR)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(FG_COLOR)
            };
            
            if key.is_empty() {
                ListItem::new(Line::from(vec![Span::styled(*name, Style::default().fg(MUTED_COLOR))]))
            } else {
                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {} ", key), Style::default().fg(ACCENT_COLOR)),
                    Span::styled(*name, style),
                ]))
            }
        })
        .collect();
    
    let bet_list = List::new(items)
        .block(
            Block::default()
                .title(" BET TYPES ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(
                    if app.betting_focus == BettingFocus::BetType {
                        ACCENT_COLOR
                    } else {
                        MUTED_COLOR
                    }
                ))
        );
    
    f.render_widget(bet_list, area);
}

fn draw_bet_details(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Bet type
            Constraint::Length(3),  // Amount
            Constraint::Length(3),  // Potential win
            Constraint::Length(3),  // Actions
        ])
        .split(area);
    
    // Selected bet type
    let bet_type_display = Paragraph::new(format!("Bet Type: {}", app.selected_bet_type))
        .style(Style::default().fg(FG_COLOR))
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(bet_type_display, chunks[0]);
    
    // Amount input
    let amount_style = if app.betting_focus == BettingFocus::Amount {
        Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(FG_COLOR)
    };
    
    let amount_display = Paragraph::new(format!("Amount: {} CRAP", app.bet_amount))
        .style(amount_style)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(
                    if app.betting_focus == BettingFocus::Amount {
                        ACCENT_COLOR
                    } else {
                        MUTED_COLOR
                    }
                ))
        );
    f.render_widget(amount_display, chunks[1]);
    
    // Potential win calculation
    let potential_win = calculate_potential_win(app.selected_bet_type.clone(), app.bet_amount.parse().unwrap_or(0.0));
    let win_display = Paragraph::new(format!("Potential Win: {} CRAP", potential_win))
        .style(Style::default().fg(WIN_COLOR))
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(win_display, chunks[2]);
    
    // Quick bet buttons
    let quick_bets = vec![
        Span::styled("F2 ", Style::default().fg(ACCENT_COLOR)),
        Span::raw("100 | "),
        Span::styled("F3 ", Style::default().fg(ACCENT_COLOR)),
        Span::raw("500 | "),
        Span::styled("F4 ", Style::default().fg(ACCENT_COLOR)),
        Span::raw("1k | "),
        Span::styled("F5 ", Style::default().fg(ACCENT_COLOR)),
        Span::raw("5k"),
    ];
    
    let quick_bet_display = Paragraph::new(Line::from(quick_bets))
        .style(Style::default().fg(FG_COLOR))
        .alignment(Alignment::Center);
    f.render_widget(quick_bet_display, chunks[3]);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.mode {
        AppMode::Dashboard => "B:Bet H:History S:Stats C:Settings Q:Quit",
        AppMode::Betting => "↑↓:Select TAB:Next Enter:Place ESC:Cancel",
        AppMode::History => "↑↓:Navigate ESC:Back",
        AppMode::Statistics => "R:Refresh E:Export ESC:Back",
        AppMode::Settings => "S:Save ESC:Cancel",
        AppMode::Help => "ESC:Back",
    };
    
    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(MUTED_COLOR))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(MUTED_COLOR))
        );
    
    f.render_widget(footer, area);
}

// Helper functions
fn format_dice(d1: u8, d2: u8) -> Vec<String> {
    let dice_faces = [
        vec!["     ", "  •  ", "     "], // 1
        vec!["•    ", "     ", "    •"], // 2
        vec!["•    ", "  •  ", "    •"], // 3
        vec!["•   •", "     ", "•   •"], // 4
        vec!["•   •", "  •  ", "•   •"], // 5
        vec!["•   •", "•   •", "•   •"], // 6
    ];
    
    let mut result = Vec::new();
    result.push(format!("┌─────┐ ┌─────┐"));
    
    for i in 0..3 {
        result.push(format!("│{}│ │{}│", 
            dice_faces[(d1 - 1) as usize][i],
            dice_faces[(d2 - 1) as usize][i]
        ));
    }
    
    result.push(format!("└─────┘ └─────┘"));
    result.push(format!("   {}       {}   ", d1, d2));
    result.push(format!("   Total: {}   ", d1 + d2));
    
    result
}

fn calculate_potential_win(bet_type: BetType, amount: f64) -> f64 {
    match bet_type {
        BetType::Pass | BetType::DontPass | BetType::Come | BetType::DontCome => amount,
        BetType::Field => amount * 2.0,
        BetType::Number(n) => match n {
            4 | 10 => amount * 1.8,
            5 | 9 => amount * 1.4,
            6 | 8 => amount * 1.166,
            _ => amount,
        },
        BetType::Hard4 | BetType::Hard10 => amount * 7.0,
        BetType::Hard6 | BetType::Hard8 => amount * 9.0,
        _ => amount,
    }
}

fn draw_confirm_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(40, 20, f.size());
    
    let block = Block::default()
        .title(" CONFIRM BET ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(ACCENT_COLOR));
    
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Place {} bet", app.selected_bet_type),
            Style::default().fg(FG_COLOR)
        )),
        Line::from(Span::styled(
            format!("for {} CRAP?", app.bet_amount),
            Style::default().fg(FG_COLOR).add_modifier(Modifier::BOLD)
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Y", Style::default().fg(WIN_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw("es  "),
            Span::styled("N", Style::default().fg(LOSS_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw("o"),
        ]),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn draw_message(f: &mut Frame, message: &str, msg_type: MessageType) {
    let area = Rect {
        x: f.size().width / 4,
        y: 0,
        width: f.size().width / 2,
        height: 3,
    };
    
    let style = match msg_type {
        MessageType::Success => Style::default().fg(BG_COLOR).bg(WIN_COLOR),
        MessageType::Error => Style::default().fg(BG_COLOR).bg(LOSS_COLOR),
        MessageType::Info => Style::default().fg(BG_COLOR).bg(ACCENT_COLOR),
        MessageType::Warning => Style::default().fg(BG_COLOR).bg(Color::Yellow),
    };
    
    let message_widget = Paragraph::new(message)
        .style(style.add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(style)
        );
    
    f.render_widget(Clear, area);
    f.render_widget(message_widget, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// Additional UI components
fn draw_active_bets(f: &mut Frame, area: Rect, app: &App) {
    let headers = vec!["Type", "Amount", "Status", "Win"];
    let rows: Vec<Row> = app.active_bets
        .iter()
        .map(|bet| {
            Row::new(vec![
                bet.bet_type.to_string(),
                format!("{:.2}", bet.amount as f64 / 1_000_000_000.0),
                "Active".to_string(),
                format!("{:.2}", calculate_potential_win(bet.bet_type.clone(), bet.amount as f64 / 1_000_000_000.0)),
            ])
            .style(Style::default().fg(FG_COLOR))
        })
        .collect();
    
    let table = Table::new(rows, vec![
        Constraint::Percentage(30),
        Constraint::Percentage(25),
        Constraint::Percentage(20),
        Constraint::Percentage(25),
    ])
    .header(
        Row::new(headers)
            .style(Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD))
            .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(" ACTIVE BETS ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(MUTED_COLOR))
    );
    
    f.render_widget(table, area);
}

fn draw_session_stats(f: &mut Frame, area: Rect, app: &App) {
    let stats = vec![
        ("Bets Placed", app.statistics.current_session.bets_placed.to_string()),
        ("Win Rate", format!("{:.1}%", app.statistics.get_win_rate() * 100.0)),
        ("Profit/Loss", format!("{:.2} CRAP", app.statistics.get_profit() as f64 / 1_000_000_000.0)),
        ("ROI", format!("{:.1}%", app.statistics.get_roi())),
        ("Streak", format!("{}", app.statistics.current_session.current_streak)),
    ];
    
    let stats_text: Vec<Line> = stats
        .iter()
        .map(|(label, value)| {
            Line::from(vec![
                Span::styled(format!("{}: ", label), Style::default().fg(MUTED_COLOR)),
                Span::styled(value, if label == &"Profit/Loss" && app.statistics.get_profit() > 0 {
                    Style::default().fg(WIN_COLOR).add_modifier(Modifier::BOLD)
                } else if label == &"Profit/Loss" && app.statistics.get_profit() < 0 {
                    Style::default().fg(LOSS_COLOR).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(FG_COLOR)
                }),
            ])
        })
        .collect();
    
    let stats_widget = Paragraph::new(stats_text)
        .block(
            Block::default()
                .title(" SESSION ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(ACCENT_COLOR))
        );
    
    f.render_widget(stats_widget, area);
}

fn draw_hot_numbers(f: &mut Frame, area: Rect, app: &App) {
    let hot_nums = app.statistics.get_hot_numbers(5);
    
    let hot_text: Vec<Line> = hot_nums
        .iter()
        .map(|(num, count)| {
            let temp = app.statistics.hot_numbers.current_temperature
                .get(num)
                .copied()
                .unwrap_or(crate::statistics::Temperature::Neutral);
            
            let color = match temp {
                crate::statistics::Temperature::Hot => LOSS_COLOR,
                crate::statistics::Temperature::Warm => Color::Rgb(255, 150, 100),
                crate::statistics::Temperature::Neutral => FG_COLOR,
                crate::statistics::Temperature::Cool => Color::Rgb(100, 150, 255),
                crate::statistics::Temperature::Cold => ACCENT_COLOR,
            };
            
            Line::from(vec![
                Span::styled(format!("{:2}", num), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::raw(": "),
                Span::styled(format!("{} rolls", count), Style::default().fg(MUTED_COLOR)),
            ])
        })
        .collect();
    
    let hot_widget = Paragraph::new(hot_text)
        .block(
            Block::default()
                .title(" HOT NUMBERS ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(LOSS_COLOR))
        );
    
    f.render_widget(hot_widget, area);
}

fn draw_quick_actions(f: &mut Frame, area: Rect, _app: &App) {
    let actions = vec![
        ("B", "Place Bet", AppMode::Betting),
        ("R", "Repeat Last", AppMode::Dashboard),
        ("W", "Withdraw", AppMode::Dashboard),
        ("H", "History", AppMode::History),
        ("S", "Statistics", AppMode::Statistics),
    ];
    
    let items: Vec<ListItem> = actions
        .iter()
        .map(|(key, label, _)| {
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", key), Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)),
                Span::raw(*label),
            ]))
        })
        .collect();
    
    let actions_list = List::new(items)
        .block(
            Block::default()
                .title(" QUICK ACTIONS ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(MUTED_COLOR))
        );
    
    f.render_widget(actions_list, area);
}

fn draw_recent_bets(f: &mut Frame, area: Rect, app: &App) {
    let headers = vec!["Time", "Type", "Amount", "Result"];
    
    let rows: Vec<Row> = app.bet_history
        .iter()
        .rev()
        .take(10)
        .map(|outcome| {
            let time = outcome.bet.epoch.to_string();
            let result_style = if outcome.won {
                Style::default().fg(WIN_COLOR)
            } else {
                Style::default().fg(LOSS_COLOR)
            };
            
            Row::new(vec![
                time,
                outcome.bet.bet_type.to_string(),
                format!("{:.2}", outcome.bet.amount as f64 / 1_000_000_000.0),
                if outcome.won { format!("+{:.2}", outcome.payout as f64 / 1_000_000_000.0) } else { "Lost".to_string() },
            ])
            .style(if outcome.won { result_style } else { Style::default().fg(MUTED_COLOR) })
        })
        .collect();
    
    let table = Table::new(rows, vec![
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .header(
        Row::new(headers)
            .style(Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD))
            .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(" RECENT BETS ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(MUTED_COLOR))
    );
    
    f.render_widget(table, area);
}

fn draw_history(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);
    
    // Summary stats
    let total_bets = app.bet_history.len();
    let total_won = app.bet_history.iter().filter(|o| o.won).count();
    let total_wagered: u64 = app.bet_history.iter().map(|o| o.bet.amount).sum();
    let total_payout: u64 = app.bet_history.iter().map(|o| o.payout).sum();
    
    let summary = Paragraph::new(format!(
        "Total Bets: {} | Won: {} | Wagered: {:.2} | Payout: {:.2} | Net: {:.2}",
        total_bets,
        total_won,
        total_wagered as f64 / 1_000_000_000.0,
        total_payout as f64 / 1_000_000_000.0,
        (total_payout as i64 - total_wagered as i64) as f64 / 1_000_000_000.0
    ))
    .style(Style::default().fg(FG_COLOR))
    .block(Block::default().borders(Borders::BOTTOM));
    
    f.render_widget(summary, chunks[0]);
    
    // Detailed history
    let headers = vec!["Epoch", "Bet Type", "Amount", "Result", "Payout"];
    
    let rows: Vec<Row> = app.bet_history
        .iter()
        .rev()
        .map(|outcome| {
            Row::new(vec![
                outcome.bet.epoch.to_string(),
                outcome.bet.bet_type.to_string(),
                format!("{:.2}", outcome.bet.amount as f64 / 1_000_000_000.0),
                if outcome.won { "WIN" } else { "LOSS" }.to_string(),
                format!("{:.2}", outcome.payout as f64 / 1_000_000_000.0),
            ])
            .style(if outcome.won {
                Style::default().fg(WIN_COLOR)
            } else {
                Style::default().fg(LOSS_COLOR)
            })
        })
        .collect();
    
    let table = Table::new(rows, vec![
        Constraint::Percentage(15),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
    ])
    .header(
        Row::new(headers)
            .style(Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD))
            .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(" BET HISTORY ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(ACCENT_COLOR))
    );
    
    f.render_widget(table, chunks[1]);
}

fn draw_statistics(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    // Left: Lifetime stats
    let lifetime_stats = vec![
        ("Total Sessions", app.statistics.lifetime.total_sessions.to_string()),
        ("Total Bets", app.statistics.lifetime.total_bets.to_string()),
        ("Total Wagered", format!("{:.2} CRAP", app.statistics.lifetime.total_wagered as f64 / 1_000_000_000.0)),
        ("Total Profit", format!("{:.2} CRAP", app.statistics.lifetime.total_profit as f64 / 1_000_000_000.0)),
        ("Win Rate", format!("{:.1}%", app.statistics.lifetime.win_rate * 100.0)),
        ("Average Bet", format!("{:.2} CRAP", app.statistics.lifetime.average_bet / 1_000_000_000.0)),
        ("ROI", format!("{:.1}%", app.statistics.lifetime.roi)),
        ("Best Session", format!("{:.2} CRAP", app.statistics.lifetime.best_session_profit as f64 / 1_000_000_000.0)),
        ("Worst Session", format!("{:.2} CRAP", app.statistics.lifetime.worst_session_loss as f64 / 1_000_000_000.0)),
    ];
    
    let lifetime_text: Vec<Line> = lifetime_stats
        .iter()
        .map(|(label, value)| {
            Line::from(vec![
                Span::styled(format!("{}: ", label), Style::default().fg(MUTED_COLOR)),
                Span::styled(value, Style::default().fg(FG_COLOR)),
            ])
        })
        .collect();
    
    let lifetime_widget = Paragraph::new(lifetime_text)
        .block(
            Block::default()
                .title(" LIFETIME STATISTICS ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(ACCENT_COLOR))
        );
    
    f.render_widget(lifetime_widget, chunks[0]);
    
    // Right: Roll distribution
    let roll_dist_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(15), Constraint::Min(0)])
        .split(chunks[1]);
    
    // Roll frequency chart
    let mut roll_data = vec![];
    for num in 2..=12 {
        let count = app.statistics.hot_numbers.rolls.get(&num).copied().unwrap_or(0);
        let total_rolls: u32 = app.statistics.hot_numbers.rolls.values().sum();
        let percentage = if total_rolls > 0 {
            (count as f64 / total_rolls as f64) * 100.0
        } else {
            0.0
        };
        
        roll_data.push((num, percentage, count));
    }
    
    let roll_text: Vec<Line> = roll_data
        .iter()
        .map(|(num, pct, count)| {
            let bar_width = (*pct / 20.0 * 20.0) as usize;
            let bar = "█".repeat(bar_width.min(20));
            
            Line::from(vec![
                Span::styled(format!("{:2} ", num), Style::default().fg(ACCENT_COLOR)),
                Span::styled(bar, Style::default().fg(WIN_COLOR)),
                Span::styled(format!(" {:.1}% ({})", pct, count), Style::default().fg(MUTED_COLOR)),
            ])
        })
        .collect();
    
    let roll_widget = Paragraph::new(roll_text)
        .block(
            Block::default()
                .title(" ROLL DISTRIBUTION ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(WIN_COLOR))
        );
    
    f.render_widget(roll_widget, roll_dist_chunks[0]);
}

fn draw_settings(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Theme
            Constraint::Length(10), // Betting
            Constraint::Min(0),     // Advanced
        ])
        .split(area);
    
    // Theme settings
    let theme_settings = vec![
        ("Color Scheme", match app.config.theme.color_scheme {
            crate::config::ColorScheme::Professional => "Professional",
            crate::config::ColorScheme::HighContrast => "High Contrast",
            crate::config::ColorScheme::Casino => "Casino",
            crate::config::ColorScheme::Matrix => "Matrix",
        }.to_string()),
        ("Animations", if app.config.theme.animations_enabled { "Enabled" } else { "Disabled" }.to_string()),
        ("Sound", if app.config.theme.sound_enabled { "Enabled" } else { "Disabled" }.to_string()),
    ];
    
    let theme_text: Vec<Line> = theme_settings
        .iter()
        .map(|(label, value)| {
            Line::from(vec![
                Span::styled(format!("{}: ", label), Style::default().fg(MUTED_COLOR)),
                Span::styled(value, Style::default().fg(FG_COLOR)),
            ])
        })
        .collect();
    
    let theme_widget = Paragraph::new(theme_text)
        .block(
            Block::default()
                .title(" THEME ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(ACCENT_COLOR))
        );
    
    f.render_widget(theme_widget, chunks[0]);
    
    // Betting settings
    let betting_settings = vec![
        ("Default Bet", format!("{} CRAP", app.config.betting.default_bet_amount)),
        ("Auto Repeat", if app.config.betting.auto_repeat_last_bet { "On" } else { "Off" }.to_string()),
        ("Martingale", if app.config.betting.martingale_enabled { "Enabled" } else { "Disabled" }.to_string()),
        ("Stop Loss", app.config.betting.stop_loss.map_or("None".to_string(), |v| format!("{} CRAP", v))),
        ("Stop Win", app.config.betting.stop_win.map_or("None".to_string(), |v| format!("{} CRAP", v))),
    ];
    
    let betting_text: Vec<Line> = betting_settings
        .iter()
        .map(|(label, value)| {
            Line::from(vec![
                Span::styled(format!("{}: ", label), Style::default().fg(MUTED_COLOR)),
                Span::styled(value, Style::default().fg(FG_COLOR)),
            ])
        })
        .collect();
    
    let betting_widget = Paragraph::new(betting_text)
        .block(
            Block::default()
                .title(" BETTING ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(ACCENT_COLOR))
        );
    
    f.render_widget(betting_widget, chunks[1]);
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(Span::styled("KEYBOARD SHORTCUTS", Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  TAB     - Cycle through modes"),
        Line::from("  ESC     - Back / Cancel"),
        Line::from("  Q       - Quit application"),
        Line::from(""),
        Line::from("Betting:"),
        Line::from("  P       - Pass Line"),
        Line::from("  D       - Don't Pass"),
        Line::from("  F       - Field"),
        Line::from("  C       - Come"),
        Line::from("  4-10    - Number bets"),
        Line::from("  Ctrl+O  - Odds bet"),
        Line::from("  Ctrl+R  - Repeat last bet"),
        Line::from(""),
        Line::from("Quick Amounts:"),
        Line::from("  F2      - 100 CRAP"),
        Line::from("  F3      - 500 CRAP"),
        Line::from("  F4      - 1,000 CRAP"),
        Line::from("  F5      - 5,000 CRAP"),
    ];
    
    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" HELP ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(ACCENT_COLOR))
        )
        .style(Style::default().fg(FG_COLOR))
        .alignment(Alignment::Left);
    
    f.render_widget(help, area);
}