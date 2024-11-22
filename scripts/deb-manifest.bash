#!/usr/bin/env bash

Help()
{
    echo "\
Usage: deb-manifest <arch[,...]> [-n, --native] [-a, --all] [-h, --help]

generate termux deb packages manifests 

options:
  -n, --native    build natively with cargo (termux only)
  -a, -all        build for all targets
  -h, --help      display this help message & exit

available targets:
  aarch64         64-bit ARM
  arm             32-bit ARM
  i686            32-bit x86
  x86_64          64-bit x86"
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

! [ -d deb/manifests ] && mkdir -p deb/manifests

just install-tool jq
for arch in $archs; do
    if [ -z "${targets[$arch]}" ]; then
        echo "error: invalid architecture: $arch. skipping" >&2
        continue
    fi
    echo "Building deb manifest for arch: $arch"
    jq ".control.Architecture = \"${arch}\" |\
    .data_files.\"bin/termux-clock\".source = \"target/$([[ "${targets[$arch]}" != "$NATIVE_TARGET" ]] && echo "${targets[$arch]}/")release/termux-clock\" |\
    .control.Version = \"`just fetch-version`\"" pkg.json > "deb/manifests/${arch}.json"
done
