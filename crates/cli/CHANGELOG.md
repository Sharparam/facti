# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-heading -->
## [Unreleased] <!-- next-date -->

## [0.2.3] - 2023-08-20

### Fixed

 - Wrong path to changelog in release workflow.

## [0.2.2] - 2023-08-20

### Fixed

 - More fixes to release workflow.

## [0.2.1] - 2023-08-20

### Fixed

 - Build issue in release workflow, nothing related to the CLI itself.

## [0.2.0] - 2023-08-20

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
 - Added logging to file:
    - A log file will be created in the log directory which will have more
      detailed logs written to it.
    - A log file in JSON format will also be created in the log directory which
      contains *even more detailed* logs in it, meant for debugging or
      troubleshooting and not for direct human consumption.
 - New CLI parameter to set game base URL: `--game-base-url`.
 - New environment variable to set game base URL: `FACTI_GAME_BASE_URL`.
 - New config file setting to set game base URL: `game-base-url` under the
   `factorio-api` section.

### Changed

 - Can now use either `kebab-case` or `snake_case` for config file keys.
 - `--api-key` can now only be specified at the base of the `facti` command,
   it is no longer accepted by the subcommands.
 - Refined logging to the terminal (STDERR) to be more readable.
 - **BREAKING:** The `--base-url` argument has been renamed to `--portal-base-url`.
 - **BREAKING:** The `FACTI_BASE_URL` environment variable has been renamed
   to `FACTI_PORTAL_BASE_URL`.
 - **BREAKING:** The `base-url` setting under `factorio-api` in the config file
   has been renamed to `portal-base-url`.

## [0.1.0] - 2023-08-15

### Added

 - Basic commands to interact with mod portal.
 - Ability to generate shell completion files.
 - Configuration system.

<!-- next-url -->
[unreleased]: https://github.com/Sharparam/facti/compare/facti/v0.2.3...HEAD
[0.2.3]: https://github.com/Sharparam/facti/compare/facti/v0.2.2...facti/v0.2.3
[0.2.2]: https://github.com/Sharparam/facti/compare/facti/v0.2.1...facti/v0.2.2
[0.2.1]: https://github.com/Sharparam/facti/compare/facti/v0.2.0...facti/v0.2.1
[0.2.0]: https://github.com/Sharparam/facti/compare/facti/v0.1.0...facti/v0.2.0
[0.1.0]: https://github.com/Sharparam/facti/releases/tag/facti/v0.1.0
