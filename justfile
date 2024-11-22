build *FLAGS:
    cargo build {{FLAGS}}

test *FLAGS: build
    cargo test {{FLAGS}}

install: install-termux-deps
    cargo install --path .

cross-build +ARGS:
    scripts/cross-build.bash {{ARGS}}

deb +ARGS:
    scripts/deb.bash {{ARGS}}

deb-native:
    scripts/deb.bash --native

deb-all:
    scripts/deb.bash --all

install-tool +ARGS:
    scripts/install-tool.bash {{ARGS}}

fetch-version:
    just install-tool rust-script
    scripts/fetch-version.rs

install-termux-deps:
    pkg install termux-api at cronie

deb-manifest +ARGS:
    scripts/deb-manifest.bash {{ARGS}}

# Path to scripts/common.bash file
common-file-path:
    echo "`realpath scripts/common.bash`"

mod clean 'justmodules/clean.just'
