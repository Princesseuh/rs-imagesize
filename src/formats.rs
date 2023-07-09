pub mod gif;
pub mod jpg;
pub mod png;

pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
}

pub(crate) trait Format {
    fn verify(&self, buffer: [u8; 524288]) -> bool;
    fn get_metadata(&self, buffer: [u8; 524288]) -> Result<ImageMetadata, &'static str>;
}
