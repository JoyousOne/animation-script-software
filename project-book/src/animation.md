# Animation

keyword (`ANIM`):

- **color** (**_optional_**):
  - **color** <_color_>: Valid if the provided object has an initial color. Will save the difference between the 2 colors in order to be able to get the old color back if we need to reverse the animation.
  - **color** <_starting_color_> `->` <_ending_color_>: Set the starting color and the ending color. If the provided object already has a color, _starting_color_ will overwrite it.

- **delay** <_delay_> (**_optional_**): apply a delay to all present field. The `transition` field will override the delay for provided fields.

- **move** <[_x_, _y_]> (**_optional_**): movement to be applied on a movable element.

- **transition** <_transition_type_> [(\<field\> \<delay\>)...] (**_optional_**): css like syntax for transition.

Syntax:

```
[animation_name] ANIM
    [fields...]
```

<!-- ## Example(s) : -->

## **Declaring and applying an animation**:

To declare an animation, the keyword `ANIM` must follow a variable name.

```
// Defining the animation
r_to_g ANIM
    color red -> green
    delay 2s


// Defining a rectangle
r1 RECT
    height 10
    width  10
    position  [0, 0]

// Animation sequence
r_to_g(r1)
```

## **Animation Sequence**:

The **Animation sequence** is the last line and contains the animation in a declared order with the following **operators**:

- `->` (followed by).
  - The following: `animation1(x) -> animation2(y)` will execute `animation1(x)`, wait for it to finish, then starts the execution of `animation2(y)`.
  - > ```
    > animation1 ANIM
    >     color blue -> red
    >
    > animation2 ANIM
    >     color blue -> red
    >
    > x RECT
    >     width  1
    >     height 1
    >
    > y = x
    >
    > animation1(x) -> animation(y)
    > ```

- `[]` (concurrently).
  - The following `[animation1(x), animation2(y)] -> animation3(z)` will execute `animation1(x)` and `animation2(y)` concurrently and once **both** are done, `animation3(z)` start executing.
  - Concurrent animations cannot have the same given object. Meaning that `[animation1(x), animation2(x)]` would be considered invalid since both animation that the argument `x`.
  - Note that if two concurrent animations happened to have their content superposed, the ones declared last would appear **above** the ones declared first (if the intent is to merge the effects of 2 differents animation, go consult [Merging animation](#merging-animations)). Example for `[first_animation(rect1), second_animation(rect2)]` we would get:
    - > ```
      >        +--------+
      > +------| rect 2 |
      > | rec1 +--------+
      > +------+
      > ```

## **Reversing an animation**:

prefix an animation with `'` to reverse it.

```
---
config:
    width: 20
    heigth: 20
---

r_to_g ANIM
    color red -> green
    delay 2s


r1 RECT
    height 10
    width  10
    position  [0, 0]

// Animation sequence
r_to_g(r1) -> 'r_to_g(r1)
```

## **Merging animations**:

TODO COMPLETE
Merging animation can be done by using the `todo` operator.

```
---
config:
    width: 20
    heigth: 20
---

r_to_g ANIM
    color red -> green
    delay 2s

lat_move ANIM
    move [2, 2]
    delay 1s

g_to_b ANIM
    color green -> blue
    delay 1s

lat_move_rev ANIM
    move [-2, -2]
    delay 1s

first_segment = |x| r_to_g(lat_move(x))

r1 RECT
    height 10
    width  10
    position  [0, 0]

// Animation sequence
r_to_g(r1) -> lat_move(r1) ->
```
