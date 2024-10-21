use std::fs;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fmt::Display;
use image::{self, ImageBuffer};
use palette::{color_difference::EuclideanDistance, FromColor, IntoColor, Lab, Srgb, Hsv};

#[derive(Deserialize)]
pub struct Color {
    r: u8, g: u8, b: u8
}


#[derive(Deserialize)]
pub struct Config {
    distance_fade: Color,
    keep_details: bool,
    invulnerability_range_high: Color,
    invulnerability_range_low: Color,
    radiation_suit: Color,
    item_pickup: Color,
}


struct ColorIterator<'a> {
    bytes: &'a [u8],
    offset: usize
}


impl<'a> Iterator for ColorIterator<'a> {
    type Item = Srgb;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.bytes.len() - 2 {
            let color = Srgb::new(
                self.bytes[self.offset] as f32,
                self.bytes[self.offset + 1] as f32,
                self.bytes[self.offset + 2] as f32,
            );

            self.offset += 3;

            Some(color)
        } else {
            None
        }
    }
}


impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r:{}, g:{}, b:{})", self.r, self.g, self.b)
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
    println!("Distance fade: {}", &config.distance_fade);
    let palette_bytes = get_playpal_buffer()?;
    let colormap_bytes = fs::read("src/assets/COLORMAP.cmp")?;
    new_palette_image(&palette_bytes)?;
    new_colormap_image(&palette_bytes, &colormap_bytes, 0)?;
    Ok(())
}

pub fn get_color_distance(c1: Srgb, c2: Srgb) -> f32 {
    let lab1 = Lab::from_color(c1);
    let lab2 = Lab::from_color(c2);

    lab1.distance(lab2)
}


pub fn best_fit_pixel_offset(
    colormap: &Vec<u8>, color: Srgb
) -> usize {
    3 * (ColorIterator {bytes: colormap, offset: 0})
        .map(|cmp_color| get_color_distance(color.clone(), cmp_color.clone()))
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Result cannot be NaN.") )
        .map(|(index, _)| index).expect("Invalid input indeces.")
}



pub fn get_playpal_buffer() -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = fs::read("src/assets/PLAYPAL.pal")?;
    if bytes.len()%3 != 0 {
        return Err("PLAYPAL.pal is not correct filesize.".into())
    }
    if bytes.len() != 14 * 256 * 3 {
        return Err("PLAYPAL.pal is not correct length.".into())
    }

    Ok(bytes)
}


pub fn map_at_distance(
    colormap: &Vec<u8>, fade_color: Srgb, distance: u8, keep_details: bool
) -> Vec<u8> {
    let mix = (distance as f32)/255.0;

    (ColorIterator {bytes: colormap, offset: 0})
        .map(|cmp_color| {
            let real_fade_color = match keep_details {
                false => fade_color.clone(),
                true => {
                    let mut fade_hsv = Hsv::from_color(fade_color);
                    let cmp_hsv = Hsv::from_color(cmp_color);
                    fade_hsv.value = cmp_hsv.value;

                    Hsv::into_color(fade_hsv)
                }
            }
        })
}


pub fn new_palette_image(palette_bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
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

    imgbuf.save("PLAYPAL.png")?;

    Ok(())
}


pub fn new_colormap_image(
    palette_bytes: &Vec<u8>,
    colormap_bytes: &Vec<u8>,
    palette_select: u32
) -> Result<(), Box<dyn Error>> {

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

    imgbuf.save("COLORMAP.png")?;

    Ok(())
}


