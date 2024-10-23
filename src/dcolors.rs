// Source: https://www.doomworld.com/idgames/historic/dmutils

pub struct BuildColor(pub i32, pub i32, pub i32);

fn color_shift_palette(
    inpal: &[u8],
    outpal_at_palette_to_shift: &mut [u8],
    r: i32,
    g: i32,
    b: i32,
    shift: i32,
    steps: i32,
) {
    for i in 0..256 {
        let offset = i * 3;
        let (in_r, in_g, in_b) = (inpal[0 + offset], inpal[1 + offset], inpal[2 + offset]);
        let (dr, dg, db) = (r - in_r as i32, g - in_g as i32, b - in_b as i32);

        outpal_at_palette_to_shift[offset..offset + 3].copy_from_slice(&[
            (in_r as i32 + dr * shift / steps) as u8,
            (in_g as i32 + dg * shift / steps) as u8,
            (in_b as i32 + db * shift / steps) as u8,
        ]);
    }
}

fn best_color(playpal: &[u8], r: u8, g: u8, b: u8) -> u8 {
    let mut best_distortion = ((r as i32).pow(2) + (g as i32).pow(2) + (b as i32).pow(2)) * 2;
    let mut best_color = 0;

    for i in 0..256 {
        let offset = i * 3;
        let (in_r, in_g, in_b) = (
            playpal[0 + offset] as i32,
            playpal[1 + offset] as i32,
            playpal[2 + offset] as i32,
        );
        let (dr, dg, db) = (r as i32 - in_r, g as i32 - in_g, b as i32 - in_b);

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

fn build_lights_colormap(playpal: &[u8], colormap: &mut [u8], fade_color: BuildColor) {
    /*
    Note: The strange order of operations is intentional for integer arithmetic.
    Likewise, the '+16' exists to round the color to its nearest whole value when
    the division happens.
     */
    fn lerp_to_darkness(pal_color: u8, target_color: i32, darkness_level: i32) -> i32 {
        target_color + ((pal_color as i32 - target_color) * (32 - darkness_level) + 16) / 32
    }

    for darkness_level in 0..32 {
        for color in 0..256 {
            let (r, g, b) = (
                ((playpal[color * 3] as i32) * (32 - darkness_level) + 16) / 32,
                ((playpal[color * 3 + 1] as i32) * (32 - darkness_level) + 16) / 32,
                ((playpal[color * 3 + 2] as i32) * (32 - darkness_level) + 16) / 32,
            );

            colormap[darkness_level as usize * 256 + color as usize] =
                best_color(playpal, r as u8, g as u8, b as u8);
        }
    }
}

fn build_invulnerability_colormap(playpal: &[u8], colormap_at_invuln_start: &mut [u8]) {
    for color in 0..256 {
        let (r, g, b) = (
            (playpal[color * 3] as f32) / 256.0,
            (playpal[color * 3 + 1] as f32) / 256.0,
            (playpal[color * 3 + 2] as f32) / 256.0,
        );

        let gray = (255.0 * (1.0 - (r * 0.299 + g * 0.587 + b * 0.144))) as u8;
        colormap_at_invuln_start[color] = best_color(playpal, gray, gray, gray)
    }
}

fn build_hurt_palette(playpal: &[u8], playpal_at_hurt_start: &mut [u8], r: i32, g: i32, b: i32) {
    for i in 1..9 {
        color_shift_palette(
            playpal,
            &mut playpal_at_hurt_start[(i - 1) * 256 * 3..],
            r,
            g,
            b,
            i as i32,
            9,
        );
    }
}

fn build_pickup_palette(
    playpal: &[u8],
    playpal_at_pickup_start: &mut [u8],
    r: i32,
    g: i32,
    b: i32,
) {
    for i in 1..5 {
        color_shift_palette(
            playpal,
            &mut playpal_at_pickup_start[(i - 1) * 256 * 3..],
            r,
            g,
            b,
            i as i32,
            8,
        );
    }
}

fn build_radiation_palette(
    playpal: &[u8],
    playpal_at_radiation_start: &mut [u8],
    r: i32,
    g: i32,
    b: i32,
) {
    color_shift_palette(playpal, playpal_at_radiation_start, r, g, b, 1, 8);
}

pub fn build_palette(
    playpal_page_0: &[u8],
    outpal: &mut [u8],
    hurt_color: BuildColor,
    pickup_color: BuildColor,
    radiation_color: BuildColor,
) {
    outpal[0..256 * 3].copy_from_slice(&playpal_page_0[0..256 * 3]);
    build_hurt_palette(
        playpal_page_0,
        &mut outpal[256 * 1 * 3..],
        hurt_color.0,
        hurt_color.1,
        hurt_color.2,
    );
    build_pickup_palette(
        playpal_page_0,
        &mut outpal[256 * 9 * 3..],
        pickup_color.0,
        pickup_color.1,
        pickup_color.2,
    );
    build_radiation_palette(
        playpal_page_0,
        &mut outpal[256 * 13 * 3..],
        radiation_color.0,
        radiation_color.1,
        radiation_color.2,
    );
}

pub fn build_vanilla_palette(playpal_page_0: &[u8], outpal: &mut [u8]) {
    build_palette(
        playpal_page_0,
        outpal,
        BuildColor(255, 0, 0),
        BuildColor(215, 186, 69),
        BuildColor(0, 256, 0),
    );
}

pub fn build_colormap(playpal_page_0: &[u8], outmap: &mut [u8], fade_color: BuildColor) {
    for i in 0u8..=255u8 {
        outmap[i as usize] = i;
    }

    build_lights_colormap(playpal_page_0, outmap, fade_color);
    build_invulnerability_colormap(playpal_page_0, &mut outmap[256 * 32..]);

    for i in 256 * 33..256 * 34 {
        outmap[i] = 0
    }
}

pub fn build_vanilla_colormap(playpal_page_0: &[u8], outmap: &mut [u8]) {
    build_colormap(playpal_page_0, outmap, BuildColor(0, 0, 0));
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
}
