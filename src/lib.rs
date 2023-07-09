use crate::formats::Format;
mod formats;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use formats::gif::GIF;
use formats::jpg::JPEG;
use formats::png::PNG;
use formats::ImageMetadata;

pub fn get_image_metadata(file: File) -> Result<ImageMetadata, &'static str> {
    let mut buf_reader = BufReader::new(file);
    let mut buffer = [0; 524288];
    buf_reader.read(&mut buffer).unwrap();

    match buffer[0] {
        0x89 => match PNG.verify(buffer) {
            true => match PNG.get_metadata(buffer) {
                Ok(metadata) => Ok(metadata),
                Err(e) => Err(e),
            },
            false => Err("Image had first byte of a PNG (0x89) but was not a PNG"),
        },
        0x47 => match GIF.verify(buffer) {
            true => match GIF.get_metadata(buffer) {
                Ok(metadata) => Ok(metadata),
                Err(e) => Err(e),
            },
            false => Err("Image had first byte of a GIF (0x47) but was not a GIF"),
        },
        0xff => match JPEG.verify(buffer) {
            true => match JPEG.get_metadata(buffer) {
                Ok(metadata) => Ok(metadata),
                Err(e) => Err(e),
            },
            false => Err("Image had first byte of a JPEG (0xFF) but was not a JPEG"),
        },
        _ => Err("Unknown image format"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_case {
        ($fname:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname) // assumes Linux ('/')!
        };
    }

    #[test]
    fn test_png() {
        let file = File::open(test_case!("./png_transparency.png")).unwrap();
        let metadata = get_image_metadata(file).unwrap();

        assert_eq!(metadata.width, 800);
        assert_eq!(metadata.height, 600);
        assert_eq!(metadata.format, "png");
    }

    #[test]
    fn test_gif() {
        let file = File::open(test_case!("./rotating_earth.gif")).unwrap();
        let metadata = get_image_metadata(file).unwrap();

        assert_eq!(metadata.width, 400);
        assert_eq!(metadata.height, 400);
        assert_eq!(metadata.format, "gif");
    }

    #[test]
    fn test_jpg() {
        let file = File::open(test_case!("./flower.jpg")).unwrap();
        let metadata = get_image_metadata(file).unwrap();

        assert_eq!(metadata.width, 240);
        assert_eq!(metadata.height, 214);
        assert_eq!(metadata.format, "jpg");
    }
}
