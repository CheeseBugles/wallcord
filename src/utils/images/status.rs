#[derive(Debug, Default, PartialEq)]
/// Describe the status of the image.
pub enum ImageStatus {
    /// The image has been fetched and is ready for review.
    Done,
    #[default]
    /// The image is currently being fetched and not ready for review.
    Lodding,
    /// The fetch process failed, typically due to unsupported format or a timeout.
    Faild,
}

impl ImageStatus {
    /// The image has been fetched and is ready for review.
    pub fn is_done(&self) -> bool {
        *self == Self::Done
    }
    /// The image is currently being fetched and not ready for review.
    pub fn is_faild(&self) -> bool {
        *self == Self::Faild
    }
    /// The fetch process failed, typically due to unsupported format or a timeout.
    pub fn is_loading(&self) -> bool {
        *self == Self::Lodding
    }
}
