//! UI theme colors for the application.
//!
//! This module defines all UI element colors that can be customized via config.

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// RGB color representation for config serialization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn to_color(self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        rgb.to_color()
    }
}

/// Border colors for panes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BorderColors {
    pub active: Rgb,
    pub inactive: Rgb,
}

impl Default for BorderColors {
    fn default() -> Self {
        Self {
            active: Rgb::new(0, 255, 255),     // Cyan
            inactive: Rgb::new(128, 128, 128), // DarkGray
        }
    }
}

/// Ruler display colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RulerColors {
    pub numbers: Rgb,
    pub ticks: Rgb,
    pub pair_line: Rgb,
}

impl Default for RulerColors {
    fn default() -> Self {
        Self {
            numbers: Rgb::new(128, 128, 128), // DarkGray
            ticks: Rgb::new(128, 128, 128),   // DarkGray
            pair_line: Rgb::new(255, 0, 255), // Magenta
        }
    }
}

/// Status bar mode indicator colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModeColors {
    pub normal_bg: Rgb,
    pub normal_fg: Rgb,
    pub insert_bg: Rgb,
    pub insert_fg: Rgb,
    pub command_bg: Rgb,
    pub command_fg: Rgb,
    pub search_bg: Rgb,
    pub search_fg: Rgb,
    pub visual_bg: Rgb,
    pub visual_fg: Rgb,
}

impl Default for ModeColors {
    fn default() -> Self {
        Self {
            normal_bg: Rgb::new(0, 0, 255),     // Blue
            normal_fg: Rgb::new(255, 255, 255), // White
            insert_bg: Rgb::new(0, 128, 0),     // Green
            insert_fg: Rgb::new(0, 0, 0),       // Black
            command_bg: Rgb::new(255, 255, 0),  // Yellow
            command_fg: Rgb::new(0, 0, 0),      // Black
            search_bg: Rgb::new(255, 0, 255),   // Magenta
            search_fg: Rgb::new(255, 255, 255), // White
            visual_bg: Rgb::new(100, 100, 180), // Purple-ish
            visual_fg: Rgb::new(255, 255, 255), // White
        }
    }
}

/// Status bar colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusBarColors {
    pub background: Rgb,
    pub position: Rgb,
    pub alignment_info: Rgb,
    pub sequence_type: Rgb,
    pub color_scheme: Rgb,
    pub structure_info: Rgb,
    pub selection_info: Rgb,
    #[serde(flatten)]
    pub modes: ModeColors,
}

impl Default for StatusBarColors {
    fn default() -> Self {
        Self {
            background: Rgb::new(128, 128, 128),     // DarkGray
            position: Rgb::new(255, 255, 255),       // White (default)
            alignment_info: Rgb::new(0, 255, 255),   // Cyan
            sequence_type: Rgb::new(0, 128, 0),      // Green
            color_scheme: Rgb::new(255, 0, 255),     // Magenta
            structure_info: Rgb::new(255, 255, 0),   // Yellow
            selection_info: Rgb::new(173, 216, 230), // LightBlue
            modes: ModeColors::default(),
        }
    }
}

/// ID column colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IdColumnColors {
    pub text: Rgb,
    pub selected_bg: Rgb,
    pub selected_fg: Rgb,
}

impl Default for IdColumnColors {
    fn default() -> Self {
        Self {
            text: Rgb::new(0, 255, 255),          // Cyan
            selected_bg: Rgb::new(80, 80, 140),   // Purple-ish
            selected_fg: Rgb::new(255, 255, 255), // White
        }
    }
}

/// Annotation bar colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AnnotationColors {
    pub ss_cons_fg: Rgb,
    pub ss_cons_bg: Rgb,
    pub ss_cons_paired_fg: Rgb,
    pub ss_cons_paired_bg: Rgb,
    pub rf_conserved_fg: Rgb,
    pub rf_conserved_bg: Rgb,
    pub rf_variable_fg: Rgb,
    pub rf_variable_bg: Rgb,
    pub pp_cons_bg: Rgb,
    pub consensus_fg: Rgb,
    pub consensus_bg: Rgb,
    pub conservation_bg: Rgb,
    pub label_ss_cons_fg: Rgb,
    pub label_rf_fg: Rgb,
    pub label_pp_cons_fg: Rgb,
    pub label_consensus_fg: Rgb,
    pub label_conservation_fg: Rgb,
}

