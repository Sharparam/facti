# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

 - Proper changelog.
 - Can now set API key from STDIN with `--api-key-stdin`.
 - Can now load API key from file with `--api-key-file`.
 - Can now specify path to file containing API key in config file
   (`api-key-file` under the `factorio-api` section).

## [0.1.0] - 2023-08-15

### Added

 - Basic commands to interact with mod portal.
 - Ability to generate shell completion files.
 - Configuration system.

[unreleased]: https://github.com/Sharparam/facti/compare/cli/v0.1.0...HEAD
[0.1.0]: https://github.com/Sharparam/facti/releases/tag/cli/v0.1.0
