use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::path::PathBuf;

/// Represents a complete .torrent file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentMetaInfo {
    // Required fields
    pub announce: String,  // Tracker URL
    pub info: TorrentInfo, // Core torrent information

    // Optional fields with attributes
    #[serde(default)] // Use None if field is missing
    #[serde(skip_serializing_if = "Option::is_none")] // Don't serialize if None
    pub announce_list: Option<Vec<Vec<String>>>, // List of backup trackers

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub piece_length: i64, // Size of each piece in bytes
    pub pieces: Vec<u8>,   // Concatenated SHA1 hashes of each piece
    pub name: String,      // Suggested name for saving file/directory

    // These fields are mutually exclusive:
    // - length for single file mode
    // - files for multiple file mode
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>, // Single file mode

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileInfo>>, // Multiple file mode

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<i64>, // Whether to use DHT/PEX/LPD
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub length: i64,
    pub path: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
}
