# MFEKstroke

(c) 2021 Matthew Blanchard, Fredrick R. Brennan & MFEK authors

A set of utilities for stroking paths in font glyphs written in Rust.

This program is part of the [MFEK project](https://github.com/MFEK/).

MFEKstroke takes UFO `.glif` files and applies path stroking algorithms to them.

Four stroking algorithms are provided:

* PAP (**P**attern-**A**long-**P**ath)
* VWS (**V**ariable **W**idth **S**troking)
* CWS (**C**onstant **W**idth **S**troking)
* Nib (requires FontForge be installed, uses `libfontforge.(so|dll)`)

This makes MFEKstroke more complete in this department than Glyphsapp, FontForge or Runebender.

# Building

MFEKstroke is a Rust project. The best way to get the Rust toolchain is via [rustup](https://rustup.rs/).

A Makefile is provided for your convenience.

The build command is:

```bash
make
```

Which runs ``cargo build``.

You can provide the environment variables `FONTFORGE` and `DEBUG` to `make`. Without `DEBUG`, the binary goes in `target/release`; otherwise `target/debug`. `FONTFORGE` will compile the nib stroking mode; it will only work if `libfontforge.so` (`.dll` on Windows) is locatable by `cargo`.

```bash
DEBUG=y FONTFORGE=y make
```

## Pattern Along Path
### As seen in MFEK/glif…
![Chomsky `Q` patterned with arrows](https://user-images.githubusercontent.com/310356/104128458-9ac66f00-5335-11eb-94d3-f458f6cfb464.png)

This was generated with the following command:

```
cargo run -- --pattern simple.glif --path Q_.glif --out arrow.glif --sx 0.3 --sy 0.1 --stretch true --subdivide 2 --mode repeated
```

### As seen in FontForge…
![FRB Standard Cursive `elk` patterened with arrows](https://user-images.githubusercontent.com/838783/104132949-8d69ae80-534e-11eb-811f-afd18e5fe405.png)

```
cargo run -- --out Untitled2.ufo/glyphs/k.low.glif --path FRBStandardCursive-Regular.ufo/glyphs/k.low.glif --pattern arrow.ufo/glyphs/arrow.glif -m repeated --sx 0.1 --sy 0.1 -s 3 --simplify true --stretch true
```

## Nib Stroking
![FRB Standard Cursive with a calligraphic nib](https://raw.githubusercontent.com/MFEK/stroke/main/docs/blob/MFEKstroke%20NIB%20Q.png)

## `MFEKstroke --help`
### Pattern Along Path
```
MFEK-PaP 0.0.0
Matthew Blanchard <matthewrblanchard åţ gmail … com>
A utility for applying pattern-along-path to ufo files.

USAGE:
    MFEKstroke [OPTIONS] --out <output> --path <path> --pattern <pattern>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --center_pattern <center_pattern>    <boolean (true)> if you want to align a pattern manually you can change
                                             this to false.
    -m, --mode <mode>                        <[single|repeated] (single)> set our repeat mode.
        --noffset <normal_offset>            <f64 (0)> how much to offset the pattern along the normal of the path.
        --out <output>                       The path where the output will be saved.
        --path <path>                        The path to the input path file.
        --pattern <pattern>                  The path to the input pattern file.
        --sx <scale_x>                       <f64 (1)> how much we scale our input pattern on the x-axis.
        --sy <scale_y>                       <f64 (1)> how much we scale our input pattern on the y-axis.
        --simplify <simplify>                <boolean (false)> if we should run the result through a simplify routine.
        --spacing <spacing>                  <f64 (0)> how much padding to trail each copy with.
        --stretch <stretch>                  <boolean (false)> whether to stretch the input pattern or not.
    -s, --subdivide <subdivide>              <f64 (0)> how many times to subdivide the patterns at their midpoint.
        --toffset <tangent_offset>           <f64 (0)> how much to offset the pattern along the tangent of the path.
````
### Variable Width Stroking
(Note: In VWS mode, it is expected that you are using MFEKglif to generate the input files. Therefore, not many helpful command line options are provided. If you wish to use VWS programatically, play with MFEKglif's VWS tool, get some output, and study it; then generate conformant XML.)

```
MFEKstroke-VWS 0.0
Matthew Blanchard <matthewrblanchard@gmail.com>
Takes a .glif file and strokes it with variable width.

USAGE:
    MFEKstroke VWS -i <input> -o <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <input>         The path to the input file.
    -o <output>        The path where the output will be saved.
```

### Constant Width Stroking

```
MFEKstroke-CWS 0.0
Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Matthew Blanchard <matthewrblanchard@gmail.com>
Takes a .glif file and strokes it at a constant width.

USAGE:
    MFEKstroke CWS [OPTIONS] --input <input> --output <output> --width <width>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>          The path to the input file.
    -o, --output <output>        The path where the output will be saved.
    -e, --endcap <endcap>        Either the constant strings "round" or "square", or a .glif file. [default: round]
    -j, --jointype <jointype>    Either of the constant strings "round", "miter", or "bevel". [default: round]
    -s, --startcap <startcap>    Either the constant strings "round" or "square", or a .glif file. [default: round]
    -w, --width <width>          <f64> Constant stroke width.
```

### Nib Stroking

```
MFEKstroke-NIB 0.1.0
Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Skef Iterum (FontForge)
Takes a nib and a path, both in .glif format, and emulates a pen, with the chosen nib, stroking the path.

Important note: FontForge is used for this, so it may be more unstable than other modes as FontForge is implemented in C
and not memory safe. To prevent bugs, we turn off simplification and overlap removal. Use MFEK for that.

USAGE:
    MFEKstroke NIB --input <input> --nib <nib> --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --nib <nib>          The path to the nib file. FontForge is quite strict about these. The .glif must contain a
                             single closed spline, running clockwise, which represents a convex shape.
    -i, --input <input>      The path to the input path file.
    -o, --output <output>    The path where the output .glif will be saved.
```

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
