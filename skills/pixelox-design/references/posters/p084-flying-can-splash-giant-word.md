---
id: p084
title: Flying can, white splash, giant word
genre: [drink-ad, product-ad, social-post]
mood: [dramatic, energetic, dark, bold]
style: [photo-composite, 3d-render]
format: square
archetype: A-text-behind-subject
palette: ['#431212', '#7f1d1d', '#d23939', '#886059', '#c9968e', '#eed8d2']
techniques: [T-text-behind-subject, T-fit-width, T-gradient-type, T-cutout-drop-shadow, T-vignette, T-water-splash]
needs: product-cutout
---

# p084: Flying can, white splash, giant word

## 1. Concept

A square product post with real impact: a red can tumbles through the frame at an angle,
a sheet of white water bursts behind it, and a four-letter word in giant red capitals sits
behind both on a dark maroon ground, repeated once more at the bottom edge. Three layers
of depth (word, water, can) and only two colors (red and the white of the water). The
tilt is what gives it motion; an upright can in the same scene would look parked.

## 2. Format and grid

* Square. No margins: the word, the splash, and the can all run close to or past the
  edges.
* The can lies on a diagonal, its top toward the upper left: axis rotated about 18 degrees
  counter-clockwise, centered at 50% W, 52% H, 68% H long and 35% W wide.
* The splash crosses the other diagonal: from 13% W, 18% H at the upper left down to
  88% W, 78% H at the lower right, so can and water form an X.
* Giant word: 19 to 78% W, 4 to 33% H. Its echo: the same word, same size, cropped by the
  bottom edge, visible from 77% H down, much dimmer.
* Corners are the darkest part of the image.

## 3. Layer stack

1. Ground: dark maroon `#431212`.
2. Glow: a warmer red pool `#7f1d1d` behind the center, slightly right of the middle.
3. Giant word: 4 heavy condensed capitals, bright red `#d23939` at the top of the letters,
   fading to the ground color at their feet; cap height 29% H, 59% W wide. The can's top
   and the splash hide the lower half of its middle letters.
4. Echo word: the same word again along the bottom edge at about 40% opacity, only its
   upper two thirds on the canvas.
5. Splash: white-to-pink water `#eed8d2` in a wide, torn sheet behind the can, with fine
   spray and separate drops flying to both sides; its shadows pick up the ground's red
   `#c9968e`, `#886059`.
6. Can: a red aluminum can with white script lettering running along its length and a
   pale ribbon line around it, pull-tab up; glossy highlights along the upper edge, deep
   red shadow on the underside.
7. Vignette: maroon-black in all four corners.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#431212` | maroon ground | 42% |
| `#7f1d1d` | glow, can shadows | 29% |
| `#d23939` | giant word, can | 8% |
| `#886059` | shaded water | 7% |
| `#c9968e` | mid-tone water | 7% |
| `#eed8d2` | bright water, can lettering | 7% |

Red monochrome with a pink-white accent. The water is never pure white: it carries a
little of the ground color, which keeps it in the scene.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Giant word | 1 word, 4 letters (the product type or half the brand name) | caps | heavy condensed grotesque (Anton, Bebas Neue / Impact) | 400 (the face is heavy) | 29% | 0 | - | center | `#d23939` fading down to `#5a1414` | 19 to 78% W, top 4% |
| Echo word | the same word | caps | same | same | 29% | 0 | - | center | same, 40% opacity | 19 to 78% W, top 77% (cropped) |

No small text. The lettering on the can is part of the product.

## 6. Imagery

A 3D-style product render: one can in mid-air with a liquid collision behind it. The can
is lit from the upper left with a hard highlight; the splash is a frozen sheet with a
torn edge, larger than the can. For a build, a can cutout is rotated and laid over a
white splash that is turned across the opposite diagonal; add loose drops on both
sides.

## 7. Effects and textures

* **T-text-behind-subject**: word, then splash, then can. The can covers no more than the
  lower half of two letters.
* **T-fit-width** (640 px on a 1080 px canvas) and **T-gradient-type**: `fx overlay` at 90
  degrees from `#5a1414` at the letters' feet to `#e84848` at the top, so the word sinks
  into the ground.
* Echo: `layer dup`, moved to the bottom edge, opacity 40.
* **T-water-splash** (`scripts/splash.js`) in pink-white, rotated 35 degrees,
  `screen` mode at 90%; `particles.js` dots for spray.
