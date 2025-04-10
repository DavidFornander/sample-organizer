// This file contains the definitions of data structures and models used in the application. 
// It may export structs and enums that represent the application's core data. 

// Collection of data models for the sample organizer

pub mod sample;

pub struct AudioFile {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: u32, // duration in seconds
}

pub enum FileType {
    Mp3,
    Wav,
    Flac,
    Other(String),
}