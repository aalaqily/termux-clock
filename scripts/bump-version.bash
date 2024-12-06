#!/usr/bin/env bash

Help()
{
    echo "\
Usage: bump-version [[-M, --force-major | -I, --ignore-major] | [-m, --force-minor | -i, --ignore-minor ] | -p, --force-patch] [-p, --print-only] [-h, --help]

bump version, make release branch, push it, make release tag and push it (Release workflow will create a new release from tag)

options:
  -M, --force-major    force bumping major version even if there is no breaking change
  -I, --ignore-major   ignore bumping major version even if there is breaking change
  -m, --force-minor    force bumping minor version even if there is no new feature
  -i, --ignore-minor   ignore bumping minor version even if there is new feature
  -p, --force-patch    force bumping patch version even if there is a breaking change or a feature
  -o, --print-only     just print new version & exit
  -h, --help           display this help message & exit"
}

# Source common file
source "`just common-file-path`"

# Install required tool
just install-tool toml-cli

OPTIONS=$(getopt -o MImipoh --long force-major,ignore-major,force-minor,ignore-minor,force-patch,print-only,help -n "$0" -- "$@")
if [[ $? -ne 0 ]]; then
    echo "Error parsing options" >&2
    exit 1
fi

# Reorganize the arguments
eval set -- "$OPTIONS"

# Loop through the options
while true; do
    case "$1" in
        -M|--force-major)
            force_major=true
            shift
            ;;
        -I|--ignore-major)
            ignore_major=true
            shift
            ;;
	-m|--force-minor)
            force_minor=true
            shift
            ;;
        -i|--ignore-minor)
            ignore_minor=true
            shift
            ;;
	-p|--force-patch)
            force_patch=true
            shift
            ;;
	-o|--print-only)
	    print_only=true
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

# Parse current version
echo -e "${BOLD}${CYAN}parsing current version${NORMAL}"
current_version="`just fetch-version`"
IFS="." read -r current_major current_minor current_patch <<< "$current_version"

# Check for Breaking changes
echo -e "${BOLD}${CYAN}checking for breaking changes${NORMAL}"
if (git log v$current_version..HEAD --pretty=format:"%b" | grep -q "BREAKING CHANGE:"); then
    major=true
fi

# Check for new features
echo -e "${BOLD}${CYAN}checking for new features${NORMAL}"
if (git log v$current_version..HEAD --pretty=format:"%s" | grep -qE "feat\(?[^)]*\)?:"); then
    minor=true
fi

# Generate new version
echo -e "${BOLD}${CYAN}generating new version${NORMAL}"
if [ -n "$force_major" ] || ([ -n "$major" ] && ! ([ -n "$ignore_major" ] || [ -n "$force_minor" ] || [ -n "$force_patch" ])); then
    new_version="$((current_major+1)).0.0"
elif [ -n "$force_minor" ] || ([ -n "$minor" ] && ! ([ -n "$ignore_minor" ] || [ -n "$force_patch" ])); then
    new_version="${current_major}.$((current_minor+1)).0"
else
    new_version="${current_major}.${current_minor}.$((current_patch+1))"
fi

# Print new version and exit if --print-only option is set
if [ -n "$print_only" ]; then
    echo "$new_version"
    exit
fi

# Make a release branch
echo -e "${BOLD}${CYAN}making a release branch${NORMAL}"
git checkout -b release/v$new_version

# Change version in Cargo.toml
echo -e "${BOLD}${CYAN}changing version in Cargo.toml${NORMAL}"
toml set Cargo.toml package.version $new_version > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

# Execute cargo check to update version in Cargo.lock
echo -e "${BOLD}${CYAN}executing cargo check to update version in Cargo.lock${NORMAL}"
cargo check

# Commit changes
echo -e "${BOLD}${CYAN}commiting changes${NORMAL}"
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $new_version"

# Push branch into remote repo
echo -e "${BOLD}${CYAN}pushing branch into remote repo${NORMAL}"
git push --set-upstream origin release/v$new_version

# Open release pull request
echo -e "${BOLD}${CYAN}opening release pull request${NORMAL}"
pr="$(gh pr create --title "release: v$new_version" --fill --label release --assignee @me --head release/v$new_version --base main)"

# Merge opened pull request
echo -e "${BOLD}${CYAN}merging opened pull request${NORMAL}"
gh pr merge $pr --merge

# Checkout main branch and pull merge commit
echo -e "${BOLD}${CYAN}checking out main branch and pulling merge commit${NORMAL}"
git checkout main
git pull

# Tag latest commit and push tag
echo -e "${BOLD}${CYAN}tagging latest commit and push tag${NORMAL}"
git tag -a v$new_version -m v$new_version
git push --tags
