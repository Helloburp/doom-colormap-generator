// Source: https://www.doomworld.com/idgames/historic/dmutils

use crate::BlendMode;

fn color_shift_palette(
    inpal: &[u8],
    outpal_at_palette_to_shift: &mut [u8],
    rgb: (i32, i32, i32),
    shift: i32,
    steps: i32,
    mode: &BlendMode,
) {
    let get_mix_result = |rgb_in: (i32, i32, i32), target: (i32, i32, i32)| -> [u8; 3] {
        let mix = mix_colors(rgb_in, target, shift as f32 / steps as f32);
        [mix.0 as u8, mix.1 as u8, mix.2 as u8]
    };
    for i in 0..256 {
        let offset = i * 3;
        let rgb_in = (
            inpal[0 + offset] as i32,
            inpal[1 + offset] as i32,
            inpal[2 + offset] as i32,
        );

        let out_color_slice = match *mode {
            BlendMode::Normal => [
                (rgb_in.0 + (rgb.0 - rgb_in.0) * shift / steps) as u8,
                (rgb_in.1 + (rgb.1 - rgb_in.1) * shift / steps) as u8,
                (rgb_in.2 + (rgb.2 - rgb_in.2) * shift / steps) as u8,
            ],
            BlendMode::Multiply => {
                let target = (
                    (rgb.0 as f32 / 255.0 * rgb_in.0 as f32 / 255.0) as i32 * 255,
                    (rgb.1 as f32 / 255.0 * rgb_in.1 as f32 / 255.0) as i32 * 255,
                    (rgb.2 as f32 / 255.0 * rgb_in.2 as f32 / 255.0) as i32 * 255,
                );
                get_mix_result(rgb_in, target)
            }
            BlendMode::Screen => {
                let target = (
                    (1.0 - (1.0 - rgb.0 as f32 / 255.0) * (1.0 - rgb_in.0 as f32 / 255.0)) as i32
                        * 255,
                    (1.0 - (1.0 - rgb.1 as f32 / 255.0) * (1.0 - rgb_in.1 as f32 / 255.0)) as i32
                        * 255,
                    (1.0 - (1.0 - rgb.2 as f32 / 255.0) * (1.0 - rgb_in.2 as f32 / 255.0)) as i32
                        * 255,
                );
                let mix = mix_colors(rgb_in, target, shift as f32 / steps as f32);

                [mix.0 as u8, mix.1 as u8, mix.2 as u8]
            }
            BlendMode::Hue => {
                get_mix_result(rgb_in, combine_hsv(rgb_in, rgb, (true, false, false)))
            }
            BlendMode::Saturation => {
                get_mix_result(rgb_in, combine_hsv(rgb_in, rgb, (false, true, false)))
            }
            BlendMode::Color => {
                get_mix_result(rgb_in, combine_hsv(rgb_in, rgb, (true, true, false)))
            }
            BlendMode::Luminosity => {
                get_mix_result(rgb_in, combine_hsv(rgb_in, rgb, (false, false, true)))
            }
        };

        outpal_at_palette_to_shift[offset..offset + 3].copy_from_slice(&out_color_slice);
    }
}

fn best_color(playpal: &[u8], rgb: (u8, u8, u8)) -> u8 {
    let mut best_distortion =
        ((rgb.0 as i32).pow(2) + (rgb.1 as i32).pow(2) + (rgb.2 as i32).pow(2)) * 2;
    let mut best_color = 0;

    for i in 0..256 {
        let offset = i * 3;
        let (in_r, in_g, in_b) = (
            playpal[0 + offset] as i32,
            playpal[1 + offset] as i32,
            playpal[2 + offset] as i32,
        );
        let (dr, dg, db) = (
            rgb.0 as i32 - in_r,
            rgb.1 as i32 - in_g,
            rgb.2 as i32 - in_b,
        );

        let distortion = dr.pow(2) + dg.pow(2) + db.pow(2);

        best_distortion = if distortion < best_distortion {
            best_color = i;
            distortion
        } else {
            best_distortion
        }
    }

    best_color as u8
}

