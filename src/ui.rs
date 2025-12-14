//! TUI rendering with ratatui.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, ColorScheme, Mode};
use crate::color::get_color;

/// Render the application UI.
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // Alignment view
            Constraint::Length(1), // Status bar
            Constraint::Length(1), // Command/message line
        ])
        .split(frame.area());

    render_alignment(frame, app, chunks[0]);
    render_status_bar(frame, app, chunks[1]);
    render_command_line(frame, app, chunks[2]);
}

/// Render the alignment view.
fn render_alignment(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " {} {} ",
            app.file_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "[No file]".to_string()),
            if app.modified { "[+]" } else { "" }
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.alignment.sequences.is_empty() {
        let msg = Paragraph::new("No alignment loaded. Use :e <file> to open a file.");
        frame.render_widget(msg, inner);
        return;
    }

    // Calculate visible area
    let max_id_len = app.alignment.max_id_len().max(10);
    let id_width = max_id_len + 2; // padding
    let seq_width = (inner.width as usize).saturating_sub(id_width);
    let visible_rows = inner.height as usize;

    // Adjust viewport
    let viewport_row = app.viewport_row;
    let viewport_col = app.viewport_col;

    let mut lines = Vec::new();

    // Render sequences
    for row in viewport_row..(viewport_row + visible_rows).min(app.alignment.num_sequences()) {
        let seq = &app.alignment.sequences[row];
        let mut spans = Vec::new();

        // Sequence ID
        let id_style = if row == app.cursor_row {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        let id_display = format!("{:width$}", seq.id, width = id_width);
        spans.push(Span::styled(id_display, id_style));

        // Sequence data
        let seq_chars: Vec<char> = seq.data.chars().collect();
        for col in viewport_col..(viewport_col + seq_width).min(seq_chars.len()) {
            let ch = seq_chars[col];
            let is_cursor = row == app.cursor_row && col == app.cursor_col;

            let mut style = Style::default();

            // Apply color scheme
            if let Some(color) = get_color(
                app.color_scheme,
                ch,
                col,
                row,
                &app.alignment,
                &app.structure_cache,
                &app.gap_chars,
                app.reference_seq,
            ) {
                style = style.bg(color).fg(Color::Black);
            }

            // Highlight cursor
            if is_cursor {
                style = style.add_modifier(Modifier::REVERSED);
            }

            // Highlight paired position
            if app.color_scheme == ColorScheme::Structure {
                if let Some(paired) = app.structure_cache.get_pair(col) {
                    if app.cursor_col == paired {
                        style = style.add_modifier(Modifier::UNDERLINED);
                    }
                }
            }

            spans.push(Span::styled(ch.to_string(), style));
        }

        lines.push(Line::from(spans));
    }

    // Render SS_cons line if visible and we have room
    if lines.len() < visible_rows {
        if let Some(ss) = app.alignment.ss_cons() {
            let mut spans = Vec::new();
            let id_display = format!("{:width$}", "#=GC SS_cons", width = id_width);
            spans.push(Span::styled(id_display, Style::default().fg(Color::Yellow)));

            let ss_chars: Vec<char> = ss.chars().collect();
            for col in viewport_col..(viewport_col + seq_width).min(ss_chars.len()) {
                let ch = ss_chars[col];
                let is_cursor_col = col == app.cursor_col;

                let mut style = Style::default().fg(Color::Yellow);

                // Highlight if paired with cursor
                if let Some(paired) = app.structure_cache.get_pair(col) {
                    if app.cursor_col == paired || app.cursor_col == col {
                        style = style.add_modifier(Modifier::BOLD);
                    }
                }

                if is_cursor_col {
                    style = style.add_modifier(Modifier::REVERSED);
                }

                spans.push(Span::styled(ch.to_string(), style));
            }

            lines.push(Line::from(spans));
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render the status bar.
fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let mode_style = match app.mode {
        Mode::Normal => Style::default().bg(Color::Blue).fg(Color::White),
        Mode::Insert => Style::default().bg(Color::Green).fg(Color::Black),
        Mode::Command => Style::default().bg(Color::Yellow).fg(Color::Black),
    };

    let mode_span = Span::styled(format!(" {} ", app.mode.as_str()), mode_style);

    // Position info
    let pos_info = format!(
        " {}:{} ",
        app.cursor_row + 1,
        app.cursor_col + 1
    );

    // Alignment info
    let align_info = format!(
        " {}x{} ",
        app.alignment.num_sequences(),
        app.alignment.width()
    );

    // Color scheme
    let color_info = if app.color_scheme != ColorScheme::None {
        format!(" [{}] ", app.color_scheme.as_str())
    } else {
        String::new()
    };

    // Structure info
    let structure_info = if app.structure_cache.is_paired(app.cursor_col) {
        if let Some(paired) = app.structure_cache.get_pair(app.cursor_col) {
            format!(" pair:{} ", paired + 1)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Current character
    let char_info = app
        .current_char()
        .map(|c| format!(" '{}' ", c))
        .unwrap_or_default();

    let spans = vec![
        mode_span,
        Span::raw(pos_info),
        Span::styled(align_info, Style::default().fg(Color::Cyan)),
        Span::styled(color_info, Style::default().fg(Color::Magenta)),
        Span::styled(structure_info, Style::default().fg(Color::Yellow)),
        Span::raw(char_info),
    ];

    let status = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::DarkGray));

    frame.render_widget(status, area);
}

/// Render the command/message line.
fn render_command_line(frame: &mut Frame, app: &App, area: Rect) {
    let content = match app.mode {
        Mode::Command => {
            Line::from(vec![
                Span::styled(":", Style::default().fg(Color::Yellow)),
                Span::raw(&app.command_buffer),
                Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
            ])
        }
        _ => {
            if let Some(msg) = &app.status_message {
                Line::from(Span::raw(msg.as_str()))
            } else {
                // Show help hint
                Line::from(Span::styled(
                    "Press : for commands, ? for help",
                    Style::default().fg(Color::DarkGray),
                ))
            }
        }
    };

    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, area);
}

/// Calculate visible dimensions for the alignment area.
pub fn visible_dimensions(area: Rect, max_id_len: usize) -> (usize, usize) {
    let id_width = max_id_len.max(10) + 2;
    let inner_height = area.height.saturating_sub(4) as usize; // borders + status + command
    let inner_width = (area.width as usize).saturating_sub(id_width + 2); // borders
    (inner_height, inner_width)
}
