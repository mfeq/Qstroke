# MFEKstroke

(c) 2021 Matthew Blanchard, Fredrick R. Brennan & MFEK Authors

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

## Variable Width Stroke
![Besley Small Caps Q](https://raw.githubusercontent.com/MFEK/stroke/main/docs/blob/MFEKstroke%20VWS%20Q.png)

## `MFEKstroke --help`
### Pattern Along Path
```
MFEKstroke-PAP 0.2.1
Matthew Blanchard <matthewrblanchard@gmail.com>; Fredrick R. Brennan <copypasteⒶkittens.ph>; MFEK Authors
Maps a pattern glyph along a path glyph.

USAGE:
    MFEKstroke PAP [OPTIONS] --path <path> --pattern <pattern>

OPTIONS:
    -p, --pattern <pattern>           The path to the input pattern file. You may also provide either --dot-pattern or
                                      --dash-pattern to use built-in patterns.
    -=, --dash-pattern                Use a simple dash pattern
    -., --dot-pattern                 Use a simple dot pattern
    -P, --path <path>                 The path to the input path file.
    -o, --output <output>             The path where the output will be saved. If omitted, or `-`, stdout.
                                      
                                      
    -c, --contour <contour>           <isize> if this is a positive number we stroke only that specific contour in the
                                      outline by index. [default: -1]
    -m, --mode <mode>                 Repeat mode. [default: single]  [possible values: single, repeated]
    -s, --subdivide <subdivide>       <usize> how many times to subdivide the patterns at their midpoint. [default: 0]
                                      
                                      
    -X, --sx <sx>                     <f64> how much we scale our input pattern on the x-axis. [default: 1]
    -Y, --sy <sy>                     <f64> how much we scale our input pattern on the y-axis. [default: 1]
    -n, --noffset <normal-offset>     <f64> how much to offset the pattern along the normal of the path. [default: 0]
    -t, --toffset <tangent-offset>    <f64> how much to offset the pattern along the tangent of the path. [default: 0]
                                      
                                      
    -W, --spacing <spacing>           <f64> how much padding to trail each copy with. [default: 0]
    -!, --stretch <stretch>           <stretch> false if not given, true if given, spacing mode if value of spacing
                                      given [possible values: spacing]
    -S, --simplify                    <boolean> if we should run the result through Skia's (buggy) simplify routine.
    -O, --overdraw <overdraw>         <f64> pattern copies overlapping more than arg% are removed. [default: 100%]
    -Q, --one-pass                    <boolean> whether we should not reflow the path after culling during overdraw
                                      (faster but worse).
    -C, --no-center-pattern           <boolean> supply if you wish to center the pattern
    -r, --reverse                     <boolean> true will reverse the path.
    -R, --reverse-culling             <boolean> true will reverse the order we check for overlaps during overlap
                                      culling.
                                      
    -h, --help                        Prints help information
    -V, --version                     Prints version information
```
### Variable Width Stroking
(Note: In VWS mode, it is expected that you are using MFEKglif to generate the input files. Therefore, not many helpful command line options are provided. If you wish to use VWS programatically, play with MFEKglif's VWS tool, get some output, and study it; then generate conformant XML.)

```
MFEKstroke-VWS 0.1
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
MFEKstroke-CWS 0.1
Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Matthew Blanchard <matthewrblanchard@gmail.com>
Takes a .glif file and strokes it at a constant width.

USAGE:
    MFEKstroke CWS [FLAGS] [OPTIONS] --input <input> --output <output> --width <width>

FLAGS:
    -h, --help               Prints help information
    -E, --remove-external    Remove external contour
    -I, --remove-internal    Remove internal contour
    -V, --version            Prints version information

OPTIONS:
    -i, --input <input>          The path to the input file.
    -o, --output <output>        The path where the output will be saved.
    -e, --endcap <endcap>        Either the constant strings "round" or "square", or a .glif file. [default: round]
    -j, --jointype <jointype>    Either of the constant strings "round", "miter", or "bevel". [default: round]
    -l, --left <left>            <f64> Constant stroke width (left).
    -r, --right <right>          <f64> Constant stroke width (right).
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
    -n, --nib <nib>              The path to the nib file. FontForge is quite strict about these. The .glif must contain
                                 a single closed spline, running clockwise, which represents a convex shape.
    -i, --input <input>          The path to the input path file.
    -o, --output <output>        The path where the output .glif will be saved.
    -a, --accuracy <accuracy>    <f64> Accuracy target [default: 0.25]
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

# Contributing

The biggest TODO's right now:

* tests;
* documentation/demos/user stories;
* descriptions of how the algorithms work.
