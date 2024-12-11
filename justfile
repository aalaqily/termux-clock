set dotenv-filename := "pkg.env"

build *FLAGS:
    cargo build {{FLAGS}}

test *FLAGS: build
    cargo test {{FLAGS}}

# Install as crate with cargo, installed binary will be stored in: `~/.cargo/bin/`
install: install-termux-deps
    cargo install --path .

# Install as termux deb package with apt, installed binary will be stored in: `/data/data/com.termux/files/usr/bin/`
install-deb: deb-native
    apt install "./target/deb/packages/termux-clock_`just fetch-version`_{{ arch() }}.deb"

# Cross build for android targets
cross-build +ARGS:
    dev/cross-build {{ARGS}}

# Cross build for all valid android targets. equivalent to: `cross-build --all`
cross-build-all:
    dev/cross-build --all

# Build termux deb packages. Built packages are stored in: `target/deb/packages/`
deb +ARGS:
    dev/deb {{ARGS}}

# Build termux deb package for native target (in case you are using Termux). Equivalent to: `deb --native`
deb-native:
    dev/deb --native

# Build termux deb packages for all targets. Equivalent to: `deb --all`
deb-all:
    dev/deb --all

# Install tool/s used in build process
install-tool +ARGS:
    dev/install-tool {{ARGS}}

# Fetch package version and print it
fetch-version:
    just install-tool yq >&2
    yq eval ".package.version" Cargo.toml

# Bump version, make release branch, push it, make release tag and push it (Release workflow will create a new release from tag)
bump-version *ARGS:
    dev/bump-version {{ ARGS }}

# Install package dependencies with apt
install-termux-deps:
    pkg install $TERMUX_DEPS

# Generate termux deb packages manifests. Generated manifests are stored in: `target/deb/manifests/`
deb-manifest +ARGS:
    dev/deb-manifest {{ARGS}} --deps "$TERMUX_DEPS"

# Path to scripts/common.bash file
common-file-path:
    @echo "`realpath dev/common.bash`"

# Build manpages. Equivalent to: `cargo build-man`
man:
    cargo build-man

# Clean cache
mod clean 'justmodules/clean.just'
