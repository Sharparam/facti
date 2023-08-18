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
 - New subcommand: `facti new` to create a new mod.
   Use `facti new --help` or `facti help new` for usage information.
 - New subcommand: `facti noop` to do nothing.
   Only enabled in debug builds, useful for testing CLI flags without performing
   any operations.
 - Can now configure default values for some of the fields in `info.json` in
   the config file. These are used when creating a new mod with `facti new`.

   The following can now be configured under the `mod-defaults` section:
    * `author`
    * `contact`
    * `factorio-version`
 - New command: `facti changelog convert` to convert changelogs between various
   formats.

### Changed

 - Can now use either `kebab-case` or `snake_case` for config file keys.
 - `--api-key` can now only be specified at the base of the `facti` command,
   it is no longer accepted by the subcommands.

## [0.1.0] - 2023-08-15

### Added

 - Basic commands to interact with mod portal.
 - Ability to generate shell completion files.
 - Configuration system.

[unreleased]: https://github.com/Sharparam/facti/compare/cli/v0.1.0...HEAD
[0.1.0]: https://github.com/Sharparam/facti/releases/tag/cli/v0.1.0
