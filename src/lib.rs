// This file defines the library interface for the project. It may contain public functions and types that can be used by other modules or external crates.

// Module declarations
pub mod audio;
pub mod indexer;
pub mod storage;
pub mod models;
pub mod ui;

// Re-export commonly used types for convenience
pub use models::sample::Sample;

// Library initialization
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize necessary components
    storage::init()?;
    
    Ok(())
}