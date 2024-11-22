set dotenv-filename := "pkg.env"

build *FLAGS:
    cargo build {{FLAGS}}

test *FLAGS: build
    cargo test {{FLAGS}}

install: install-termux-deps
    cargo install --path .

install-deb: deb-native
    apt install "./deb/packages/termux-clock_`just fetch-version`_{{ arch() }}.deb"
# Cross build for android targets
cross-build +ARGS:
    scripts/cross-build.bash {{ARGS}}

cross-build-all:
    scripts/cross-build.bash --all

# Build termux deb packages
deb +ARGS:
    scripts/deb.bash {{ARGS}}

deb-native:
    scripts/deb.bash --native

deb-all:
    scripts/deb.bash --all

install-tool +ARGS:
    scripts/install-tool.bash {{ARGS}}

# Fetch package version and print it
fetch-version:
    just install-tool rust-script
    scripts/fetch-version.rs

install-termux-deps:
    pkg install $TERMUX_DEPS

# Generate termux deb packages manifests
deb-manifest +ARGS:
    scripts/deb-manifest.bash {{ARGS}} --deps "$TERMUX_DEPS"

# Path to scripts/common.bash file
common-file-path:
    echo "`realpath scripts/common.bash`"

mod clean 'justmodules/clean.just'
