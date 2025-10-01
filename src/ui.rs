use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Gauge, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};

use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        Screen::Menu => draw_menu(f, app),
        Screen::Test => draw_test(f, app),
        Screen::Results => draw_results(f, app),
        Screen::History => draw_history(f, app),
        Screen::ModeSelection => draw_mode_selection(f, app),
    }
}

fn draw_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    // Title
    let title = Paragraph::new("Terminal Typing Test")
        .style(Style::default().fg(app.config.theme.accent()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );
    f.render_widget(title, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let menu_items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_menu_item {
                Style::default()
                    .fg(app.config.theme.highlight())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.config.theme.text())
            };
            ListItem::new(format!("{}. {}", i + 1, item)).style(style)
        })
        .collect();

    let menu_list = List::new(menu_items)
        .block(
            Block::default()
                .title("Menu")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        )
        .style(Style::default().fg(app.config.theme.text()));

    f.render_widget(menu_list, main_chunks[0]);

    let mode_info = vec![
        Line::from(vec![
            Span::styled("Current Mode: ", Style::default().fg(app.config.theme.text())),
            Span::styled(
                app.current_mode.display_name(),
                Style::default().fg(app.config.theme.accent()),
            ),
        ]),
        Line::from(""),
        Line::from("Keybindings:"),
        Line::from("  ↑/↓ or j/k - Navigate"),
        Line::from("  Enter - Select"),
        Line::from("  1-4 - Quick select"),
        Line::from("  q - Quit"),
    ];

    let info_panel = Paragraph::new(mode_info)
        .block(
            Block::default()
                .title("Info")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        )
        .style(Style::default().fg(app.config.theme.text()))
        .wrap(Wrap { trim: true });

    f.render_widget(info_panel, main_chunks[1]);

    // Footer
    let footer = Paragraph::new("Press Enter to start, or use number keys for quick selection")
        .style(Style::default().fg(app.config.theme.muted()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );
    f.render_widget(footer, chunks[2]);
}

fn draw_mode_selection(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    // Title
    let title = Paragraph::new("⚙️ Select Test Mode")
        .style(Style::default().fg(app.config.theme.accent()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );
    f.render_widget(title, chunks[0]);

    // Mode list
    let mode_items: Vec<ListItem> = app
        .available_modes
        .iter()
        .enumerate()
        .map(|(i, mode)| {
            let style = if i == app.selected_mode_index {
                Style::default()
                    .fg(app.config.theme.highlight())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.config.theme.text())
            };
            
            let indicator = if i == app.selected_mode_index { "► " } else { "  " };
            let display = format!("{}{}", indicator, mode.display_name());
            
            ListItem::new(display).style(style)
        })
        .collect();

    let mode_list = List::new(mode_items)
        .block(
            Block::default()
                .title("Available Modes")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        )
        .style(Style::default().fg(app.config.theme.text()));

    f.render_widget(mode_list, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("↑/↓ to navigate, Enter to select, Esc/M to return to menu")
        .style(Style::default().fg(app.config.theme.muted()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );
    f.render_widget(instructions, chunks[2]);
}

fn draw_test(f: &mut Frame, app: &mut App) {
    if let Some(test) = &app.test {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Title with mode
        let title = format!("Typing Test - {}", app.current_mode.display_name());
        let title_widget = Paragraph::new(title)
            .style(Style::default().fg(app.config.theme.accent()))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            );
        f.render_widget(title_widget, chunks[0]);

        // Progress and live stats
        draw_test_progress(f, chunks[1], app, test);

        // Text area
        draw_text_area(f, chunks[2], app, test);

        // Instructions
        let instructions = Paragraph::new("Type the text above. Press Esc to return to menu.")
            .style(Style::default().fg(app.config.theme.muted()))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            );
        f.render_widget(instructions, chunks[3]);
    }
}

