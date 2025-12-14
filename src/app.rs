//! Application state and main loop.

use std::path::PathBuf;

use crate::editor::History;
use crate::stockholm::Alignment;
use crate::structure::StructureCache;

/// Editor mode (vim-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Command,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Command => "COMMAND",
        }
    }
}

/// Color scheme for the alignment display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    #[default]
    None,
    /// Color by secondary structure (helix coloring).
    Structure,
    /// Color by base identity (A, C, G, U).
    Base,
    /// Color by conservation.
    Conservation,
    /// Color by compensatory changes.
    Compensatory,
}

impl ColorScheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorScheme::None => "none",
            ColorScheme::Structure => "structure",
            ColorScheme::Base => "base",
            ColorScheme::Conservation => "conservation",
            ColorScheme::Compensatory => "compensatory",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "none" | "off" => Some(ColorScheme::None),
            "structure" | "ss" => Some(ColorScheme::Structure),
            "base" | "nt" => Some(ColorScheme::Base),
            "conservation" | "cons" => Some(ColorScheme::Conservation),
            "compensatory" | "comp" => Some(ColorScheme::Compensatory),
            _ => None,
        }
    }
}

/// Application state.
pub struct App {
    /// Current alignment.
    pub alignment: Alignment,
    /// File path (if loaded from file).
    pub file_path: Option<PathBuf>,
    /// Whether the alignment has been modified.
    pub modified: bool,

    /// Current cursor row (sequence index).
    pub cursor_row: usize,
    /// Current cursor column.
    pub cursor_col: usize,

    /// Viewport offset (row).
    pub viewport_row: usize,
    /// Viewport offset (column).
    pub viewport_col: usize,

    /// Current editor mode.
    pub mode: Mode,
    /// Command line buffer (for command mode).
    pub command_buffer: String,
    /// Status message.
    pub status_message: Option<String>,

    /// Gap character.
    pub gap_char: char,
    /// Characters considered as gaps.
    pub gap_chars: Vec<char>,

    /// Color scheme.
    pub color_scheme: ColorScheme,

    /// Structure cache.
    pub structure_cache: StructureCache,

    /// Undo/redo history.
    pub history: History,

    /// Should quit.
    pub should_quit: bool,

    /// Reference sequence index for compensatory coloring.
    pub reference_seq: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            alignment: Alignment::new(),
            file_path: None,
            modified: false,
            cursor_row: 0,
            cursor_col: 0,
            viewport_row: 0,
            viewport_col: 0,
            mode: Mode::Normal,
            command_buffer: String::new(),
            status_message: None,
            gap_char: '.',
            gap_chars: vec!['.', '-', '_', '~', ':'],
            color_scheme: ColorScheme::None,
            structure_cache: StructureCache::new(),
            history: History::new(),
            should_quit: false,
            reference_seq: 0,
        }
    }
}

impl App {
    /// Create a new app with default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load an alignment from a file.
    pub fn load_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let alignment = crate::stockholm::parser::parse_file(path)
            .map_err(|e| format!("Failed to parse file: {}", e))?;

        self.alignment = alignment;
        self.file_path = Some(path.clone());
        self.modified = false;
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.viewport_row = 0;
        self.viewport_col = 0;
        self.history.clear();

        // Update structure cache
        if let Some(ss) = self.alignment.ss_cons() {
            let _ = self.structure_cache.update(ss);
        }

