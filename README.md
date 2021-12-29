# Yoyo

## About

This project is to learn the basic workings of a browser using [the classic browser tutorial](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html) by Matt Brubeck. I've additionally implemented inline layout, text rendering, a few more html/css specs, and unit tests. [The other great browser guide](https://browserbook.shift-js.info/chapters/rendering) helped me to implement the painting function. Thank you!

## What I learned

- What the browser does to receive and display the arbitrary html and css.
- How to parse html (css) while preserving tags and attributes (selectors and declarations) as a manageable object.
- How css's Specificity is set up and applied to an element.
- How to build a layout and calculate the position of each node depending on a display property including the case where "auto" is specified as the css value.

## Run

```sh
cargo run examples/test.html examples/test.css
```
