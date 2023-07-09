use super::{Format, ImageMetadata};

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub struct PNG;

impl Format for PNG {
    fn verify(&self, buffer: [u8; 524288]) -> bool {
        &buffer[0..8] == PNG_SIGNATURE
    }
    fn get_metadata(&self, buffer: [u8; 524288]) -> Result<ImageMetadata, &'static str> {
        Ok(ImageMetadata {
            width: u32::from_be_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]),
            height: u32::from_be_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]),
            format: "png".to_string(),
        })
    }
}
