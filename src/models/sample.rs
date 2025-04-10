use std::path::PathBuf;
use std::time::Duration;

/// Represents an audio sample in the organizer
#[derive(Debug, Clone)]
pub struct Sample {
    /// Unique identifier for the sample
    pub id: String,
    /// Sample name (usually derived from filename)
    pub name: String,
    /// File path on disk
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Sample duration
    pub duration: Duration,
    /// Audio format (wav, mp3, etc)
    pub format: String,
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: Option<u16>,
    /// Detected BPM if available
    pub bpm: Option<f32>,
    /// Musical key if available
    pub key: Option<String>,
    /// User-defined tags
    pub tags: Vec<String>,
    /// User rating (0-5)
    pub rating: Option<u8>,
    /// Notes added by the user
    pub notes: Option<String>,
    /// Date added to the library
    pub date_added: chrono::DateTime<chrono::Utc>,
    /// Date last modified
    pub date_modified: chrono::DateTime<chrono::Utc>,
}

impl Sample {
    /// Create a new Sample from a file path
    pub fn from_path(path: PathBuf) -> Result<Self, std::io::Error> {
        // This would normally analyze the file and extract metadata
        // For now, we'll create a minimal sample with default values
        
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
            
        let format = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_lowercase();
            
        let now = chrono::Utc::now();
        
        Ok(Sample {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            size: 0, // Would be populated by checking the actual file
            duration: Duration::from_secs(0),
            path,
            format,
            sample_rate: 44100,
            bit_depth: Some(16),
            bpm: None,
            key: None,
            tags: Vec::new(),
            rating: None,
            notes: None,
            date_added: now,
            date_modified: now,
        })
    }
    
    /// Add a tag to the sample
    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
            self.date_modified = chrono::Utc::now();
        }
    }
    
    /// Set the rating for this sample
    pub fn set_rating(&mut self, rating: u8) {
        let clamped = rating.min(5); // Ensure rating is 0-5
        self.rating = Some(clamped);
        self.date_modified = chrono::Utc::now();
    }
}