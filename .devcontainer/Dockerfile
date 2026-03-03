# Use the latest Arch Linux base image
FROM archlinux:latest

# Initialize pacman keys and update system with base-devel group
RUN pacman-key --init && \
    pacman-key --populate archlinux && \
    pacman -Syu --noconfirm base-devel git openssh python python-pip cross just yq && \
    ssh-keygen -A

# Enable and start SSH
RUN systemctl enable sshd

# Install rust toolchain
RUN rustup default stable

# Install ctoml
RUN cargo install ctoml -f

# Make Python venv
RUN python -m venv .python

# Install termux-create-package
RUN .python/bin/pip install git+https://github.com/termux/termux-create-package