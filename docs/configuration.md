# Configuration

aform-rs can be customized via a TOML configuration file. The application looks for configuration in the following locations (in order):

1. `./aform.toml` (current directory)
2. `~/.config/aform/aform.toml` (XDG config directory)

## Partial Configuration

You only need to specify the settings you want to change. Any omitted settings will use the default values. For example, to change just the border colors:

```toml
[theme.border]
active = { r = 255, g = 165, b = 0 }  # Orange
```

## Default Configuration

Below is the complete default configuration with all available settings:

```toml
# aform.toml - aform-rs configuration file

# =============================================================================
# Theme Configuration
# =============================================================================
# All colors are specified as RGB values: { r = 0-255, g = 0-255, b = 0-255 }

# -----------------------------------------------------------------------------
# Border Colors
# -----------------------------------------------------------------------------
[theme.border]
active = { r = 0, g = 255, b = 255 }      # Cyan - active pane border
inactive = { r = 128, g = 128, b = 128 }  # Gray - inactive pane border

# -----------------------------------------------------------------------------
# Ruler Colors
# -----------------------------------------------------------------------------
[theme.ruler]
numbers = { r = 128, g = 128, b = 128 }   # Gray - position numbers
ticks = { r = 128, g = 128, b = 128 }     # Gray - tick marks
pair_line = { r = 255, g = 0, b = 255 }   # Magenta - base pair connection line

# -----------------------------------------------------------------------------
# Status Bar Colors
# -----------------------------------------------------------------------------
[theme.status_bar]
background = { r = 128, g = 128, b = 128 }     # Gray
position = { r = 255, g = 255, b = 255 }       # White - cursor position
alignment_info = { r = 0, g = 255, b = 255 }   # Cyan - alignment dimensions
sequence_type = { r = 0, g = 128, b = 0 }      # Green - RNA/DNA/Protein indicator
color_scheme = { r = 255, g = 0, b = 255 }     # Magenta - color scheme name
structure_info = { r = 255, g = 255, b = 0 }   # Yellow - base pair info
selection_info = { r = 173, g = 216, b = 230 } # Light blue - visual selection info

# Mode indicator colors (background and foreground for each mode)
normal_bg = { r = 0, g = 0, b = 255 }          # Blue
normal_fg = { r = 255, g = 255, b = 255 }      # White
insert_bg = { r = 0, g = 128, b = 0 }          # Green
insert_fg = { r = 0, g = 0, b = 0 }            # Black
command_bg = { r = 255, g = 255, b = 0 }       # Yellow
command_fg = { r = 0, g = 0, b = 0 }           # Black
search_bg = { r = 255, g = 0, b = 255 }        # Magenta
search_fg = { r = 255, g = 255, b = 255 }      # White
visual_bg = { r = 100, g = 100, b = 180 }      # Purple
visual_fg = { r = 255, g = 255, b = 255 }      # White

# -----------------------------------------------------------------------------
# ID Column Colors
# -----------------------------------------------------------------------------
[theme.id_column]
text = { r = 0, g = 255, b = 255 }             # Cyan - sequence IDs
selected_bg = { r = 80, g = 80, b = 140 }      # Purple - visual selection
selected_fg = { r = 255, g = 255, b = 255 }    # White

# -----------------------------------------------------------------------------
# Annotation Bar Colors
# -----------------------------------------------------------------------------
[theme.annotations]
# SS_cons (secondary structure consensus)
ss_cons_fg = { r = 255, g = 255, b = 0 }       # Yellow
ss_cons_bg = { r = 30, g = 30, b = 40 }        # Dark blue-gray
ss_cons_paired_fg = { r = 0, g = 0, b = 0 }    # Black - paired bracket
ss_cons_paired_bg = { r = 255, g = 255, b = 0 }# Yellow - paired bracket

# RF (reference sequence)
rf_conserved_fg = { r = 0, g = 128, b = 0 }    # Green - conserved positions
rf_conserved_bg = { r = 30, g = 40, b = 30 }   # Dark green-gray
rf_variable_fg = { r = 128, g = 128, b = 128 } # Gray - variable positions
rf_variable_bg = { r = 30, g = 30, b = 30 }    # Dark gray

# PP_cons (posterior probability consensus)
pp_cons_bg = { r = 30, g = 30, b = 40 }        # Dark blue-gray

# Consensus sequence
consensus_fg = { r = 0, g = 255, b = 255 }     # Cyan
consensus_bg = { r = 30, g = 40, b = 30 }      # Dark green-gray

# Conservation bar
conservation_bg = { r = 40, g = 30, b = 40 }   # Dark purple-gray

# Annotation labels (left column)
label_ss_cons_fg = { r = 255, g = 255, b = 0 }     # Yellow
label_rf_fg = { r = 0, g = 128, b = 0 }            # Green
label_pp_cons_fg = { r = 255, g = 255, b = 0 }     # Yellow
label_consensus_fg = { r = 0, g = 255, b = 255 }   # Cyan
label_conservation_fg = { r = 255, g = 0, b = 255 }# Magenta

# -----------------------------------------------------------------------------
# Selection and Highlight Colors
# -----------------------------------------------------------------------------
[theme.selection]
visual_bg = { r = 80, g = 80, b = 140 }        # Purple - visual selection
visual_fg = { r = 255, g = 255, b = 255 }      # White

search_current_bg = { r = 255, g = 255, b = 0 }# Yellow - current match
search_current_fg = { r = 0, g = 0, b = 0 }    # Black
search_other_bg = { r = 100, g = 100, b = 50 } # Olive - other matches
search_other_fg = { r = 255, g = 255, b = 255 }# White

pair_highlight_bg = { r = 255, g = 0, b = 255 }# Magenta - paired base
pair_highlight_fg = { r = 255, g = 255, b = 255 }# White

gap_column_bg = { r = 80, g = 50, b = 50 }     # Dim red - gap columns

# -----------------------------------------------------------------------------
# Command Line Colors
# -----------------------------------------------------------------------------
[theme.command_line]
command_prefix = { r = 255, g = 255, b = 0 }   # Yellow - ":" prefix
search_prefix = { r = 255, g = 0, b = 255 }    # Magenta - "/" prefix
help_hint = { r = 128, g = 128, b = 128 }      # Gray - help text

# -----------------------------------------------------------------------------
# Miscellaneous Colors
# -----------------------------------------------------------------------------
[theme.misc]
separator = { r = 128, g = 128, b = 128 }      # Gray - vertical separators
tree_dark_theme = { r = 255, g = 255, b = 255 }# White - tree on dark background
tree_light_theme = { r = 0, g = 0, b = 0 }     # Black - tree on light background
```

## Example: High Contrast Theme

Here's an example of a high-contrast configuration for better visibility:

```toml
[theme.border]
active = { r = 0, g = 255, b = 0 }    # Bright green
inactive = { r = 100, g = 100, b = 100 }

[theme.selection]
visual_bg = { r = 0, g = 100, b = 200 }
search_current_bg = { r = 255, g = 200, b = 0 }
pair_highlight_bg = { r = 255, g = 100, b = 100 }
```

## Example: Solarized-Inspired Theme

```toml
[theme.border]
active = { r = 38, g = 139, b = 210 }   # Solarized blue
inactive = { r = 88, g = 110, b = 117 } # Solarized base01

[theme.status_bar]
background = { r = 7, g = 54, b = 66 }  # Solarized base02
normal_bg = { r = 38, g = 139, b = 210 }
insert_bg = { r = 133, g = 153, b = 0 }
command_bg = { r = 181, g = 137, b = 0 }
search_bg = { r = 211, g = 54, b = 130 }
visual_bg = { r = 108, g = 113, b = 196 }
```
