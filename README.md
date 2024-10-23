# Doom Colormap Generator

Usage: `doom-colormap-generator --config [path_to_input].json --output [path_to_output_dir]`

## Valid Input Format

```json
{
    "distance_fade": {"red":50, "green":0, "blue": 0},
    "radiation_suit": {"red":0, "green":0, "blue": 255},
    "item_pickup": {"red":255, "green":0, "blue": 255},
    "hurt": {"red":0, "green":255, "blue": 0},
    "masking": {
        "keep_hue": false,
        "keep_saturation": false,
        "keep_value": false
    }
}
```