use image::{ImageError, io::Reader as ImageReader};
use std::fs::File;
use std::path::{Path};
use std::io::Write;
use webp::Encoder;

pub fn convert_jpeg_to_webp(input_path: &Path, output_path: &Path, quality: f32) -> Result<(), ImageError> {
    let img = ImageReader::open(input_path)?.decode()?;
    let webp = Encoder::from_image(&img).unwrap().encode(quality);

    let output_file_path = if output_path.is_dir() {
        let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
        let output_file_name = format!("{}.webp", file_stem);
        output_path.join(output_file_name)
    } else {
        output_path.with_extension("webp")
    };

    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(&webp)?;
    Ok(())
}
