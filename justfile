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
    apt install "./deb/packages/termux-clock_`just fetch-version`_{{ arch() }}.deb"

# Cross build for android targets
cross-build +ARGS:
    scripts/cross-build.bash {{ARGS}}

# Cross build for all valid android targets. equivalent to: `cross-build --all`
cross-build-all:
    scripts/cross-build.bash --all

# Build termux deb packages. Built packages are stored in: `deb/packages/`
deb +ARGS:
    scripts/deb.bash {{ARGS}}

# Build termux deb package for native target (in case you are using Termux). Equivalent to: `deb --native`
deb-native:
    scripts/deb.bash --native

# Build termux deb packages for all targets. Equivalent to: `deb --all`
deb-all:
    scripts/deb.bash --all

# Install tool/s used in build process
install-tool +ARGS:
    scripts/install-tool.bash {{ARGS}}

# Fetch package version and print it
fetch-version:
    just install-tool yq >&2
    yq eval ".package.version" Cargo.toml

# Bump version, make release branch, push it, make release tag and push it (Release workflow will create a new release from tag)
bump-version *ARGS:
    scripts/bump-version.bash {{ ARGS }}

# Install package dependencies with apt
install-termux-deps:
    pkg install $TERMUX_DEPS

# Generate termux deb packages manifests. Generated manifests are stored in: `deb/manifests/`
deb-manifest +ARGS:
    scripts/deb-manifest.bash {{ARGS}} --deps "$TERMUX_DEPS"

# Path to scripts/common.bash file
common-file-path:
    @echo "`realpath scripts/common.bash`"

# Clean cache
mod clean 'justmodules/clean.just'
