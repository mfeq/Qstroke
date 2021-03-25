export RUSTFLAGS :=
export RUST_LOG := debug
export RUST_BACKTRACE := 1

DEBUGARGS := $(if $(DEBUG),,"--release")

FONTFORGE := $(if $(FONTFORGE),"fontforge","")

.PHONY: all
all:
	cargo build $(DEBUGARGS) --features $(FONTFORGE)