fn build_lights_colormap(
    playpal: &[u8],
    colormap: &mut [u8],
    fade_color: (i32, i32, i32),
    mode: BlendMode,
) {
    let get_mix_result = |rgb_in: (i32, i32, i32), target: (i32, i32, i32), darkness_level: i32| {
        let mix = mix_colors(rgb_in, target, darkness_level as f32 / 32.0);

        (mix.0 as u8, mix.1 as u8, mix.2 as u8)
    };
    for darkness_level in 0..32 {
        for color in 0..256 {
            let rgb_in = (
                playpal[color * 3] as i32,
                playpal[color * 3 + 1] as i32,
                playpal[color * 3 + 2] as i32,
            );
            let rgb_out = match mode {
                BlendMode::Normal => (
                    (fade_color.0 + ((rgb_in.0 - fade_color.0) * (32 - darkness_level) + 16) / 32)
                        as u8,
                    (fade_color.1 + ((rgb_in.1 - fade_color.1) * (32 - darkness_level) + 16) / 32)
                        as u8,
                    (fade_color.2 + ((rgb_in.2 - fade_color.2) * (32 - darkness_level) + 16) / 32)
                        as u8,
                ),
                BlendMode::Multiply => {
                    let target = (
                        (fade_color.0 as f32 / 255.0 * rgb_in.0 as f32 / 255.0) as i32 * 255,
                        (fade_color.1 as f32 / 255.0 * rgb_in.1 as f32 / 255.0) as i32 * 255,
                        (fade_color.2 as f32 / 255.0 * rgb_in.2 as f32 / 255.0) as i32 * 255,
                    );
                    get_mix_result(rgb_in, target, darkness_level)
                }
                BlendMode::Screen => {
                    let target = (
                        (1.0 - (1.0 - fade_color.0 as f32 / 255.0)
                            * (1.0 - rgb_in.0 as f32 / 255.0)) as i32
                            * 255,
                        (1.0 - (1.0 - fade_color.1 as f32 / 255.0)
                            * (1.0 - rgb_in.1 as f32 / 255.0)) as i32
                            * 255,
                        (1.0 - (1.0 - fade_color.2 as f32 / 255.0)
                            * (1.0 - rgb_in.2 as f32 / 255.0)) as i32
                            * 255,
                    );
                    get_mix_result(rgb_in, target, darkness_level)
                }
                BlendMode::Hue => get_mix_result(
                    rgb_in,
                    combine_hsv(rgb_in, fade_color, (true, false, false)),
                    darkness_level,
                ),
                BlendMode::Saturation => get_mix_result(
                    rgb_in,
                    combine_hsv(rgb_in, fade_color, (false, true, false)),
                    darkness_level,
                ),
                BlendMode::Color => get_mix_result(
                    rgb_in,
                    combine_hsv(rgb_in, fade_color, (true, true, false)),
                    darkness_level,
                ),
                BlendMode::Luminosity => get_mix_result(
                    rgb_in,
                    combine_hsv(rgb_in, fade_color, (false, false, true)),
                    darkness_level,
                ),
            };

            colormap[darkness_level as usize * 256 + color as usize] = best_color(playpal, rgb_out);
        }
    }
}

fn combine_hsv(
    top_color: (i32, i32, i32),
    bottom_color: (i32, i32, i32),
    hsv_from_top: (bool, bool, bool),
) -> (i32, i32, i32) {
    use crate::MySrgb;
    use palette::{Hsv, IntoColor, Srgb};
    let (color1_srgb, color2_srgb): (MySrgb<f32>, MySrgb<f32>) =
        (top_color.into(), bottom_color.into());
    let (color1_hsv, color2_hsv): (Hsv, Hsv) =
        ((color1_srgb.0).into_color(), color2_srgb.0.into_color());

    let resulting_hsv = Hsv::new(
        if hsv_from_top.0 {
            color1_hsv.hue
        } else {
            color2_hsv.hue
        },
        if hsv_from_top.1 {
            color1_hsv.saturation
        } else {
            color2_hsv.saturation
        },
        if hsv_from_top.2 {
            color1_hsv.value
        } else {
            color2_hsv.value
        },
    );
    let resulting_srgb: Srgb<f32> = resulting_hsv.into_color();
    (
        (resulting_srgb.red * 255.0) as i32,
        (resulting_srgb.green * 255.0) as i32,
        (resulting_srgb.blue * 255.0) as i32,
    )
}

fn mix_colors(color1: (i32, i32, i32), color2: (i32, i32, i32), factor: f32) -> (i32, i32, i32) {
    use crate::MySrgb;
    use palette::Mix;
    let (color1_srgb, color2_srgb): (MySrgb<f32>, MySrgb<f32>) = (color1.into(), color2.into());
    let mixed_srgb = Mix::mix(color1_srgb.0, color2_srgb.0, factor);
    (
        (mixed_srgb.red * 255.0) as i32,
        (mixed_srgb.green * 255.0) as i32,
        (mixed_srgb.blue * 255.0) as i32,
    )
}

fn build_invulnerability_colormap(
    playpal: &[u8],
    colormap_at_invuln_start: &mut [u8],
    low_color: (i32, i32, i32),
    high_color: (i32, i32, i32),
) {
    for color in 0..256 {
        let (r, g, b) = (
            (playpal[color * 3] as f32) / 256.0,
            (playpal[color * 3 + 1] as f32) / 256.0,
            (playpal[color * 3 + 2] as f32) / 256.0,
        );

        let brightness = 1.0 - (r * 0.299 + g * 0.587 + b * 0.144);
        let mixed_color = mix_colors(low_color, high_color, brightness);
        colormap_at_invuln_start[color] = best_color(
            playpal,
            (
                mixed_color.0 as u8,
                mixed_color.1 as u8,
                mixed_color.2 as u8,
            ),
        )
    }
}

