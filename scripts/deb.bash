#!/usr/bin/env bash

Help()
{
    echo "\
Usage: deb <arch[,...]> [-n, --native] [-a, --all] [-h, --help]

build termux deb packages

options:
  -n, --native    build for native target
  -a, --all       build for all targets
  -h, --help      display this help message & exit

available architectures:
  aarch64  =>  aarch64-linux-android    64-bit ARM
  arm      =>  arm-linux-androideabi    32-bit ARM
  x86_64   =>  x86_64-linux-android     64-bit x86 
  i686     =>  i686-linux-android       32-bit x86"
}

source "`just common-file-path`"

OPTIONS=$(getopt -o nah --long native,all,help -n "$0" -- "$@")
if [[ $? -ne 0 ]]; then
    echo "Error parsing options" >&2
    exit 1
fi

# Reorganize the arguments
eval set -- "$OPTIONS"

# Loop through the options
while true; do
    case "$1" in
        -n|--native)
            native=true
            shift
            ;;
        -a|--all)
            all=true
            shift
            ;;
        -h|--help)
            Help
            shift
            exit
            ;;
        --)
            shift
            break
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [ -n "$all" ]; then
    archs="aarch64 arm x86_64 i686"
else
    for arch in $@; do
	if [[ "$archs" == *"$arch"* ]]; then
	    continue
	fi
	archs+=" $arch"
    done
fi

if [ -n "$native" ]; then
    if [[ "$VALID_TARGETS" == *"$NATIVE_TARGET"* ]]; then
	if [[ "$archs" != *"$NATIVE_ARCH"* ]]; then
	    archs+=" $NATIVE_ARCH"
	fi
    else
	echo "error: native target: $NATIVE_TARGET is not valid. skipping" >&2
    fi
fi

if [ -z "$archs" ]; then
    echo "architecture/s not specified" >&2
    Help
    exit 1
fi

just install-tool termux-create-package
for arch in $archs; do
    if [[ "${targets[$arch]}" == "$NATIVE_TARGET" ]]; then
	just build --release
    else
	just cross-build $arch
    fi
    just deb-manifest $arch
    echo "Building termux deb package for architecture: $arch..."
    termux-create-package "deb/manifests/${arch}.json"
done
