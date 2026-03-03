# Use the latest Arch Linux base image
FROM archlinux:latest

# Update system and install base-devel group
RUN pacman -Syu --noconfirm base-devel

# Install other necessary packages
RUN pacman -S --noconfirm git python cross
