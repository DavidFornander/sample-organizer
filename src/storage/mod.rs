// Database and storage functionality

pub mod db;

/// Initialize the storage system
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    db::initialize_database()?;
    Ok(())
}