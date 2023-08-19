# facti-api &ensp; [![crates.io][cratesio-badge]][cratesio] [![docs.rs][docsrs-badge]][docsrs] [![Build status][build-badge]][build] [![Audit status][audit-badge]][audit]

Rust crate for interacting with the various Factorio APIs.

## Contributing

[![GitHub discussions][discussions-badge]][discussions] &emsp; [![Matrix room][matrix-badge]][matrix-room]

Contributors are very welcome!

If you want to discuss the project you can do so in [the discussions on GitHub][discussions] or join the [Matrix room][matrix-room].

## APIs supported

All APIs that use an API key or do not require authentication will be
supported. If there are any that are not currently supported, please
[create an issue][new-issue] to request addition or [submit a pull request][new-pr]
if you feel like adding it yourself!

## Async or blocking

You can use either async or blocking methods with this crate, they can be
enabled independent of each other.

 - To enable async support, enable the `async` feature (this is part of the
   default enabled features).
 - For blocking support, enable the `blocking` feature.

For more details on the different APIs, consult the [documentation][docsrs].

## License

Copyright © 2023 by [Adam Hellberg][sharparam].

This Source Code Form is subject to the terms of the
[Mozilla Public License, v. 2.0][mpl-2.0].
If a copy of the MPL was not distributed with this file,
You can obtain one at <http://mozilla.org/MPL/2.0/>.

[sharparam]: https://sharparam.com
[mpl-2.0]: http://mozilla.org/MPL/2.0/

[cratesio]: https://crates.io/crates/facti-api
[librs]: https://lib.rs/crates/facti-api
[docsrs]: https://docs.rs/facti-api
[cratesio-badge]: https://img.shields.io/crates/v/facti-api?logo=rust
[docsrs-badge]: https://img.shields.io/docsrs/facti-api/latest?logo=docsdotrs

[build]: https://github.com/Sharparam/facti/actions/workflows/test.yml?query=branch%3Amain
[audit]: https://github.com/Sharparam/facti/actions/workflows/audit.yml?query=branch%3Amain
[build-badge]: https://img.shields.io/github/actions/workflow/status/Sharparam/facti/test.yml?logo=github
[audit-badge]: https://img.shields.io/github/actions/workflow/status/Sharparam/facti/audit.yml?logo=github&label=audit

[discussions]: https://github.com/Sharparam/facti/discussions
[matrix-room]: https://matrix.to/#/#facti:sharparam.com
[discussions-badge]: https://img.shields.io/github/discussions/Sharparam/facti?logo=github
[matrix-badge]: https://img.shields.io/matrix/facti%3Asharparam.com?logo=matrix&label=%23facti%3Asharparam.com
[new-issue]: https://github.com/Sharparam/facti/issues/new
[new-pr]: https://github.com/Sharparam/facti/pull/new
