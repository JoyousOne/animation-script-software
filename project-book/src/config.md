# Config

The define at the start of the file and is contained within `---`.

## Propriety:

- **format** <_format_type_>: The format of the output file. For now, only `gif` is available.
- **filename** <_filename_>: The filename of the output file. The file extension must match the format.
- **width** <_width_>: The total width of the canvas.
- **height** <_height_>: The total height of the canvas.

## Example:

```
---
format:   gif
filename: ./my_animation.gif
width:    10
height:   10
---

// rest of the file here
```
