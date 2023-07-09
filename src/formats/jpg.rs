use super::{Format, ImageMetadata};

pub struct JPEG;

const JPEG_SIGNATURE: [u8; 2] = [255, 216];

impl Format for JPEG {
    fn verify(&self, buffer: [u8; 524288]) -> bool {
        &buffer[0..2] == JPEG_SIGNATURE
    }
    fn get_metadata(&self, buffer: [u8; 524288]) -> Result<ImageMetadata, &'static str> {
        let mut cur = 4;
        while cur < buffer.len() {
            if buffer[cur] == 0xFF {
                let marker = buffer[cur + 1];
                let block_length = u32::from_be_bytes([0, 0, buffer[cur + 2], buffer[cur + 3]]);
                if marker == 0xC0 || marker == 0xC2 || marker == 0xC3 {
                    return Ok(ImageMetadata {
                        width: u32::from_be_bytes([0, 0, buffer[cur + 7], buffer[cur + 8]]),
                        height: u32::from_be_bytes([0, 0, buffer[cur + 5], buffer[cur + 6]]),
                        format: "jpg".to_string(),
                    });
                } else {
                    cur += block_length as usize;
                }
            } else {
                cur += 1;
            }
        }

        Err("Could not find dimensions")
    }
}
