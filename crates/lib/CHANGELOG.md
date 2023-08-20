# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-heading -->
## [Unreleased] <!-- next-date -->

## [0.2.1] - 2023-08-19

### Fixed

 - Fixed some failing doc tests.

## [0.2.0] - 2023-08-19

### Added

 - Proper changelog.
 - New helper function on `facti_lib::ModInfo` to construct a builder:
   `facti_lib::ModInfo::builder(...)`.
 - Can now serialize and deserialize the `Changelog` struct with serde.
 - Can now convert the `Changelog` struct to string.
   Borth "as-is" (implemented via `Display` trait) and a sorted variant
   (`Changelog::to_string_sorted`) are possible.
 - Various versions can now convert to and from other representations.
 - More documentation has been added.

### Changed

 - Restructured some code internally, does not affect the public API.
 - **BREAKING:** Renamed fields on `facti_lib::modinfo::ModPackageInfo` and
   changed their types to be more appropriate (they are now `PathBuf` instead
   of `String` and named appropriately).

### Fixed

 - Adjusted generic type args on `facti_lib::ModInfoBuilder` to avoid possible
   issue of conflicting types.

### Removed

 - `facti_lib::ModInfoBuilder::new(...)` is no longer publicly accessible,
   instead use the new `facti_lib::ModInfo::builder(...)` helper.

## [0.1.0] - 2023-08-15

### Added

 - Factorio mod changelog parser.
 - Ability to load info.json files.
 - Implementations of various data formats relevant to Factorio mods.

<!-- next-url -->
[unreleased]: https://github.com/Sharparam/facti/compare/facti-lib/v0.2.1...HEAD
[0.2.1]: https://github.com/Sharparam/facti/compare/facti-lib/v0.2.0...facti-lib/v0.2.1
[0.2.0]: https://github.com/Sharparam/facti/compare/facti-lib/v0.1.0...facti-lib/v0.2.0
[0.1.0]: https://github.com/Sharparam/facti/releases/tag/facti-lib/v0.1.0
