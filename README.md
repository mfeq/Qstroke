# Qstroke
A set of utilities for stroking paths in font glyphs written in rust.

This program is part of the [MFEQ project](https://github.com/mfeq/mfeq/).

Disclaimer: This repo has absolutely nothing to do with QAnon. It is a part of the MFEQ editor project.
Qstroke takes unified font object files and applies path stroking algorithms to them. Currently only pattern along path is provided.

## Pattern Along Path

![alt text](https://user-images.githubusercontent.com/310356/104104000-4955ab80-5273-11eb-9d16-4b8052a05df7.PNG)

This was generated with the following command:

```
cargo run -- --pattern simple.glif --path Q_.glif --out output.glif --sx 0.3 --sy 0.1 --stretch true --subdivide 2 --mode repeated
```

(c) 2021 Matthew Blanchard
(c) 2021 MFEQ authors
