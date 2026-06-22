use image_to_pure_css::{ImageData, Rgb, assemble_css};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn image_to_pure_css(
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    tolerance: f32,
) -> Result<String, JsValue> {
    if !rgba.len().is_multiple_of(4) {
        return Err(JsValue::from_str("rgba length is not multiple of four"));
    }
    let mut pixels = Vec::with_capacity(rgba.len() / 4);
    for px in rgba.chunks_exact(4) {
        pixels.push(Rgb([px[0], px[1], px[2]]));
    }
    let mut data = vec![];
    let w = std::io::Cursor::new(&mut data);
    if let Err(e) = assemble_css(
        w,
        &ImageData {
            width,
            height,
            pixels,
        },
        tolerance,
    ) {
        return Err(JsValue::from_str(&format!(
            "cannot assemble css due to error: {e}",
        )));
    }
    String::from_utf8(data).map_err(|_| JsValue::from_str("invalid string"))
}
