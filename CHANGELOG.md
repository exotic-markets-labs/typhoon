# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.0] - 2026-02-09

### Added

- CLI (`new`, `add` subcommands) ([#305](https://github.com/exotic-markets-labs/typhoon/pull/305)).
- `typhoon-traits` crate ([#315](https://github.com/exotic-markets-labs/typhoon/pull/315)).
- `const` discriminator calculation ([#322](https://github.com/exotic-markets-labs/typhoon/pull/322)).
- Reworked error logging ([#317](https://github.com/exotic-markets-labs/typhoon/pull/317)).
- Initial book chapters ([#323](https://github.com/exotic-markets-labs/typhoon/pull/323)).

### Changed

- Codama version updated ([#327](https://github.com/exotic-markets-labs/typhoon/pull/327)).
- Create account method refactored ([#320](https://github.com/exotic-markets-labs/typhoon/pull/320)).
- Package descriptions updated ([#319](https://github.com/exotic-markets-labs/typhoon/pull/319)).
- Context CPI optimized.
- MSRV bumped to 1.87 ([#318](https://github.com/exotic-markets-labs/typhoon/pull/318)).
- Pinocchio bumped to 0.10.0 ([#316](https://github.com/exotic-markets-labs/typhoon/pull/316)).
- Bytes writing: use `copy_nonoverlapping` instead of classic write ([#309](https://github.com/exotic-markets-labs/typhoon/pull/309)).
- More efficient bytes writing ([#314](https://github.com/exotic-markets-labs/typhoon/pull/314)).
- Benchmark table headers updated to reflect correct naming ([#312](https://github.com/exotic-markets-labs/typhoon/pull/312)).

### Fixed

- Unnecessary check in `Mut` removed ([#326](https://github.com/exotic-markets-labs/typhoon/pull/326)).
- Seed in `escrow` example ([#328](https://github.com/exotic-markets-labs/typhoon/pull/328)).
- Seeds and seeded usability ([#324](https://github.com/exotic-markets-labs/typhoon/pull/324)).
- CI workflow ([#330](https://github.com/exotic-markets-labs/typhoon/pull/330)).
- cargo-audit ([#329](https://github.com/exotic-markets-labs/typhoon/pull/329)).
- Macro error message typos ([#310](https://github.com/exotic-markets-labs/typhoon/pull/310)).

## [0.1.0] - 2025-12-16

### Added

- Initial release.

[Unreleased]: https://github.com/exotic-markets-labs/typhoon/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/exotic-markets-labs/typhoon/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/exotic-markets-labs/typhoon/releases/tag/v0.1.0
