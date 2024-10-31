# Doom Colormap Generator

Usage: `doom-colormap-generator --input [path_to_input].json --output [path_to_output_dir]`

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

Current valid blend modes are "Normal", "Multiply", and "Screen".
Support for Photoshop-like Hue/Saturation/Color/Luminosity modes are planned. for the future.