fn draw_test_progress(f: &mut Frame, area: Rect, app: &App, test: &crate::test::Test) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Progress bar
    let (progress, progress_label) = match &app.current_mode {
        crate::app::TestMode::Timed(duration) => {
            let elapsed = test.elapsed_time().as_secs() as f64;
            let total = *duration as f64;
            let ratio = (elapsed / total).min(1.0);
            let remaining = (total - elapsed).max(0.0) as u32;
            (ratio, format!("Time: {}s", remaining))
        }
        crate::app::TestMode::WordCount(target) => {
            let typed = app.input_handler.get_typed_words();
            let ratio = (typed as f64 / *target as f64).min(1.0);
            (ratio, format!("Words: {}/{}", typed, target))
        }
        crate::app::TestMode::Text(_) => {
            let progress = app.input_handler.get_progress(&test.get_text());
            (progress, format!("Progress: {:.1}%", progress * 100.0))
        }
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        )
        .gauge_style(Style::default().fg(app.config.theme.accent()))
        .ratio(progress)
        .label(progress_label);

    f.render_widget(gauge, chunks[0]);

    // Live stats
    let stats = app.input_handler.get_live_stats(test.elapsed_time());
    let stats_text = vec![
        Line::from(format!("WPM: {:.0}", stats.wpm)),
        Line::from(format!("Accuracy: {:.1}%", stats.accuracy * 100.0)),
    ];

    let stats_widget = Paragraph::new(stats_text)
        .style(Style::default().fg(app.config.theme.text()))
        .block(
            Block::default()
                .title("Live Stats")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );

    f.render_widget(stats_widget, chunks[1]);
}

fn draw_text_area(f: &mut Frame, area: Rect, app: &App, test: &crate::test::Test) {
    let text = test.get_text();
    let typed = app.input_handler.get_typed_text();
    let current_pos = typed.chars().count();
    let mut spans = Vec::new();

    let text_chars: Vec<char> = text.chars().collect();
    let typed_chars: Vec<char> = typed.chars().collect();

    for (i, &ch) in text_chars.iter().enumerate() {
        let style = if i < typed_chars.len() {
            let typed_char = typed_chars[i];
            if typed_char == ch {
                Style::default().fg(app.config.theme.correct())
            } else {
                Style::default().fg(app.config.theme.error()).bg(Color::Red)
            }
        } else if i == current_pos {
            Style::default()
                .fg(app.config.theme.text())
                .bg(app.config.theme.cursor())
                .add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default().fg(app.config.theme.muted())
        };

        spans.push(Span::styled(ch.to_string(), style));
    }

    let text_paragraph = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .title("Text to Type")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        )
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    f.render_widget(text_paragraph, area);
}

