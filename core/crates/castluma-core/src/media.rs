#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaSource {
    LocalFile { resource_id: String },
    DirectUrl { url: String },
}

impl MediaSource {
    pub fn is_local(&self) -> bool {
        match self {
            Self::LocalFile { .. } => true,
            Self::DirectUrl { .. } => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Media {
    pub title: String,             // Title to Display
    pub source: MediaSource,       // Reference to the content
    pub mime_type: Option<String>, // MIME type, if known
    pub size_bytes: Option<u64>,   // Size in bytes, if known
}
