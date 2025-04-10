// src/main.rs
use sample_organizer::{self, indexer, ui};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the library
    sample_organizer::init()?;
    
    println!("Starting Sample Organizer...");
    
    // Start the indexing process in the background
    let _indexer = indexer::start_background_indexing()?;
    
    // Launch the user interface
    ui::run()?;
    
    println!("Sample Organizer shutdown successfully");
    Ok(())
}