use clap::Parser;
use dcolors::BuildColor;
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

#[derive(Parser, Debug)]
pub struct Input {
    #[arg(short, long)]
    config: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Deserialize)]
pub struct MaskConfig {
    keep_hue: bool,
    keep_saturation: bool,
    keep_value: bool,
}

#[derive(Deserialize)]
pub struct UserConfig {
    distance_fade: palette::Srgb<u8>,
    masking: MaskConfig,
    // invulnerability_range_high: palette::Srgb<u8>,
    // invulnerability_range_low: palette::Srgb<u8>,
    // radiation_suit: palette::Srgb<u8>,
    // item_pickup: palette::Srgb<u8>,
}

pub fn config_from_input(input: &Input) -> Result<UserConfig, Box<dyn Error>> {
    let contents = fs::read_to_string(&input.config)?;
    let config: UserConfig = serde_json::from_str(&contents)?;

    Ok(config)
}

fn new_colormap(playpal_page_0: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0; constants::COLORMAP_LEN];
    dcolors::build_colormap(playpal_page_0, &mut bytes, BuildColor(0, 0, 0));

    bytes
}

fn new_palette(playpal_page_0: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0; constants::PLAYPAL_LEN];
    dcolors::build_vanilla_palette(playpal_page_0, &mut bytes);

    bytes
}

pub fn run(input: Input, config: UserConfig) -> Result<(), Box<dyn Error>> {
    let new_colormap_bytes = new_colormap(assets::VANILLA_PLAYPAL);
    let new_playpal_bytes = new_palette(assets::VANILLA_PLAYPAL);

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