fn draw_results(f: &mut Frame, app: &App) {
    if let Some(stats) = &app.last_stats {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        let title = Paragraph::new("🎉 Test Results")
            .style(Style::default().fg(app.config.theme.accent()))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            );
        f.render_widget(title, chunks[0]);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let primary_stats = vec![
            Line::from(vec![
                Span::styled("WPM: ", Style::default().fg(app.config.theme.text())),
                Span::styled(
                    format!("{:.0}", stats.wpm),
                    Style::default()
                        .fg(app.config.theme.accent())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Raw WPM: ", Style::default().fg(app.config.theme.text())),
                Span::styled(
                    format!("{:.0}", stats.raw_wpm),
                    Style::default().fg(app.config.theme.text()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Accuracy: ", Style::default().fg(app.config.theme.text())),
                Span::styled(
                    format!("{:.1}%", stats.accuracy * 100.0),
                    Style::default().fg(if stats.accuracy > 0.95 {
                        app.config.theme.correct()
                    } else if stats.accuracy > 0.90 {
                        app.config.theme.accent()
                    } else {
                        app.config.theme.error()
                    }),
                ),
            ]),
            Line::from(vec![
                Span::styled("Errors: ", Style::default().fg(app.config.theme.text())),
                Span::styled(
                    stats.error_count.to_string(),
                    Style::default().fg(app.config.theme.error()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Characters: ", Style::default().fg(app.config.theme.text())),
                Span::styled(
                    format!("{}/{}", stats.correct_chars, stats.total_chars),
                    Style::default().fg(app.config.theme.text()),
                ),
            ]),
        ];

        let primary_panel = Paragraph::new(primary_stats)
            .block(
                Block::default()
                    .title("Statistics")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            )
            .style(Style::default().fg(app.config.theme.text()));

        f.render_widget(primary_panel, main_chunks[0]);

        let duration_text = if stats.test_duration.as_secs() > 0 {
            format!("{:.1}s", stats.test_duration.as_secs_f64())
        } else {
            "< 1s".to_string()
        };

        let additional_info = vec![
            Line::from(vec![
                Span::styled("Duration: ", Style::default().fg(app.config.theme.text())),
                Span::styled(duration_text, Style::default().fg(app.config.theme.text())),
            ]),
            Line::from(vec![
                Span::styled("Mode: ", Style::default().fg(app.config.theme.text())),
                Span::styled(
                    app.current_mode.display_name(),
                    Style::default().fg(app.config.theme.accent()),
                ),
            ]),
            Line::from(""),
            Line::from("Most Common Errors:"),
        ];
        let mut info_lines = additional_info;
        let mut error_list: Vec<_> = stats.error_frequency.iter().collect();
        error_list.sort_by_key(|&(_, &count)| std::cmp::Reverse(count));

        for (_i, &(ch, count)) in error_list.iter().take(5).enumerate() {
            if *count > 0 {
                info_lines.push(Line::from(format!("  {}: {} times", ch, count)));
            }
        }

        let additional_panel = Paragraph::new(info_lines)
            .block(
                Block::default()
                    .title("Details")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            )
            .style(Style::default().fg(app.config.theme.text()));

        f.render_widget(additional_panel, main_chunks[1]);

        let instructions = Paragraph::new("Press R to restart test, M to return to menu")
            .style(Style::default().fg(app.config.theme.muted()))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            );
        f.render_widget(instructions, chunks[2]);
    }
}

fn draw_history(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("📊 Test History")
        .style(Style::default().fg(app.config.theme.accent()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );
    f.render_widget(title, chunks[0]);

    let results = app.history.get_results();
    if results.is_empty() {
        let empty_msg = Paragraph::new("No test results yet. Complete a test to see your history!")
            .style(Style::default().fg(app.config.theme.muted()))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            );
        f.render_widget(empty_msg, chunks[1]);
    } else {
        let history_items: Vec<ListItem> = results
            .iter()
            .enumerate()
            .map(|(i, result)| {
                let style = if i == app.selected_history_item {
                    Style::default()
                        .fg(app.config.theme.highlight())
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.config.theme.text())
                };

                let date_str = result.timestamp.format("%Y-%m-%d %H:%M").to_string();
                let content = format!(
                    "{} | WPM: {:.0} | Acc: {:.1}% | Mode: {}",
                    date_str,
                    result.wpm,
                    result.accuracy * 100.0,
                    match result.test_mode.as_str() {
                        mode if mode.starts_with("Timed") => mode,
                        mode if mode.starts_with("WordCount") => mode,
                        _ => "Custom",
                    }
                );

                ListItem::new(content).style(style)
            })
            .collect();

        let history_list = List::new(history_items)
            .block(
                Block::default()
                    .title(format!("History ({} tests)", results.len()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.config.theme.border())),
            )
            .style(Style::default().fg(app.config.theme.text()));

        f.render_widget(history_list, chunks[1]);
    }

    let instructions = Paragraph::new("↑/↓ to navigate, M or Esc to return to menu")
        .style(Style::default().fg(app.config.theme.muted()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.config.theme.border())),
        );
    f.render_widget(instructions, chunks[2]);
}