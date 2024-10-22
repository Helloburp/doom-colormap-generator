# Doom Colormap Generator

Usage: `doom-colormap-generator --input [path_to_input].json --output [path_to_output_dir]`

## Valid Input Format

```json
{
    "distance_fade": {"red":0, "green":0, "blue": 0},
    "masking": {
        "keep_hue": false,
        "keep_saturation": false,
        "keep_value": false
    }
}
```