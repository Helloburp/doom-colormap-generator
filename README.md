# Doom Colormap Generator

Usage: `doom-colormap-generator <INPUT> --output <OUTPUT> --playpal <OUTPUT>`

If Output is not specified, an output directory will be created in the current directory named "output".

If Playpal is not specified, the default DOOM playpal will be used.

## Examples

Example 1: Input only. `doom-colormap-generator example-input/vanilla.json`
Example 2: With playpal override. `doom-colormap-generator example-input/vanilla.json --playpal my_custom_playpal.cmp`

## Valid Input Format

The following input file will produce the vanilla COLORMAP and PLAYPAL.
```json
{
    "distance_fade": {"red":0, "green":0, "blue": 0},
    "distance_fade_blend_mode": "Normal",

    "invulnerability_range_low": {"red":0, "green":0, "blue":0},
    "invulnerability_range_high": {"red":255, "green":255, "blue":255},

    "hurt": {"red":255, "green":0, "blue": 0},
    "hurt_blend_mode": "Normal",

    "radiation_suit": {"red":0, "green":256, "blue": 0},
    "radiation_suit_blend_mode": "Normal",

    "item_pickup": {"red":215, "green":186, "blue": 69},
    "item_pickup_blend_mode": "Normal"
}
```

Current valid blend modes are "Normal", "Multiply", "Screen", "Hue", "Saturation", "Color", and "Luminosity",