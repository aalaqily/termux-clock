#!/usr/bin/env bash

Help()
{
    echo "\
Usage: install-tool <tool[,...]> [-a, --all] [-h, --help]

install required tools (for amd64 linux machines)

options:
  -a, --all                install all tools
  -h, --help               display this help message & exit

available tools:
  cross                    for cross building for android targets
  termux-create-package    for creating termux deb packages
  yq                       for generating termux deb packages manifests and reading version from Cargo.toml"
}

# All required tools
all_tools="cross termux-create-package yq"

# Functions to check for tools if installed with required versions
check_cross() { command -v cross > /dev/null; }
check_termux-create-package() { command -v termux-create-package > /dev/null; }
check_yq() { command -v yq > /dev/null && yq --version | grep -qE 'version v[4]'; }

# Functions to install tools
install_cross() { cargo install cross --git https://github.com/cross-rs/cross; }
install_termux-create-package() { pip3 install git+https://github.com/termux/termux-create-package; }
install_yq() {
    wget https://github.com/mikefarah/yq/releases/download/v4.44.5/yq_linux_amd64.tar.gz -O - | tar xz
    mv yq_linux_amd64 /usr/bin/yq
    ./install-man-page.sh
    rm install-man-page.sh yq.1
}

### Executing part

# Source common file
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
    tools="${all_tools}"
elif [ "$#" -eq 0 ]; then
    echo "tool/s not specified"
    Help
    exit 1
else
    tools="$@"
fi

for tool in $tools; do
    if [[ "$all_tools" != *"$tool"* ]]; then
        echo -e "  ${BOLD}${YELLOW}skipping${NORMAL}\t$tool"
	continue
    elif ! check_$tool; then
        echo -e  "${BOLD}${CYAN}installing${NORMAL}\t$tool"
        install_$tool
    fi
    echo -e " ${BOLD}${GREEN}installed${NORMAL}\t$tool"
done
