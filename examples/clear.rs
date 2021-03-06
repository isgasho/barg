use barg::{Image, Size};

use png::HasParameters;
use std::fs::File;
use std::io;

/// Write the raster to a PNG (portable network graphics) file.
pub fn write_png(
    width: u32,
    height: u32,
    pixels: &[u8],
    filename: &str,
) -> io::Result<()> {
    let fl = File::create(filename)?;
    let ref mut bw = io::BufWriter::new(fl);
    let mut enc = png::Encoder::new(bw, width, height);
    enc.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = enc.write_header()?;
    writer.write_image_data(&pixels[..])?;
    Ok(())
}

fn main() {
    // Initialize variables need to write to PNG
    let w = 256;
    let h = 256;
    let mut buffer = vec![0; w * h * 4];
    let mut surface = Image::new(Size(w as u16, h as u16));

    surface.clear(&mut buffer);

    // Save the image to a PNG file.
    write_png(w as u32, h as u32, buffer.as_slice(), "image_example.png")
        .unwrap();
}
