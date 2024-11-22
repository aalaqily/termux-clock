#!/usr/bin/env bash

NATIVE_TARGET="`rustc --version --verbose | grep 'host' | sed 's/host: //g'`"

NATIVE_ARCH="`uname -m`"

declare -A targets
targets["aarch64"]="aarch64-linux-android"
targets["arm"]="arm-linux-androideabi"
targets["x86_64"]="x86_64-linux-android"
targets["i686"]="i686-linux-android"

VALID_TARGETS="${targets[@]}"
