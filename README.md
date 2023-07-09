# rs-imagesize

> **Note**
> Looking for a real library to do this? Checkout [https://github.com/Roughsketch/imagesize](https://github.com/Roughsketch/imagesize)

An incomplete port of [image-size](https://www.npmjs.com/package/image-size) to Rust. Mostly for fun and learning Rust.

## Usage

```rust
use rs_imagesize::get_image_metadata;

fn main() {
		let metadata = get_image_metadata("path/to/image").unwrap();
		println!("Format: {}. Dimensions: {}x{}", metadata.format, metadata.width, metadata.height);
}
```

## Supported formats

- [x] PNG
- [x] JPEG
- [x] GIF
- [ ] BMP
- [ ] TIFF
- [ ] PSD
- [ ] WebP
- [ ] SVG
- [ ] ICO
- [ ] CUR
- [ ] DDS
- [ ] AVIF
