# Changelog

All notable changes to this project will be documented in this file. The format
is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

[keep a changelog]: https://keepachangelog.com/
[semantic versioning]: https://semver.org/

## [v0.5.0] - 2024-12-13

[v0.5.0]: https://github.com/iahmadgad/termux-clock/releases/v0.5.0

### Added

- Included manpgaes in deb packages. ([#30](https://github.com/iahmadgad/termux-clock/pull/30))

### Fixed

- Start `crond` and `atd` daemons when needed. ([#28](https://github.com/iahmadgad/termux-clock/pull/28))

### Changed

- Days start from sunday in both termux and android alarms. ([#27](https://github.com/iahmadgad/termux-clock/pull/27))

## [v0.4.0] - 2024-12-6

[v0.4.0]: https://github.com/iahmadgad/termux-clock/releases/v0.4.0

### Added

- `--pm` option for alarms. ([#25](https://github.com/iahmadgad/termux-clock/pull/25))

## [v0.3.3] - 2024-12-6

[v0.3.3]: https://github.com/iahmadgad/termux-clock/releases/v0.3.3

A release to test new changes to build system

## [v0.3.2] - 2024-11-22

[v0.3.2]: https://github.com/iahmadgad/termux-clock/releases/v0.3.2

### Changed

- New build system with just. ([2babc1c](https://github.com/iahmadgad/termux-clock/commit/00770102c2b9ab6bcfa6a82296bbaef3868b6360))
- Updated CI workflows. ([0077010](https://github.com/iahmadgad/termux-clock/commit/2babc1cf956149ef081143cb23b977ebacf7cdab))

## [v0.3.1] - 2024-11-22

[v0.3.1]: https://github.com/iahmadgad/termux-clock/releases/v0.3.1

### Fixed

- Added missing dependency `cronie` in deb packages. ([#21](https://github.com/iahmadgad/termux-clock/pull/21))

## [v0.3.0] - 2024-11-15

[v0.3.0]: https://github.com/iahmadgad/termux-clock/releases/v0.3.0

### Added

- `--days` option for termux alarms.
- `--hours`, `--minutes`, `--seconds` options for timers.

### Removed
- `--length` option.

## [v0.2.0] - 2024-10-25

[v0.2.0]: https://github.com/iahmadgad/termux-clock/releases/v0.2.0

### Added

- `alarm` subcommand.
- `timer` subcommand.

### Removed
- `termux-alarm` shell script.

### Changed
- Reimplementation in Rust with extra features.

## [v0.1.0] - 2024-09-29

[v0.1.0]: https://github.com/iahmadgad/termux-clock/releases/v0.1.0

### Added

- `termux-alarm` shell script.
