//! Terminal capability detection and validation.
//!
//! This module provides utilities for detecting terminal capabilities
//! such as size validation for the TUI.

/// Minimum terminal width required for the TUI
pub const MIN_TERMINAL_WIDTH: u16 = 80;
/// Minimum terminal height required for the TUI
pub const MIN_TERMINAL_HEIGHT: u16 = 24;

/// Terminal capabilities and current state
#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    /// Current terminal width
    pub width: u16,
    /// Current terminal height
    pub height: u16,
}

impl TerminalCapabilities {
    /// Detect terminal capabilities from environment and terminal state
    pub fn detect() -> Self {
        // Get terminal size, default to 80x24 if detection fails
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));

        Self { width, height }
    }

    /// Check if terminal size meets minimum requirements
    pub fn is_size_sufficient(&self) -> bool {
        self.width >= MIN_TERMINAL_WIDTH && self.height >= MIN_TERMINAL_HEIGHT
    }

    /// Get a user-friendly error message for insufficient terminal size
    pub fn get_size_error_message(&self) -> String {
        format!(
            "Terminal too small for lbxd TUI.\n\n\
             Required: {}x{} (width x height)\n\
             Current:  {}x{}\n\n\
             Please resize your terminal window and try again.\n\
             Tip: You can also use non-TUI commands like:\n\
               lbxd recent <username>    - View recent activity\n\
               lbxd movie \"Movie Name\"  - Search for a movie",
            MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT, self.width, self.height
        )
    }
}

impl Default for TerminalCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_sufficient_when_large_enough() {
        let caps = TerminalCapabilities {
            width: 100,
            height: 40,
        };
        assert!(caps.is_size_sufficient());
    }

    #[test]
    fn test_size_insufficient_when_too_narrow() {
        let caps = TerminalCapabilities {
            width: 60,
            height: 40,
        };
        assert!(!caps.is_size_sufficient());
    }

    #[test]
    fn test_size_insufficient_when_too_short() {
        let caps = TerminalCapabilities {
            width: 100,
            height: 20,
        };
        assert!(!caps.is_size_sufficient());
    }

    #[test]
    fn test_error_message_contains_dimensions() {
        let caps = TerminalCapabilities {
            width: 60,
            height: 20,
        };
        let msg = caps.get_size_error_message();
        assert!(msg.contains("60"));
        assert!(msg.contains("20"));
        assert!(msg.contains("80"));
        assert!(msg.contains("24"));
    }
}
