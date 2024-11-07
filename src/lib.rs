use clap::Parser;
use palette::Srgb;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::{fs, path::PathBuf};

mod dcolors;
mod draw;
mod assets {
    pub static VANILLA_PLAYPAL: &'static [u8] = include_bytes!("assets/PLAYPAL.pal");
    pub static VANILLA_COLORMAP: &'static [u8] = include_bytes!("assets/COLORMAP.cmp");
}
mod constants {
    pub static PLAYPAL_LEN: usize = 14 * 256 * 3;
    pub static COLORMAP_LEN: usize = 34 * 256;
}

pub struct MySrgb<T>(Srgb<T>);
impl Into<(i32, i32, i32)> for MySrgb<i32> {
    fn into(self) -> (i32, i32, i32) {
        (self.0.red as i32, self.0.green as i32, self.0.blue as i32)
    }
}

impl From<(i32, i32, i32)> for MySrgb<f32> {
    fn from(value: (i32, i32, i32)) -> Self {
        MySrgb(Srgb::new(
            value.0 as f32 / 255.0,
            value.1 as f32 / 255.0,
            value.2 as f32 / 255.0,
        ))
    }
}

#[derive(Parser, Debug)]
pub struct Input {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Deserialize)]
pub struct UserConfig {
    distance_fade: Srgb<i32>,
    distance_fade_blend_mode: BlendMode,

    invulnerability_range_low: palette::Srgb<i32>,
    invulnerability_range_high: palette::Srgb<i32>,

    hurt: Srgb<i32>,
    hurt_blend_mode: BlendMode,

    radiation_suit: Srgb<i32>,
    radiation_suit_blend_mode: BlendMode,

    item_pickup: Srgb<i32>,
    item_pickup_blend_mode: BlendMode,
}

pub fn config_from_input(input: &Input) -> Result<UserConfig, Box<dyn Error>> {
    let contents = fs::read_to_string(&input.input)?;
    let config: UserConfig = serde_json::from_str(&contents)?;

    Ok(config)
}

pub fn run(input: Input, config: UserConfig) -> Result<(), Box<dyn Error>> {
    let mut new_colormap_bytes = vec![0; constants::COLORMAP_LEN];
    let mut new_playpal_bytes = vec![0; constants::PLAYPAL_LEN];

    dcolors::build_colormap(
        assets::VANILLA_PLAYPAL,
        &mut new_colormap_bytes,
        MySrgb(config.distance_fade).into(),
        MySrgb(config.invulnerability_range_low).into(),
        MySrgb(config.invulnerability_range_high).into(),
        config.distance_fade_blend_mode,
    );

    dcolors::build_palette(
        assets::VANILLA_PLAYPAL,
        &mut new_playpal_bytes,
        MySrgb(config.hurt).into(),
        MySrgb(config.item_pickup).into(),
        MySrgb(config.radiation_suit).into(),
        config.hurt_blend_mode,
        config.radiation_suit_blend_mode,
        config.item_pickup_blend_mode,
    );

    let new_playpal_image = draw::draw_playpal(&new_playpal_bytes);
    let new_colormap_image = draw::draw_colormap(assets::VANILLA_PLAYPAL, &new_colormap_bytes, 0);

    if !fs::exists(&input.output).unwrap_or_default() {
        fs::create_dir(&input.output)?;
    }

    fs::write(
        {
            let mut path = input.output.clone();
            path.push("PLAYPAL");
            path.set_extension("pal");
            path
        },
        new_playpal_bytes,
    )?;

    fs::write(
        {
            let mut path = input.output.clone();
            path.push("COLORMAP");
            path.set_extension("cmp");
            path
        },
        new_colormap_bytes,
    )?;

    new_playpal_image.save({
        let mut path = input.output.clone();
        path.push("PLAYPAL_preview");
        path.set_extension("png");
        path
    })?;

    new_colormap_image.save({
        let mut path = input.output.clone();
        path.push("COLORMAP_preview");
        path.set_extension("png");
        path
    })?;

    Ok(())
}
