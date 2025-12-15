//! Generic input history management for command and search buffers.

/// Generic input history that handles navigation through previous entries.
/// Used for both command history (`:` commands) and search history (`/` searches).
#[derive(Debug, Clone, Default)]
pub struct InputHistory {
    /// History entries (oldest first).
    entries: Vec<String>,
    /// Current position in history (None = new entry, not browsing history).
    index: Option<usize>,
    /// Saved input when browsing history (restored when exiting history).
    saved: String,
}

impl InputHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry to history (avoids consecutive duplicates).
    pub fn push(&mut self, entry: String) {
        if self.entries.last() != Some(&entry) {
            self.entries.push(entry);
        }
        self.index = None;
    }

    /// Navigate to previous entry (older). Returns the entry to display.
    /// `current_input` is the current buffer content before navigating.
    pub fn prev(&mut self, current_input: &str) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        match self.index {
            None => {
                // Save current input and go to most recent history
                self.saved = current_input.to_string();
                self.index = Some(self.entries.len() - 1);
            }
            Some(0) => {
                // Already at oldest, stay there
                return self.entries.first().map(String::as_str);
            }
            Some(i) => {
                self.index = Some(i - 1);
            }
        }

        self.index
            .and_then(|i| self.entries.get(i).map(String::as_str))
    }

    /// Navigate to next entry (newer). Returns the entry to display, or saved input if at end.
    pub fn next(&mut self) -> Option<&str> {
        match self.index {
            None => {
                // Not in history, nothing to do
                None
            }
            Some(i) if i >= self.entries.len().saturating_sub(1) => {
                // At end of history, restore saved input
                self.index = None;
                Some(&self.saved)
            }
            Some(i) => {
                self.index = Some(i + 1);
                self.entries.get(i + 1).map(String::as_str)
            }
        }
    }

    /// Reset history navigation state (call when entering the mode).
    pub fn reset_navigation(&mut self) {
        self.index = None;
        self.saved.clear();
    }

    /// Check if currently browsing history.
    #[allow(dead_code)]
    pub fn is_browsing(&self) -> bool {
        self.index.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_history() {
        let mut history = InputHistory::new();
        assert!(history.prev("current").is_none());
        assert!(history.next().is_none());
    }

    #[test]
    fn test_navigation() {
        let mut history = InputHistory::new();
        history.push("first".to_string());
        history.push("second".to_string());
        history.push("third".to_string());

        // Navigate back
        assert_eq!(history.prev("current"), Some("third"));
        assert_eq!(history.prev("current"), Some("second"));
        assert_eq!(history.prev("current"), Some("first"));
        // At oldest, stays there
        assert_eq!(history.prev("current"), Some("first"));

        // Navigate forward
        assert_eq!(history.next(), Some("second"));
        assert_eq!(history.next(), Some("third"));
        // At end, restores saved
        assert_eq!(history.next(), Some("current"));
    }

    #[test]
    fn test_avoids_duplicates() {
        let mut history = InputHistory::new();
        history.push("same".to_string());
        history.push("same".to_string());
        history.push("different".to_string());
        history.push("different".to_string());

        assert_eq!(history.prev(""), Some("different"));
        assert_eq!(history.prev(""), Some("same"));
        assert!(history.prev("").is_some()); // stays at oldest
    }
}
