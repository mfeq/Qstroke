# QStroke
A set of utilities for stroking paths in font glyphs written in rust.

QStroke takes unified font object files and applies path stroking algorithms to them. Currently only pattern along path is provided.

## Pattern Along Path

![alt text](https://user-images.githubusercontent.com/310356/104104000-4955ab80-5273-11eb-9d16-4b8052a05df7.PNG)

This was generated with the following command:

```
cargo run -- --pattern simple.glif --path Q_.glif --out output.glif --sx 0.3 --sy 0.1 --stretch true --subdivide 2 --mode repeated
```
