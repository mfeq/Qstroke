# Qstroke

(c) 2021 Matthew Blanchard, MFEQ authors

A set of utilities for stroking paths in font glyphs written in Rust.

This program is part of the [MFEQ project](https://github.com/mfeq/mfeq/).

Qstroke takes UFO `.glif` files and applies path stroking algorithms to them. Currently only pattern-along-path (PaP) is provided.

## Pattern Along Path
### As seen in Qglif…
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

## `Qstroke --help`
### Pattern Along Path
```
QPaP 0.0.0
Matthew Blanchard <matthewrblanchard åţ gmail … com>
A utility for applying pattern-along-path to ufo files.

USAGE:
    Qstroke [OPTIONS] --out <output> --path <path> --pattern <pattern>

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
