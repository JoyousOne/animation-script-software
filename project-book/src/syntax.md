# Syntax

File example:

```
---
format:   gif
filename: ./my_animation.gif
width:    10
height:   10
---

r_to_g ANIM
    color red -> green
    transition ease-in 2s,


RECT r1
    height    10
    width     10
    position  [0, 5]

r_to_g(r1)
```

## Overview :

### [Configuration](./config.md)

```
---
format:   gif
filename: ./my_animation.gif
width:    10
height:   10
---
```

### [Animation](./animation.md)

```
r_to_g ANIM
    color red -> green
    transition ease-in 2s,
```

### [Shapes](./shapes.md)

```
RECT r1
    height    10
    width     10
    position  [0, 5]
```

### [Colors](./colors.md)

```
// rgb codes
red   = rgb(255, 0, 0)
green = rgb(0, 255, 0)
blue  = rgb(0, 0, 255)

// hexa codes
white = #FFFFFF
black = #000000
```
