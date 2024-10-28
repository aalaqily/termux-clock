#!/usr/bin/env sh

get_arch() {
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

while getopts ":v:a:t:" option; do
    case $option in
	v)
	    version="$OPTARG"
	    ;;
	a)
	    arch="$OPTARG"
	    ;;
	t)
	    arch=`get_arch $OPTARG`
	    ;;
	\?)
	    echo "Invalid option"
	    exit
	    ;;
   esac
done

echo "termux-create-package pkg/$arch.json --pkg-version $version"
#termux-create-package pkg/$arch.json --pkg-version $version
