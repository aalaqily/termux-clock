# Termux Clock
[![WIP](https://img.shields.io/badge/%F0%9F%9B%A0-WIP-cyan)](#)
[![Rust](https://img.shields.io/github/actions/workflow/status/iahmadgad/termux-clock/rust.yml?label=Rust&logo=rust)](https://github.com/iahmadgad/termux-clock/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/github/actions/workflow/status/iahmadgad/termux-clock/publish.yml?label=Publish&logo=github)](https://github.com/iahmadgad/termux-clock/actions/workflows/publish.yml)
[![Lines of code](https://tokei.rs/b1/github/iahmadgad/termux-clock?category=code&label=Lines%20of%20code&style=flat)](#)
[![Latest Tag](https://img.shields.io/github/v/tag/iahmadgad/termux-clock?label=Latest%20tag&sort=semver)](https://github.com/iahmadgad/termux-clock/tags)
[![Downloads](https://img.shields.io/github/downloads/iahmadgad/termux-clock/total?label=Downloads%20(GH))](https://github.com/iahmadgad/termux-clock/releases)
[![PRs: welcome](https://img.shields.io/badge/PRs-welcome-lemon)](https://github.com/iahmadgad/termux-clock/fork)

Termux tool to set alarms & timers headlessly.

# Requirements
- [Termux](https://github.com/termux/termux-app) >= `0.118.1` _(this version solves issue termux/termux-app#3990)_
- [Termux: API](https://github.com/termux/termux-api)
- Termux packages: `termux-api`, `at`, `cronie`

# Install
> [!IMPORTANT]
> both 2 methods install `termux-api` package not the Android app, which needs to be installed manually.
> Read [Installation](https://github.com/termux/termux-api?tab=readme-ov-file#installation) on `termux/termux-api` repo to know how to install the Android app.
## Pre-built Termux packages
- Navigate to [GitHub Releases](https://github.com/iahmadgad/termux-clock/releases), and choose package with your desired version & phone architecture.
- copy deb file url & download it using `wget` or `curl`.
- Install the downloaded package with `apt`:
```sh
apt install ./<package>.deb
```
## From source
- Install dependencies
```sh
pkg install termux-api at cronie
```
- Compile and install from the latest commit on `main` branch:
```sh
cargo install --force --git https://github.com/iahmadgad/termux-clock
```
- Or from tag (recommended):
```sh
cargo install --force --git https://github.com/iahmadgad/termux-clock --tag <tag>
```
See [`cargo-install(1)`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more `cargo install` options.
# Usage
## `timer`
### `-H, --hours <HOURS>`
add hours to timer length.
### `-M, --minutes <MINUTES>`
add minutes to timer length.
### `-S, --seconds <SECONDS>`
add seconds to timer length.
### `-m, --message <MESSAGE>`
timer message.
### `-v, --vibrate`
enable vibration.
> [!NOTE]
> `-v, --vibrate` option is not available in android timers, so use it with `--termux` option.
### `-t, --termux`
set timer in termux instead of android alarm clock.
### Example
```sh
termux-clock timer -l 60 -m "Timer for 1 minute in Termux" -t
```
## `alarm`
### `-H, --hour <HOUR>`
alarm hour.
> [!NOTE]
> Hour is supposed to be `am` by default, to make an alarm at `pm` hour add `12` to hour value.
### `-M, --minutes <MINUTES>`
alarm extra minutes.
### `-d, --days <DAYS>`
days to recurr the alarm, denoted by comma-seperated numbers between double quotes, e. g. `1,2,3`, where each number corresponds to a weekday, starting from Sunday in android alarms, or Monday in termux alarms.
### `-m, --message`
alarm message.
### `-v, --vibrate`
enable vibration.
### `-t, --termux`
set alarm in termux instead of android alarm clock.
### Example
```sh
termux-clock alarm -H 6 -M 30 -d "1 2 3" -v true -m "Alarm at 06:30 am 🕡 every Sunday, Monday and Tuesday with vibration enabled 📳"
```
