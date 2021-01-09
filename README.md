# Archived

This repo is archived. Please do not use this code. You can find an updated version of this stroking utility at https://github.com/MFEK/stroke. You can also find an updated version of the algorithms at https://github.com/MFEK/math.rlib.

# Stroke
A set of utilities for stroking paths in font glyphs written in rust.

It is a part of the MFEQ editor project.

Stroke takes unified font object files and applies path stroking algorithms to them. Currently only pattern along path is provided.

## Pattern Along Path

This was generated with the following command:

```
cargo run -- --pattern simple.glif --path Q_.glif --out output.glif --sx 0.3 --sy 0.1 --stretch true --subdivide 2 --mode repeated
```

(c) 2021 Matthew Blanchard
(c) 2021 MFEQ authors
