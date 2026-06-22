use image::GenericImageView;

pub fn read_image<R>(
    reader: image::ImageReader<R>,
    max_width: Option<u32>,
) -> Result<crate::ImageData, image::ImageError>
where
    R: std::io::Seek + std::io::Read + std::io::BufRead,
{
    let img = reader.with_guessed_format()?.decode()?;
    Ok(decode_to_image_data(img, max_width))
}

fn decode_to_image_data(img: image::DynamicImage, max_width: Option<u32>) -> crate::ImageData {
    let (orig_w, orig_h) = img.dimensions();

    let img = match max_width {
        Some(max_w) if max_w > 0 && orig_w > max_w => {
            let ratio = max_w as f64 / orig_w as f64;
            let new_h = ((orig_h as f64) * ratio).round().max(1.0) as u32;
            img.resize(max_w, new_h, image::imageops::FilterType::Lanczos3)
        }
        _ => img,
    }
    .to_rgb8();

    let (width, height) = img.dimensions();

    let mut pixels = Vec::with_capacity(height as usize);
    for y in 0..height {
        for x in 0..width {
            pixels.push(*img.get_pixel(x, y));
        }
    }

    crate::ImageData {
        width,
        height,
        pixels,
    }
}