impl Default for AnnotationColors {
    fn default() -> Self {
        Self {
            ss_cons_fg: Rgb::new(255, 255, 0), // Yellow
            ss_cons_bg: Rgb::new(30, 30, 40),
            ss_cons_paired_fg: Rgb::new(0, 0, 0),     // Black
            ss_cons_paired_bg: Rgb::new(255, 255, 0), // Yellow
            rf_conserved_fg: Rgb::new(0, 128, 0),     // Green
            rf_conserved_bg: Rgb::new(30, 40, 30),
            rf_variable_fg: Rgb::new(128, 128, 128), // DarkGray
            rf_variable_bg: Rgb::new(30, 30, 30),
            pp_cons_bg: Rgb::new(30, 30, 40),
            consensus_fg: Rgb::new(0, 255, 255), // Cyan
            consensus_bg: Rgb::new(30, 40, 30),
            conservation_bg: Rgb::new(40, 30, 40),
            label_ss_cons_fg: Rgb::new(255, 255, 0), // Yellow
            label_rf_fg: Rgb::new(0, 128, 0),        // Green
            label_pp_cons_fg: Rgb::new(255, 255, 0), // Yellow
            label_consensus_fg: Rgb::new(0, 255, 255), // Cyan
            label_conservation_fg: Rgb::new(255, 0, 255), // Magenta
        }
    }
}

/// Selection and highlight colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SelectionColors {
    pub visual_bg: Rgb,
    pub visual_fg: Rgb,
    pub search_current_bg: Rgb,
    pub search_current_fg: Rgb,
    pub search_other_bg: Rgb,
    pub search_other_fg: Rgb,
    pub pair_highlight_bg: Rgb,
    pub pair_highlight_fg: Rgb,
    pub gap_column_bg: Rgb,
}

impl Default for SelectionColors {
    fn default() -> Self {
        Self {
            visual_bg: Rgb::new(80, 80, 140),
            visual_fg: Rgb::new(255, 255, 255),
            search_current_bg: Rgb::new(255, 255, 0), // Yellow
            search_current_fg: Rgb::new(0, 0, 0),     // Black
            search_other_bg: Rgb::new(100, 100, 50),
            search_other_fg: Rgb::new(255, 255, 255),
            pair_highlight_bg: Rgb::new(255, 0, 255), // Magenta
            pair_highlight_fg: Rgb::new(255, 255, 255),
            gap_column_bg: Rgb::new(80, 50, 50), // Dim red
        }
    }
}

/// Command and search line colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandLineColors {
    pub command_prefix: Rgb,
    pub search_prefix: Rgb,
    pub help_hint: Rgb,
}

impl Default for CommandLineColors {
    fn default() -> Self {
        Self {
            command_prefix: Rgb::new(255, 255, 0), // Yellow
            search_prefix: Rgb::new(255, 0, 255),  // Magenta
            help_hint: Rgb::new(128, 128, 128),    // DarkGray
        }
    }
}

/// Separator and misc UI colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MiscColors {
    pub separator: Rgb,
    pub tree_dark_theme: Rgb,
    pub tree_light_theme: Rgb,
}

impl Default for MiscColors {
    fn default() -> Self {
        Self {
            separator: Rgb::new(128, 128, 128),       // DarkGray
            tree_dark_theme: Rgb::new(255, 255, 255), // White
            tree_light_theme: Rgb::new(0, 0, 0),      // Black
        }
    }
}

/// Complete UI theme containing all color settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub border: BorderColors,
    pub ruler: RulerColors,
    pub status_bar: StatusBarColors,
    pub id_column: IdColumnColors,
    pub annotations: AnnotationColors,
    pub selection: SelectionColors,
    pub command_line: CommandLineColors,
    pub misc: MiscColors,
}
