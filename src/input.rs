//! Vim-style input handling.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Mode};

/// Handle a key event.
pub fn handle_key(app: &mut App, key: KeyEvent, page_size: usize) {
    // Close help overlay on any keypress
    if app.show_help {
        app.show_help = false;
        return;
    }

    match app.mode {
        Mode::Normal => handle_normal_mode(app, key, page_size),
        Mode::Insert => handle_insert_mode(app, key),
        Mode::Command => handle_command_mode(app, key),
    }
}

/// Handle keys in normal mode.
fn handle_normal_mode(app: &mut App, key: KeyEvent, page_size: usize) {
    app.clear_status();

    match (key.modifiers, key.code) {
        // Quit
        (KeyModifiers::NONE, KeyCode::Char('q')) => {
            if app.modified {
                app.set_status("No write since last change (use :q! to force)");
            } else {
                app.should_quit = true;
            }
        }

        // Movement - basic
        (KeyModifiers::NONE, KeyCode::Char('h')) | (KeyModifiers::NONE, KeyCode::Left) => {
            app.cursor_left();
        }
        (KeyModifiers::NONE, KeyCode::Char('j')) | (KeyModifiers::NONE, KeyCode::Down) => {
            app.cursor_down();
        }
        (KeyModifiers::NONE, KeyCode::Char('k')) | (KeyModifiers::NONE, KeyCode::Up) => {
            app.cursor_up();
        }
        (KeyModifiers::NONE, KeyCode::Char('l')) | (KeyModifiers::NONE, KeyCode::Right) => {
            app.cursor_right();
        }

        // Movement - line
        (KeyModifiers::NONE, KeyCode::Char('0')) => {
            app.cursor_line_start();
        }
        (KeyModifiers::NONE, KeyCode::Char('$')) | (KeyModifiers::SHIFT, KeyCode::Char('$')) => {
            app.cursor_line_end();
        }
        (KeyModifiers::NONE, KeyCode::Home) => {
            app.cursor_line_start();
        }
        (KeyModifiers::NONE, KeyCode::End) => {
            app.cursor_line_end();
        }

        // Movement - document
        (KeyModifiers::NONE, KeyCode::Char('g')) => {
            // Waiting for second 'g'
            app.set_status("g...");
        }
        (KeyModifiers::SHIFT, KeyCode::Char('G')) => {
            app.cursor_last_sequence();
        }

        // Movement - scrolling
        (KeyModifiers::CONTROL, KeyCode::Char('f')) | (KeyModifiers::NONE, KeyCode::PageDown) => {
            app.page_down(page_size);
        }
        (KeyModifiers::CONTROL, KeyCode::Char('b')) | (KeyModifiers::NONE, KeyCode::PageUp) => {
            app.page_up(page_size);
        }
        (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
            app.half_page_down(page_size);
        }
        (KeyModifiers::CONTROL, KeyCode::Char('u')) => {
            app.half_page_up(page_size);
        }

        // Movement - word-like (jump by 10 columns)
        (KeyModifiers::NONE, KeyCode::Char('w')) => {
            app.scroll_right(10);
        }
        (KeyModifiers::NONE, KeyCode::Char('b')) => {
            app.scroll_left(10);
        }

        // Go to pair
        (KeyModifiers::NONE, KeyCode::Char('p')) => {
            // Check if previous key was 'g'
            if app.status_message.as_deref() == Some("g...") {
                app.goto_pair();
                app.clear_status();
            }
        }

        // Insert mode
        (KeyModifiers::NONE, KeyCode::Char('i')) => {
            app.enter_insert_mode();
        }

        // Delete gap
        (KeyModifiers::NONE, KeyCode::Char('x')) => {
            app.delete_gap();
        }

        // Insert gap column
        (KeyModifiers::SHIFT, KeyCode::Char('I')) => {
            app.insert_gap_column();
        }

        // Delete gap column
        (KeyModifiers::SHIFT, KeyCode::Char('X')) => {
            app.delete_gap_column();
        }

        // Shift sequence
        (KeyModifiers::SHIFT, KeyCode::Char('<')) => {
            app.shift_sequence_left();
        }
        (KeyModifiers::SHIFT, KeyCode::Char('>')) => {
            app.shift_sequence_right();
        }

        // Throw sequence
        (KeyModifiers::SHIFT, KeyCode::Char('{')) => {
            app.throw_sequence_left();
        }
        (KeyModifiers::SHIFT, KeyCode::Char('}')) => {
            app.throw_sequence_right();
        }

        // Undo/Redo
        (KeyModifiers::NONE, KeyCode::Char('u')) => {
            app.undo();
        }
        (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
            app.redo();
        }

        // Command mode
        (KeyModifiers::NONE, KeyCode::Char(':')) | (KeyModifiers::SHIFT, KeyCode::Char(':')) => {
            app.enter_command_mode();
        }

        // Delete line
        (KeyModifiers::NONE, KeyCode::Char('d')) => {
            // Waiting for second 'd'
            app.set_status("d...");
        }

        // Help
        (KeyModifiers::SHIFT, KeyCode::Char('?')) => {
            app.toggle_help();
        }

        _ => {}
    }

    // Handle two-key sequences
    if let Some(status) = &app.status_message.clone() {
        match (status.as_str(), key.code) {
            ("g...", KeyCode::Char('g')) => {
                app.cursor_first_sequence();
                app.clear_status();
            }
            ("g...", KeyCode::Char('p')) => {
                app.goto_pair();
                app.clear_status();
            }
            ("d...", KeyCode::Char('d')) => {
                app.delete_sequence();
                app.clear_status();
            }
            _ => {}
        }
    }
}

/// Handle keys in insert mode.
fn handle_insert_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.enter_normal_mode();
        }
        KeyCode::Char('.') | KeyCode::Char('-') => {
            app.insert_gap();
        }
        KeyCode::Backspace => {
            // Delete gap behind cursor
            if app.cursor_col > 0 {
                app.cursor_left();
                app.delete_gap();
            }
        }
        KeyCode::Left => {
            app.cursor_left();
        }
        KeyCode::Right => {
            app.cursor_right();
        }
        KeyCode::Up => {
            app.cursor_up();
        }
        KeyCode::Down => {
            app.cursor_down();
        }
        _ => {}
    }
}

/// Handle keys in command mode.
fn handle_command_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.enter_normal_mode();
        }
        KeyCode::Enter => {
            app.execute_command();
        }
        KeyCode::Backspace => {
            app.command_buffer.pop();
            if app.command_buffer.is_empty() {
                app.enter_normal_mode();
            }
        }
        KeyCode::Up => {
            app.command_history_prev();
        }
        KeyCode::Down => {
            app.command_history_next();
        }
        KeyCode::Char(c) => {
            app.command_buffer.push(c);
        }
        _ => {}
    }
}

