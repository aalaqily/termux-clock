#!/usr/bin/env bash

Help()
{
    echo "\
Usage: cross-build <arch[,...]> [-a, --all] [-h, --help]

cross build for android targets

options:
  -a, -all      build for all targets
  -h, --help    display this help message & exit

available architectures:
  aarch64  =>  aarch64-linux-android    64-bit ARM
  arm      =>  arm-linux-androideabi    32-bit ARM
  x86_64   =>  x86_64-linux-android     64-bit x86
  i686     =>  i686-linux-android       32-bit x86"
}

source "`just common-file-path`"

OPTIONS=$(getopt -o ah --long all,help -n "$0" -- "$@")
if [[ $? -ne 0 ]]; then
    echo "Error parsing options" >&2
    exit 1
fi

# Reorganize the arguments
eval set -- "$OPTIONS"

# Loop through the options
while true; do
    case "$1" in
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
    archs="$@"
fi

if [ -z "$archs" ]; then
    echo "architecture/s not specified"
    Help
    exit 1
fi

just install-tool cross
for arch in $archs; do
    if [ -z "${targets[$arch]}" ]; then
	echo "error: invalid architecture: $arch. skipping" >&2
	continue
    fi
    echo "Building for architecture: $arch..."
    cross build --target "${targets[$arch]}" --release
done
