# VHS tape for main editor screenshot
# Run: vhs docs/tapes/editor.tape
# Pre-req: cargo install --path .

Output docs/images/editor.gif

Set Shell "bash"
Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 16
Set Width 1200
Set Height 800
Set LetterSpacing 0
Set LineHeight 1.2
Set Padding 20
Set Theme "Dracula"

# Open editor with structure coloring
Type "aform --color structure examples/ralee/RNaseP.stk"
Enter
Sleep 2s

# Jump to last sequence, then scroll to show SS_cons
Type "G"
Sleep 500ms
Ctrl+d
Sleep 1s

# Navigate to a paired position (move right into the structure)
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 1s

# Jump to paired base
Type "gp"
Sleep 1.5s

# Jump back to show highlighting
Type "gp"
Sleep 1.5s

# Final pause
Sleep 2s

# Quit
Type ":q"
Enter
