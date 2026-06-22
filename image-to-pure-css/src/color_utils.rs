static HEX: &[u8; 16] = b"0123456789abcdef";

pub fn rgb_to_compressed_color(color: image::Rgb<u8>) -> (usize, [u8; 7]) {
    // rgb to common color name replacer has disabled since with zstd, it would mostly be bigger.
    let [r, g, b] = color.0;
    let rh = ((r >> 4) & 0x0F) as usize;
    let rl = (r & 0x0f) as usize;

    let gh = ((g >> 4) & 0x0F) as usize;
    let gl = (g & 0x0f) as usize;

    let bh = ((b >> 4) & 0x0F) as usize;
    let bl = (b & 0x0f) as usize;
    if (rh == rl) && (gh == gl) && (bh == bl) {
        (4, [b'#', HEX[rh], HEX[gh], HEX[bh], 0, 0, 0])
    } else {
        (
            7,
            [b'#', HEX[rh], HEX[rl], HEX[gh], HEX[gl], HEX[bh], HEX[bl]],
        )
    }
}