        self.set_status(format!("Loaded {}", path.display()));
        Ok(())
    }

    /// Save the alignment to a file.
    pub fn save_file(&mut self) -> Result<(), String> {
        let path = self.file_path.as_ref().ok_or("No file path set")?;
        crate::stockholm::writer::write_file(&self.alignment, path)
            .map_err(|e| format!("Failed to save file: {}", e))?;
        self.modified = false;
        self.set_status(format!("Saved {}", path.display()));
        Ok(())
    }

    /// Save the alignment to a new file.
    pub fn save_file_as(&mut self, path: PathBuf) -> Result<(), String> {
        crate::stockholm::writer::write_file(&self.alignment, &path)
            .map_err(|e| format!("Failed to save file: {}", e))?;
        self.file_path = Some(path.clone());
        self.modified = false;
        self.set_status(format!("Saved {}", path.display()));
        Ok(())
    }

    /// Set a status message.
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    /// Clear the status message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Get the current character under the cursor.
    pub fn current_char(&self) -> Option<char> {
        self.alignment.get_char(self.cursor_row, self.cursor_col)
    }

    /// Check if the current character is a gap.
    pub fn is_current_gap(&self) -> bool {
        self.current_char()
            .map(|c| self.gap_chars.contains(&c))
            .unwrap_or(false)
    }

    /// Move cursor up.
    pub fn cursor_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
        }
    }

    /// Move cursor down.
    pub fn cursor_down(&mut self) {
        if self.cursor_row < self.alignment.num_sequences().saturating_sub(1) {
            self.cursor_row += 1;
        }
    }

    /// Move cursor left.
    pub fn cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    /// Move cursor right.
    pub fn cursor_right(&mut self) {
        if self.cursor_col < self.alignment.width().saturating_sub(1) {
            self.cursor_col += 1;
        }
    }

    /// Move cursor to start of line.
    pub fn cursor_line_start(&mut self) {
        self.cursor_col = 0;
    }

    /// Move cursor to end of line.
    pub fn cursor_line_end(&mut self) {
        self.cursor_col = self.alignment.width().saturating_sub(1);
    }

    /// Move cursor to first sequence.
    pub fn cursor_first_sequence(&mut self) {
        self.cursor_row = 0;
    }

    /// Move cursor to last sequence.
    pub fn cursor_last_sequence(&mut self) {
        self.cursor_row = self.alignment.num_sequences().saturating_sub(1);
    }

    /// Jump to paired base.
    pub fn goto_pair(&mut self) {
        if let Some(paired) = self.structure_cache.get_pair(self.cursor_col) {
            self.cursor_col = paired;
        }
    }

    /// Page down.
    pub fn page_down(&mut self, page_size: usize) {
        let max_row = self.alignment.num_sequences().saturating_sub(1);
        self.cursor_row = (self.cursor_row + page_size).min(max_row);
    }

    /// Page up.
    pub fn page_up(&mut self, page_size: usize) {
        self.cursor_row = self.cursor_row.saturating_sub(page_size);
    }

    /// Half page down.
    pub fn half_page_down(&mut self, page_size: usize) {
        self.page_down(page_size / 2);
    }

    /// Half page up.
    pub fn half_page_up(&mut self, page_size: usize) {
        self.page_up(page_size / 2);
    }

    /// Scroll right.
    pub fn scroll_right(&mut self, amount: usize) {
        let max_col = self.alignment.width().saturating_sub(1);
        self.cursor_col = (self.cursor_col + amount).min(max_col);
    }

    /// Scroll left.
    pub fn scroll_left(&mut self, amount: usize) {
        self.cursor_col = self.cursor_col.saturating_sub(amount);
    }

    /// Enter insert mode.
    pub fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }

    /// Enter command mode.
    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.command_buffer.clear();
    }

    /// Return to normal mode.
    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.command_buffer.clear();
    }

    /// Execute a command from command mode.
    pub fn execute_command(&mut self) {
        let command = self.command_buffer.trim().to_string();
        self.command_buffer.clear();
        self.mode = Mode::Normal;

        if command.is_empty() {
            return;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts.as_slice() {
            ["q"] | ["quit"] => {
                if self.modified {
                    self.set_status("No write since last change (use :q! to force)");
                } else {
                    self.should_quit = true;
                }
            }
            ["q!"] => {
                self.should_quit = true;
            }
            ["w"] | ["write"] => {
                if let Err(e) = self.save_file() {
                    self.set_status(e);
                }
            }
            ["w", path] => {
                if let Err(e) = self.save_file_as(PathBuf::from(path)) {
                    self.set_status(e);
                }
            }
            ["wq"] => {
                if let Err(e) = self.save_file() {
                    self.set_status(e);
                } else {
                    self.should_quit = true;
                }
            }
            ["color", scheme] => {
                if let Some(s) = ColorScheme::from_str(scheme) {
                    self.color_scheme = s;
                    self.set_status(format!("Color scheme: {}", s.as_str()));
                } else {
                    self.set_status(format!("Unknown color scheme: {}", scheme));
                }
            }
            ["set", setting] => {
                if let Some((key, value)) = setting.split_once('=') {
                    match key {
                        "gap" => {
                            if let Some(c) = value.chars().next() {
                                self.gap_char = c;
                                self.set_status(format!("Gap character: '{}'", c));
                            }
                        }
                        _ => {
                            self.set_status(format!("Unknown setting: {}", key));
                        }
                    }
                }
            }
            ["fold"] => {
                self.fold_current_sequence();
            }
            ["alifold"] => {
                self.fold_alignment();
            }
            _ => {
                self.set_status(format!("Unknown command: {}", command));
            }
        }
    }

    /// Fold current sequence using RNAfold.
    fn fold_current_sequence(&mut self) {
        self.set_status("RNAfold integration not yet implemented");
    }

    /// Fold alignment using RNAalifold.
    fn fold_alignment(&mut self) {
        self.set_status("RNAalifold integration not yet implemented");
    }

    /// Mark the alignment as modified.
    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    /// Update the structure cache if needed.
    pub fn update_structure_cache(&mut self) {
        if let Some(ss) = self.alignment.ss_cons() {
            if !self.structure_cache.is_valid_for(ss) {
                let _ = self.structure_cache.update(ss);
            }
        }
    }

    /// Ensure cursor is within bounds.
    pub fn clamp_cursor(&mut self) {
        let max_row = self.alignment.num_sequences().saturating_sub(1);
        let max_col = self.alignment.width().saturating_sub(1);
        self.cursor_row = self.cursor_row.min(max_row);
        self.cursor_col = self.cursor_col.min(max_col);
    }

    /// Adjust viewport to keep cursor visible.
    pub fn adjust_viewport(&mut self, visible_rows: usize, visible_cols: usize) {
        // Vertical scrolling
        if self.cursor_row < self.viewport_row {
            self.viewport_row = self.cursor_row;
        } else if self.cursor_row >= self.viewport_row + visible_rows {
            self.viewport_row = self.cursor_row - visible_rows + 1;
        }

        // Horizontal scrolling
        if self.cursor_col < self.viewport_col {
            self.viewport_col = self.cursor_col;
        } else if self.cursor_col >= self.viewport_col + visible_cols {
            self.viewport_col = self.cursor_col - visible_cols + 1;
        }
    }
}
