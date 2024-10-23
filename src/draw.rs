use image::{self, ImageBuffer, Rgb};

pub fn draw_playpal(palette_bytes: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (imgx, imgy) = (16, (palette_bytes.len() / (16 * 3)) as u32);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let offset = (x * 3 + y * 16 * 3) as usize;
        let (r, g, b) = (
            palette_bytes[offset],
            palette_bytes[offset + 1],
            palette_bytes[offset + 2],
        );
        *pixel = Rgb([r, g, b]);
    }

    imgbuf
}

pub fn draw_colormap(
    palette_bytes: &[u8],
    colormap_bytes: &[u8],
    palette_select: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (imgx, imgy) = (16, (colormap_bytes.len() / 16) as u32);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let colormap_index = colormap_bytes[(y * 16 + x) as usize];
        let offset = (palette_select * 256 * 3 + (colormap_index as u32) * 3) as usize;

        let (r, g, b) = (
            palette_bytes[offset],
            palette_bytes[offset + 1],
            palette_bytes[offset + 2],
        );
        *pixel = Rgb([r, g, b]);
    }

    imgbuf
}
