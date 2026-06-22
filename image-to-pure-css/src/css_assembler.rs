use crate::ImageData;
use crate::color_utils::rgb_to_compressed_color;
use crate::gradient_builder::{
    Gradient, GradientLinearData, build_row_gradient, build_row_gradient_tolerancezero,
};
use fxhash::FxHashMap;

fn find_dominant_color(image_data: &ImageData) -> image::Rgb<u8> {
    let mut color_count = FxHashMap::with_capacity_and_hasher(
        (image_data.width as usize * image_data.height as usize).min(65536),
        Default::default(),
    );

    for chunk in image_data.pixels.chunks(8)
    /* sampling only some */
    {
        let image::Rgb([r, g, b]) = chunk[0];
        let key = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        *color_count.entry(key).or_insert(0_u32) += 1;
    }

    let mut max_count = 0;
    let mut dominant = 0;
    for (color, count) in color_count {
        if count > max_count {
            max_count = count;
            dominant = color;
        }
    }
    let r = ((dominant >> 16) & 0xFF) as u8;
    let g = ((dominant >> 8) & 0xFF) as u8;
    let b = (dominant & 0xFF) as u8;

    image::Rgb([r, g, b])
}

pub fn assemble_css<W>(mut w: W, image_data: &ImageData, tolerance: f32) -> std::io::Result<()>
where
    W: std::io::Write,
{
    let width = image_data.width;
    let height = image_data.height;
    let bg_color = find_dominant_color(image_data);

    write!(
        w,
        "<div style=\"width:{width}px;height:{height}px;background-color:",
    )?;
    let (l, b) = rgb_to_compressed_color(bg_color);
    w.write_all(&b[..l])?;
    w.write_all(b";background-image:")?;
    let mut first = true;
    let mut buf = itoa::Buffer::new();
    let mut positions_vec = Vec::with_capacity(height as usize * 8);
    let mut v_solid = Vec::with_capacity(2 + 16 + 7 + 1 + 7 + 1);
    let mut v_smooth = Vec::with_capacity(1 + 7 + 1 + 4 + 2);
    let mut build_stops = Vec::with_capacity(image_data.width as usize - 1);
    let mut build_runs = Vec::with_capacity(image_data.width as usize / 2);
    let mut build_parts = Vec::with_capacity(image_data.width as usize - 1);

    for (pos, row) in image_data
        .pixels
        .chunks_exact(image_data.width as usize)
        .enumerate()
    {
        match if tolerance == 0.0 {
            build_row_gradient_tolerancezero(&mut build_runs, &mut build_parts, row)
        } else {
            build_row_gradient(&mut build_stops, &mut build_parts, row, tolerance)
        } {
            Gradient::Solid(solid_color) => {
                if solid_color == bg_color {
                    continue;
                }
                v_solid.clear();
                if first {
                    first = false
                } else {
                    positions_vec.push(b',');
                    v_solid.push(b',');
                }
                v_solid.extend_from_slice(b"linear-gradient(");
                let (l, b) = rgb_to_compressed_color(solid_color);
                v_solid.extend_from_slice(&b[..l]);
                v_solid.push(b',');
                v_solid.extend_from_slice(&b[..l]);
                v_solid.push(b')');
                w.write_all(&v_solid)?;
            }
            Gradient::Smooth => {
                if first {
                    first = false
                } else {
                    positions_vec.push(b',');
                    w.write_all(b",")?;
                }
                w.write_all(b"linear-gradient(90deg")?;
                for (color, data) in build_parts.iter() {
                    v_smooth.clear();
                    v_smooth.push(b',');
                    let (l, b) = rgb_to_compressed_color(*color);
                    v_smooth.extend_from_slice(&b[..l]);
                    match data {
                        GradientLinearData::A(pos) => {
                            v_smooth.push(b' ');
                            v_smooth.extend_from_slice(buf.format(*pos).as_bytes());
                            v_smooth.extend_from_slice(b"px");
                        }
                        GradientLinearData::B(pos) => {
                            v_smooth.extend_from_slice(b" 0 ");
                            v_smooth.extend_from_slice(buf.format(*pos).as_bytes());
                            v_smooth.extend_from_slice(b"px");
                        }
                        GradientLinearData::C => {}
                        GradientLinearData::D => {
                            v_smooth.extend_from_slice(b" 0");
                        }
                    }
                    w.write_all(&v_smooth)?;
                }
                w.write_all(b")")?;
            }
        }
        positions_vec.extend_from_slice(b"0 ");
        if pos == 0 {
            positions_vec.push(b'0');
        } else {
            positions_vec.extend_from_slice(buf.format(pos).as_bytes());
            positions_vec.extend_from_slice(b"px");
        }
    }
    w.write_all(b";background-size:100% 1px;background-position:")?;
    w.write_all(&positions_vec)?;
    w.write_all(b";background-repeat:no-repeat;\"></div>")?;
    Ok(())
}
