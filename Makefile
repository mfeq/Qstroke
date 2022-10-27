SHELL := /bin/bash
.PHONY: all fmt man

export RUST_LOG := debug
export RUST_BACKTRACE := 1

DEBUGARGS := $(if $(DEBUG),,"--release")

ifdef FONTFORGE
RUSTFLAGS += -L/opt/lib -L/usr/local/lib
endif
all:
	set -x; \
	[ $$FONTFORGE = "y" ] && FONTFORGE_FLAG="--features fontforge" && NIGHTLY_FLAG="+nightly"; \
	RUSTFLAGS="${RUSTFLAGS}" cargo $$NIGHTLY_FLAG build $(DEBUGARGS) $$FONTFORGE_FLAG

fmt:
	find src -type f -iname '*.rs' | parallel --bar RUST_LOG=error rustfmt {}

man:
	rm -f /tmp/MFEKstroke*
	LD_LIBRARY_PATH=/opt/lib help2man -N 'target/debug/MFEKstroke CWS' --no-discard-stderr | tail -n +5 | head -n -3 > /tmp/MFEKstrokeCWS
	LD_LIBRARY_PATH=/opt/lib help2man -N 'target/debug/MFEKstroke DASH' --no-discard-stderr | tail -n +5 | head -n -3 > /tmp/MFEKstrokeDASH
	LD_LIBRARY_PATH=/opt/lib help2man -N 'target/debug/MFEKstroke NIB' --no-discard-stderr | tail -n +5 | head -n -3 > /tmp/MFEKstrokeNIB
	LD_LIBRARY_PATH=/opt/lib help2man -N 'target/debug/MFEKstroke PAP' --no-discard-stderr | tail -n +5 | head -n -3 > /tmp/MFEKstrokePAP
	LD_LIBRARY_PATH=/opt/lib help2man -N 'target/debug/MFEKstroke VWS' --no-discard-stderr | tail -n +5 | head -n -1 > /tmp/MFEKstrokeVWS
	LD_LIBRARY_PATH=/opt/lib help2man -N 'target/debug/MFEKstroke' --version-string='git-rev-'`git rev-parse --short HEAD` --no-discard-stderr > /tmp/MFEKstroke
	perl -pi -e 's/MFEKSTROKE/MFEKstroke/g; s/:"$$/"/g and s/\.SS/.SH/;' /tmp/MFEKstroke
	cat /tmp/MFEKstroke /tmp/MFEKstrokeCWS /tmp/MFEKstrokeDASH /tmp/MFEKstrokeNIB /tmp/MFEKstrokePAP /tmp/MFEKstrokeVWS > /tmp/MFEKstroke-all
	gawk -i inplace -v INPLACE_SUFFIX=.bak '{ if(NR>7) { gsub(/DESCRIPTION/, "SUBCOMMAND") } }; { print }' /tmp/MFEKstroke-all
	perl -pi -e '!$$subbed and s/\.SH SUBCOMMAND/.SS MFEKstroke-CWS/ and $$subbed++' /tmp/MFEKstroke-all
	perl -pi -e '!$$subbed and s/\.SH SUBCOMMAND/.SS MFEKstroke-DASH/ and $$subbed++' /tmp/MFEKstroke-all
	perl -pi -e '!$$subbed and s/\.SH SUBCOMMAND/.SS MFEKstroke-NIB/ and $$subbed++' /tmp/MFEKstroke-all
	perl -pi -e '!$$subbed and s/\.SH SUBCOMMAND/.SS MFEKstroke-PAP/ and $$subbed++' /tmp/MFEKstroke-all
	perl -pi -e '!$$subbed and s/\.SH SUBCOMMAND/.SS MFEKstroke-VWS/ and $$subbed++' /tmp/MFEKstroke-all
	perl -pi -e 's/:"$$/"/g; s/\\fB\\-1]\\fR/-1]/g' /tmp/MFEKstroke-all
	groff -mandoc -Thtml /tmp/MFEKstroke-all > /tmp/MFEKstroke-all.html
	mv /tmp/MFEKstroke-all.html docs/index.html
	mkdir -p docs/man
	mv /tmp/MFEKstroke-all docs/man/MFEKstroke.1
