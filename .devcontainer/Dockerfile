# Use the latest Arch Linux base image
FROM archlinux:latest

# Update system and install base-devel group
RUN pacman -Syu --noconfirm

# Install other necessary packages
RUN pacman -S --noconfirm git python python-pip cross base-devel just yq

# Install rust toolchain
RUN rustup default stable

# Install ctoml
RUN cargo install ctoml -f

# Make Python venv
RUN python -m venv .python

# Activate Python venv
RUN source .python/bin/activate

# Install termux-create-package
RUN pip3 install git+https://github.com/termux/termux-create-package