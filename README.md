# Knitting Pattern Generator

Currently generator will generate a pattern for knitting a sphere.

[Visit the application](https://dmcallas.github.io/knitting-pattern-generator/)

## Install/run:

- Clone the repository
- [Install Rust](https://rustup.rs/) if it is not already installed.
- Run `rustup target add wasm32-unknown-unknown` to enable compiling
  Rust to WASM.
- Run `cargo install trunk` to install the `trunk` build tool.
- Run `trunk serve --open` to build and serve the application and open
  it in your browser.

## Using the application

- Select the units (in or cm) you will use for all of your
  measurements (This isn't part of the calculations but makes things
  look nicer).
- Enter the diameter of the desired sphere.
- Enter the row and stitches per unit of measurement in your gauge
  swatch. Decimals can be used.
