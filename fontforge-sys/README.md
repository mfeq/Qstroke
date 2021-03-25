# libfontforge-sys

We're just using this to get the SplineSetsStroke function.

Generated like this:

```bash
bindgen ~/Workspace/fontforge/fontforge/splinestroke.h -o src/bindings.rs -- -I/home/fred/Workspace/fontforge/inc -I/home/fred/Workspace/fontforge/build/inc
```
