use super::{Format, ImageMetadata};

pub struct GIF;

const GIF89A_SIGNATURE: [u8; 6] = [71, 73, 70, 56, 57, 97];
const GIF87A_SIGNATURE: [u8; 6] = [71, 73, 70, 56, 55, 97];

impl Format for GIF {
    fn verify(&self, buffer: [u8; 524288]) -> bool {
        &buffer[0..6] == GIF87A_SIGNATURE || &buffer[0..6] == GIF89A_SIGNATURE
    }
    fn get_metadata(&self, buffer: [u8; 524288]) -> Result<ImageMetadata, &'static str> {
        Ok(ImageMetadata {
            width: u32::from_le_bytes([buffer[6], buffer[7], 0, 0]),
            height: u32::from_le_bytes([buffer[8], buffer[9], 0, 0]),
            format: "gif".to_string(),
        })
    }
}
