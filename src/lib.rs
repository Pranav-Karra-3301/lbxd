#![allow(clippy::uninlined_format_args)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::print_literal)]
#![allow(clippy::unused_enumerate_index)]
#![allow(clippy::manual_map)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::needless_borrow)]

pub mod ascii;
pub mod batch_loader;
pub mod cache;
pub mod cli;
pub mod config;
pub mod display;
pub mod export;
pub mod feed;
pub mod letterboxd_client;
pub mod models;
pub mod omdb;
pub mod onboarding;
pub mod profile;
pub mod tmdb;
pub mod tui;
pub mod viu;

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_functionality() {
        // Basic smoke test to ensure core modules can be imported
        // This test just needs to compile successfully
    }

    #[test]
    fn test_config_creation() {
        // Test that config can be created without panicking
        use crate::config::ConfigManager;

        // This test should pass even if config creation fails gracefully
        let result = std::panic::catch_unwind(ConfigManager::new);

        // We just want to ensure no panic occurs
        assert!(result.is_ok(), "Config creation should not panic");
    }

    #[test]
    fn test_display_engine() {
        // Test that display engine can be created
        use crate::display::DisplayEngine;

        let _display = DisplayEngine::new();
        // Just verify we can create it without issues - test passes if no panic
    }
}
