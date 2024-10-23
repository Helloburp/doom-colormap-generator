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
    hurt: palette::Srgb<u8>,
    radiation_suit: palette::Srgb<u8>,
    item_pickup: palette::Srgb<u8>,
}

pub fn config_from_input(input: &Input) -> Result<UserConfig, Box<dyn Error>> {
    let contents = fs::read_to_string(&input.config)?;
    let config: UserConfig = serde_json::from_str(&contents)?;

    Ok(config)
}

fn build_color_from_srgb(srgb: palette::Srgb<u8>) -> BuildColor {
    BuildColor(srgb.red.into(), srgb.green.into(), srgb.blue.into())
}

pub fn run(input: Input, config: UserConfig) -> Result<(), Box<dyn Error>> {
    let mut new_colormap_bytes = vec![0; constants::COLORMAP_LEN];
    let mut new_playpal_bytes = vec![0; constants::PLAYPAL_LEN];

    dcolors::build_colormap(
        assets::VANILLA_PLAYPAL,
        &mut new_colormap_bytes,
        build_color_from_srgb(config.distance_fade),
    );

    dcolors::build_palette(
        assets::VANILLA_PLAYPAL,
        &mut new_playpal_bytes,
        build_color_from_srgb(config.hurt),
        build_color_from_srgb(config.item_pickup),
        build_color_from_srgb(config.radiation_suit),
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
