# Configuration

aform-rs can be customized via a TOML configuration file. The application looks for configuration in the following locations (in order):

1. `./aform.toml` (current directory)
2. `~/.config/aform/aform.toml` (XDG config directory)

## Color Formats

Colors can be specified in three formats (you can mix them freely):

| Format | Example | Description |
|--------|---------|-------------|
| RGB | `"255,128,0"` | Comma-separated RGB values |
| Hex | `"#FF8000"` | Standard hex color (case-insensitive) |
| Verbose | `{ r = 255, g = 128, b = 0 }` | Explicit field notation |

## Partial Configuration

You only need to specify the settings you want to change. Any omitted settings will use the default values:

```toml
[theme.border]
active = "255,165,0"  # Just change the active border to orange
```

## Default Configuration

Below is the complete default configuration:

```toml
# aform.toml - aform-rs configuration file

# Border Colors
[theme.border]
active = "0,255,255"      # Cyan
inactive = "128,128,128"  # Gray

# Ruler Colors
[theme.ruler]
numbers = "128,128,128"   # Gray
ticks = "128,128,128"     # Gray
pair_line = "255,0,255"   # Magenta - base pair connection

# Status Bar Colors
[theme.status_bar]
background = "128,128,128"
position = "255,255,255"
alignment_info = "0,255,255"
sequence_type = "0,128,0"
color_scheme = "255,0,255"
structure_info = "255,255,0"
selection_info = "173,216,230"

# Mode indicators
normal_bg = "0,0,255"
normal_fg = "255,255,255"
insert_bg = "0,128,0"
insert_fg = "0,0,0"
command_bg = "255,255,0"
command_fg = "0,0,0"
search_bg = "255,0,255"
search_fg = "255,255,255"
visual_bg = "100,100,180"
visual_fg = "255,255,255"

# ID Column Colors
[theme.id_column]
text = "0,255,255"
selected_bg = "80,80,140"
selected_fg = "255,255,255"

# Annotation Bar Colors
[theme.annotations]
ss_cons_fg = "255,255,0"
ss_cons_bg = "30,30,40"
ss_cons_paired_fg = "0,0,0"
ss_cons_paired_bg = "255,255,0"
rf_conserved_fg = "0,128,0"
rf_conserved_bg = "30,40,30"
rf_variable_fg = "128,128,128"
rf_variable_bg = "30,30,30"
pp_cons_bg = "30,30,40"
consensus_fg = "0,255,255"
consensus_bg = "30,40,30"
conservation_bg = "40,30,40"
label_ss_cons_fg = "255,255,0"
label_rf_fg = "0,128,0"
label_pp_cons_fg = "255,255,0"
label_consensus_fg = "0,255,255"
label_conservation_fg = "255,0,255"

# Selection and Highlight Colors
[theme.selection]
visual_bg = "80,80,140"
visual_fg = "255,255,255"
search_current_bg = "255,255,0"
search_current_fg = "0,0,0"
search_other_bg = "100,100,50"
search_other_fg = "255,255,255"
pair_highlight_bg = "255,0,255"
pair_highlight_fg = "255,255,255"
gap_column_bg = "80,50,50"

# Command Line Colors
[theme.command_line]
command_prefix = "255,255,0"
search_prefix = "255,0,255"
help_hint = "128,128,128"

# Miscellaneous Colors
[theme.misc]
separator = "128,128,128"
tree_dark_theme = "255,255,255"
tree_light_theme = "0,0,0"
```

## Example: High Contrast Theme

```toml
[theme.border]
active = "0,255,0"
inactive = "100,100,100"

[theme.selection]
visual_bg = "0,100,200"
search_current_bg = "255,200,0"
pair_highlight_bg = "255,100,100"
```

## Example: Solarized-Inspired Theme

```toml
[theme.border]
active = "38,139,210"    # Solarized blue
inactive = "88,110,117"  # Solarized base01

[theme.status_bar]
background = "7,54,66"
normal_bg = "38,139,210"
insert_bg = "133,153,0"
command_bg = "181,137,0"
search_bg = "211,54,130"
visual_bg = "108,113,196"
```

## Example: Using Hex Format

You can also use hex colors if preferred:

```toml
[theme.border]
active = "#00FFFF"
inactive = "#808080"
```
