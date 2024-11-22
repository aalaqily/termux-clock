#!/usr/bin/env bash

Help()
{
    echo "\
Usage: install-tool <tool[,...]> [-a, --all] [-h, --help]

install required tools

options:
  -a, -all                 install all tools
  -h, --help               display this help message & exit

available tools:
  cross                    for cross building for android targets
  termux-create-package    for creating termux deb packages
  jq                       for generating termux deb packages manifests
  rust-script              for executing rust scripts"
}

declare -A install_commands
install_commands["cross"]="cargo install cross --git https://github.com/cross-rs/cross"
install_commands["termux-create-package"]="pip3 install git+https://github.com/termux/termux-create-package"
install_commands["jq"]="sudo apt install jq"
install_commands["rust-script"]="cargo install rust-script"

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
            exit
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
    tools="${!install_commands[@]}"
elif [ "$#" -eq 0 ]; then
    echo "tool/s not specified"
    Help
    exit 1
else
    tools="$@"
fi

for tool in $tools; do
    if [[ -z "${install_commands[$tool]}" ]]; then
        echo "Unknown tool: $tool"
        exit 1
    elif ! command -v "$tool" > /dev/null 2>&1; then
        echo "$tool not found, installing..."
        eval "${install_commands[$tool]}"
    else
        echo "$tool already installed, skipping installation."
    fi
done