fn build_hurt_palette(
    playpal: &[u8],
    playpal_at_hurt_start: &mut [u8],
    rgb: (i32, i32, i32),
    mode: BlendMode,
) {
    for i in 1..9 {
        color_shift_palette(
            playpal,
            &mut playpal_at_hurt_start[(i - 1) * 256 * 3..],
            rgb,
            i as i32,
            9,
            &mode,
        );
    }
}

fn build_pickup_palette(
    playpal: &[u8],
    playpal_at_pickup_start: &mut [u8],
    rgb: (i32, i32, i32),
    mode: BlendMode,
) {
    for i in 1..5 {
        color_shift_palette(
            playpal,
            &mut playpal_at_pickup_start[(i - 1) * 256 * 3..],
            rgb,
            i as i32,
            8,
            &mode,
        );
    }
}

fn build_radiation_palette(
    playpal: &[u8],
    playpal_at_radiation_start: &mut [u8],
    rgb: (i32, i32, i32),
    mode: BlendMode,
) {
    color_shift_palette(playpal, playpal_at_radiation_start, rgb, 1, 8, &mode);
}

pub fn build_palette(
    playpal_page_0: &[u8],
    outpal: &mut [u8],
    hurt_color: (i32, i32, i32),
    pickup_color: (i32, i32, i32),
    radiation_color: (i32, i32, i32),
    hurt_mode: BlendMode,
    pickup_mode: BlendMode,
    radiation_mode: BlendMode,
) {
    outpal[0..256 * 3].copy_from_slice(&playpal_page_0[0..256 * 3]);
    build_hurt_palette(
        playpal_page_0,
        &mut outpal[256 * 1 * 3..],
        hurt_color,
        hurt_mode,
    );
    build_pickup_palette(
        playpal_page_0,
        &mut outpal[256 * 9 * 3..],
        pickup_color,
        pickup_mode,
    );
    build_radiation_palette(
        playpal_page_0,
        &mut outpal[256 * 13 * 3..],
        radiation_color,
        radiation_mode,
    );
}

pub fn build_vanilla_palette(playpal_page_0: &[u8], outpal: &mut [u8]) {
    build_palette(
        playpal_page_0,
        outpal,
        (255, 0, 0),
        (215, 186, 69),
        (0, 256, 0),
        BlendMode::Normal,
        BlendMode::Normal,
        BlendMode::Normal,
    );
}

pub fn build_colormap(
    playpal_page_0: &[u8],
    outmap: &mut [u8],
    fade_color: (i32, i32, i32),
    invuln_low_color: (i32, i32, i32),
    invuln_high_color: (i32, i32, i32),
    fade_mode: BlendMode,
) {
    for i in 0u8..=255u8 {
        outmap[i as usize] = i;
    }

    build_lights_colormap(playpal_page_0, outmap, fade_color, fade_mode);
    build_invulnerability_colormap(
        playpal_page_0,
        &mut outmap[256 * 32..],
        invuln_low_color,
        invuln_high_color,
    );

    for i in 256 * 33..256 * 34 {
        outmap[i] = 0
    }
}

pub fn build_vanilla_colormap(playpal_page_0: &[u8], outmap: &mut [u8]) {
    build_colormap(
        playpal_page_0,
        outmap,
        (0, 0, 0),
        (0, 0, 0),
        (255, 255, 255),
        BlendMode::Normal,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assets, constants};

    #[test]
    fn vanilla_palette_parity() {
        let mut outpal = vec![0; constants::PLAYPAL_LEN];
        build_vanilla_palette(assets::VANILLA_PLAYPAL, &mut outpal);
        assert_eq!(outpal, assets::VANILLA_PLAYPAL);
    }

    #[test]
    fn vanilla_colormap_parity() {
        let mut outmap = vec![0; constants::COLORMAP_LEN];
        build_vanilla_colormap(assets::VANILLA_PLAYPAL, &mut outmap);
        assert_eq!(outmap, assets::VANILLA_COLORMAP);
    }

    #[test]
    fn grayscale_mix() {
        assert_eq!(mix_colors((0, 0, 0), (255, 255, 255), 0.0), (0, 0, 0));
        assert_eq!(mix_colors((0, 0, 0), (255, 255, 255), 1.0), (255, 255, 255));
        assert_eq!(mix_colors((0, 0, 0), (255, 255, 255), 0.5), (127, 127, 127));
        assert_eq!(mix_colors((0, 0, 0), (255, 255, 255), 0.25), (63, 63, 63));
    }
    #[test]
    fn grayscale_mix_equates_to_direct_best_color_lookup() {
        for i in 0..255 {
            let gray = i as u8;
            let gray_f32 = gray as f32 / 255.0;

            let direct_lookup = best_color(assets::VANILLA_PLAYPAL, (gray, gray, gray));

            let mixed_color = mix_colors((0, 0, 0), (255, 255, 255), gray_f32);
            let mixed_lookup = best_color(
                assets::VANILLA_PLAYPAL,
                (
                    mixed_color.0 as u8,
                    mixed_color.1 as u8,
                    mixed_color.2 as u8,
                ),
            );

            assert_eq!(direct_lookup, mixed_lookup);
        }
    }
}
