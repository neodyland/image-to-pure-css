mod color_utils;
mod css_assembler;
mod gradient_builder;
pub use css_assembler::assemble_css;
pub use image::Rgb;

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<image::Rgb<u8>>,
}

#[cfg(feature = "std")]
mod image_reader;
#[cfg(feature = "std")]
mod std {
    use super::assemble_css;
    pub use super::image_reader::read_image;
    pub use image::ImageReader;

    pub struct ConvertImageOptions {
        pub width: Option<u32>,
        pub tolerance: u8,
    }

    pub fn convert_image_to_css<W>(
        input: &str,
        options: ConvertImageOptions,
        w: W,
    ) -> Result<(), image::ImageError>
    where
        W: std::io::Write,
    {
        let tolerance = options.tolerance;
        let image_data = read_image(image::ImageReader::open(input)?, options.width)?;
        assemble_css(w, &image_data, tolerance as f32)?;
        Ok(())
    }
}
#[cfg(feature = "std")]
pub use std::*;
