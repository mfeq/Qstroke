# Qstroke
A set of utilities for stroking paths in font glyphs written in rust.

This program is part of the [MFEQ project](https://github.com/mfeq/mfeq/).

Qstroke takes unified font object files and applies path stroking algorithms to them. Currently only pattern along path is provided.

## Pattern Along Path

![alt text](https://user-images.githubusercontent.com/310356/104128458-9ac66f00-5335-11eb-94d3-f458f6cfb464.png)

This was generated with the following command:

```
cargo run -- --pattern simple.glif --path Q_.glif --out arrow.glif --sx 0.3 --sy 0.1 --stretch true --subdivide 2 --mode repeated
```

(c) 2021 Matthew Blanchard
(c) 2021 MFEQ authors

License: Apache 2