* **T-cutout-drop-shadow**, soft: offset 20,30, blur 50, dark maroon, 60%.
* **T-vignette** in `#1a0505`, starting at 45% of the radius.
* **T-water-splash** in [techniques.md](../techniques.md) is the tested library version of the liquid splash; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The can (sharpest, most saturated, center).
2. The white splash around it.
3. The giant word above, read around the can's top.
4. The echo at the bottom.

The word's capitals are 29% H; the can is 68% H long. One level of type, used as a
backdrop.

## 9. Build recipe

`product.png` is your image here: a can or bottle cutout.

```bash
pixelox render js scripts/splash.js -o splash.png --size 1100x1100 --seed 7 --param color='#f2e4e0' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=20 --param drops=300 --param arc=360
pixelox render js scripts/particles.js -o spray.png --size 1080x1080 --seed 5 --param shape=dot --param colors='#ffffff,#f2d8d2,#e0b0a8' --param count=170 --param min=0.2 --param max=1.1 --param amin=0.4 --param amax=0.95 --param focus='0.5,0.5,0.46'
pixelox new -o post.pxo --size 1080x1080 --fill '#431212'
pixelox layer add post.pxo --in-place --name Glow
pixelox fill gradient post.pxo --in-place --layer Glow --kind radial --from 56%,46% --to 56%,100% --stops '#8a2020ff@0,#43121200@1'
pixelox text add post.pxo --in-place --name Word --content "SODA" --font Impact --size 40% --color '#d23939'
pixelox transform scale post.pxo --in-place --layer Word --size 640x
pixelox transform move post.pxo --in-place --layer Word --to 48.5%,4% --anchor t
pixelox fx overlay post.pxo --in-place --layer Word --stops '#5a1414@0,#d23939@0.55,#e84848@1' --angle 90
pixelox layer dup post.pxo --in-place --layer Word --name Echo
pixelox transform move post.pxo --in-place --layer Echo --to 48.5%,77% --anchor t
pixelox layer opacity post.pxo --in-place --layer Echo --value 40
pixelox comp place post.pxo splash.png --in-place --as Splash --at 50%,48% --anchor c
pixelox transform rotate post.pxo --in-place --layer Splash --degrees 35
pixelox layer blend post.pxo --in-place --layer Splash --mode screen
pixelox layer opacity post.pxo --in-place --layer Splash --value 90
pixelox comp place post.pxo spray.png --in-place --as Spray --fit cover
pixelox layer blend post.pxo --in-place --layer Spray --mode screen
pixelox comp place post.pxo product.png --in-place --as Can --size x800 --at 50%,52% --anchor c
pixelox transform rotate post.pxo --in-place --layer Can --degrees -18
pixelox fx drop-shadow post.pxo --in-place --layer Can --offset 20,30 --blur 50 --color '#1a0404' --opacity 60
pixelox layer add post.pxo --in-place --name Vignette
pixelox fill gradient post.pxo --in-place --layer Vignette --kind radial --from 50%,50% --to 50%,120% --stops '#1a050500@0.45,#1a0505cc@1'
pixelox layer add post.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise post.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend post.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity post.pxo --in-place --layer Grain --value 30
pixelox export post.pxo -o post.png
```

Change first: the word (3 to 5 letters; refit with `transform scale`), the can's angle
(-10 to -25 degrees), then the ground and word colors as one family.

## 10. Variations and pairings

* Blue version: ground `#0b1f4b`, word `#2f6bff`, water tinted `#dbeaff`. Citrus: ground
  `#5a3a00`, word `#ffb400`, juice splash in `#ffd76a` instead of water.
* Two words: one above and one below the can at full strength (no echo), for a two-word
  name.
* Portrait 4:5: word at 5 to 28% H, can 20 to 85% H, echo cropped at the bottom.
* Sneaker, burger, phone, or game controller instead of a can: any product that looks good
  thrown through the air.
* Pairs with T-motion-streak on the can (short, along its axis) and T-lens-flare on the
  rim.

## 11. Pitfalls

* Can and splash on the same diagonal merge into one blob. Cross them: can leaning left,
  water running the other way.
* A word in a wide font cannot reach 29% H cap height at 59% W. Use a condensed heavy face
  and 3 to 5 letters.
* Pure white water in `normal` mode looks pasted on. Tint it toward the ground and blend
  with `screen`.
* The word at the same brightness top to bottom fights the can. Let it fade into the
  ground toward its feet and keep the echo under 45% opacity.
* No shadow under the can: it sticks to the splash like a decal. Offset the shadow down
  and to the right, away from the light.
