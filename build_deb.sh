#!/usr/bin/env sh

while getopts ":v:a:" option; do
    case $option in
    v)
    version="$OPTARG"
    ;;
    a)
    arch="$OPTARG"
    ;;
    \?)
    echo "Invalid option"
    exit
    ;;
   esac
done

arch_alias() {
    case $1 in
    "aarch64-linux-android")
    echo "aarch64"
    ;;
    "i686-linux-android")
    echo "i686"
    ;;
    "x86_64-linux-android")
    echo "x86_64"
    ;;
    "arm-linux-androideabi")
    echo "arm"
    ;;
    ?)
    echo "Invalid architecture name"
    ;;
    esac
}


echo "termux-create-package pkg/`arch_alias $arch`.json --pkg-version $version"
termux-create-package pkg/`arch_alias $arch`.json --pkg-version $version
