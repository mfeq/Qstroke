export RUST_LOG := debug
export RUST_BACKTRACE := 1

DEBUGARGS := $(if $(DEBUG),,"--release")

FONTFORGE := $(if $(FONTFORGE),"fontforge","")

.PHONY: all
all:
	RUSTFLAGS="${RUSTFLAGS}" cargo build $(DEBUGARGS) --features $(FONTFORGE)

.PHONY: fmt
fmt:
	find src -type f -iname '*.rs' | parallel --bar RUST_LOG=error rustfmt {}
