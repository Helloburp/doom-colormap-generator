use std::fs;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fmt::Display;
use image::{self, ImageBuffer};


#[derive(Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}


impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r:{}, g:{}, b:{})", self.r, self.g, self.b)
    }
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

pub struct Input {
    file_path: String,
}


pub fn config_from_input(input: Input) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_path)?;
    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}


pub fn input_from_args(args: &Vec<String>) -> Result<Input, &'static str> {
    if args.len() < 2 {
        return Err("Please provide a file.")
    }

    let file_path = args[1].clone();

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


