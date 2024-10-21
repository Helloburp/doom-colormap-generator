use std::fs; use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fmt::Display;
use image::{self, ImageBuffer};
use palette::{
    FromColor, Hsv,
    IntoColor, Lab, Mix,
    color_difference::EuclideanDistance,
};

#[derive(Deserialize)]
pub struct MaskConfig {
    keep_hue: bool,
    keep_saturation: bool,
    keep_value: bool,
}


#[derive(Deserialize)]
pub struct Config {
    distance_fade: palette::Srgb<u8>,
    masking: MaskConfig,
    // invulnerability_range_high: palette::Srgb<u8>,
    // invulnerability_range_low: palette::Srgb<u8>,
    // radiation_suit: palette::Srgb<u8>,
    // item_pickup: palette::Srgb<u8>,
}


struct ColorIterator<'a> { bytes: &'a [u8], offset: usize }
impl<'a> Iterator for ColorIterator<'a> {
    type Item = palette::Srgb;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.bytes.len() - 2 {
            let color = palette::Srgb::new(
                (self.bytes[self.offset] as f32)/255.0,
                (self.bytes[self.offset + 1] as f32)/255.0,
                (self.bytes[self.offset + 2] as f32)/255.0,
            );

            self.offset += 3;

            Some(color)
        } else {
            None
        }
    }
}


pub struct Input {
    file_path: String,
}


pub fn config_from_input(input: Input) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_path)?;
    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}


pub fn build_input(
    mut args: impl Iterator<Item = String>
) -> Result<Input, &'static str> {
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Must specify file path.")
    };

    Ok(
        Input { file_path }
    )
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Distance fade: {}",
        serde_json::to_string(&config.distance_fade)?
    );
    let playpal_bytes = fs::read("src/assets/PLAYPAL.pal")?;
    let colormap_bytes = fs::read("src/assets/COLORMAP.cmp")?;

    let playpal_image = draw_playpal(&playpal_bytes)?;
    let color_image = draw_colormap(&playpal_bytes, &colormap_bytes, 0)?;

    playpal_image.save("PLAYPAL.png")?;
    color_image.save("COLORMAP.png")?;

    let dark_colormap_image = draw_colormap(
        &playpal_bytes, &build_colormap(
            &playpal_bytes,
            &get_invulnerability_page_from_colormap(&colormap_bytes),
            &config
        ), 0
    )?;

    dark_colormap_image.save("DARK_COLORMAP.png")?;

    Ok(())
}


pub fn get_invulnerability_page_from_colormap<'a>(
    colormap: &'a [u8]
) -> &'a [u8] {
    &colormap[32*256..33*256]
}


pub fn build_colormap(
    playpal: &[u8], invulnerability_colormap_page: &[u8], config: &Config
) -> Vec<u8> {
    let mut colormap = vec![];
    let playpal_first_page = &playpal[0..256*3];

    for n in 0..=31 {
        let darkness = n * 8 as u8;
        let new_bytes = new_colormap_bytes_at_distance(
            playpal_first_page, config.distance_fade.into_format(),
            darkness, &config.masking
        );
        colormap.extend(&new_bytes);
    }

    colormap.extend(invulnerability_colormap_page);
    colormap.extend(&[0u8; 256]);


    colormap
}


pub fn best_fit_pixel_offset(
    colormap: &[u8], color: palette::Srgb
) -> usize {
    (ColorIterator {bytes: colormap, offset: 0})
        .map(|cmp_color| {
            Lab::from_color(color).distance(Lab::from_color(cmp_color))
        })
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.partial_cmp(b).expect("Result cannot be NaN.")
        })
        .map(|(index, _)| index).expect("Invalid input indeces.")
}



pub fn new_colormap_bytes_at_distance(
    playpal_first_page: &[u8],
    fade_color: palette::Srgb,
    distance: u8,
    mask_config: &MaskConfig
) -> Vec<u8> {
    (ColorIterator {bytes: playpal_first_page, offset: 0})
        .map(|cmp_color| {
            let cmp_hsv = Hsv::from_color(cmp_color);
            let mut fade_hsv = Hsv::from_color(fade_color);
            fade_hsv.saturation =
                (if mask_config.keep_saturation { cmp_hsv } else { fade_hsv }).saturation;
            fade_hsv.hue =
                (if mask_config.keep_hue { cmp_hsv } else { fade_hsv }).hue;
            fade_hsv.value =
                (if mask_config.keep_value { cmp_hsv } else { fade_hsv }).value;

            cmp_hsv.mix(fade_hsv, (distance as f32)/255.0).into_color()
        })
        .map(|mixed_color| {
            best_fit_pixel_offset(playpal_first_page, mixed_color) as u8
        })
        .collect()
}


pub fn draw_playpal(
    palette_bytes: &[u8]
) -> Result<ImageBuffer<image::Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
    let (imgx, imgy) = (16, (palette_bytes.len()/(16*3)) as u32);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let offset = (x*3 + y*16*3) as usize;
        let (r, g, b) = (
            palette_bytes[offset],
            palette_bytes[offset + 1],
            palette_bytes[offset + 2],
        );
        *pixel = image::Rgb([r,g,b]);
    }

    Ok(imgbuf)
}


pub fn draw_colormap(
    palette_bytes: &[u8],
    colormap_bytes: &[u8],
    palette_select: u32
) -> Result<ImageBuffer<image::Rgb<u8>, Vec<u8>>, Box<dyn Error>> {

    let (imgx, imgy) = (16, (colormap_bytes.len()/16) as u32);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let colormap_index = colormap_bytes[(y*16 + x) as usize];
        let offset = (
            palette_select*256*3 +
            (colormap_index as u32) * 3
        ) as usize;

        let (r, g, b) = (
            palette_bytes[offset],
            palette_bytes[offset + 1],
            palette_bytes[offset + 2],
        );
        *pixel = image::Rgb([r,g,b]);
    }

    Ok(imgbuf)
}


