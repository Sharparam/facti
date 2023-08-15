# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

 - Proper changelog.

### Security

 - Removed transient dependency on [time (v0.1.45)][time_0.1.45]
   (via [chrono (v0.4.26)][chrono_0.4.26]).
   See [relevant GitHub issue comment][chrono_time_cve_issue] for some details
   ([CVE-2020-26235][]).

[time_0.1.45]: https://crates.io/crates/time/0.1.45
[chrono_0.4.26]: https://crates.io/crates/chrono/0.4.26
[chrono_time_cve_issue]: https://github.com/chronotope/chrono/issues/602#issuecomment-1242149249
[CVE-2020-26235]: https://cve.circl.lu/cve/CVE-2020-26235

## [0.1.0] - 2023-08-15

### Added

 - All useful API endpoints supported.

[unreleased]: https://github.com/Sharparam/facti/compare/api/v0.1.0...HEAD
[0.1.0]: https://github.com/Sharparam/facti/releases/tag/api/v0.1.0
