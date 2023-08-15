# Contributing

This document contains guidelines and instructions on how best to contribute to
Facti.

If you have any questions, do not be afraid to reach out to us and ask!

Your best option is using [GitHub discussions][discussions] or joining the
[Matrix room][matrix-room] to chat with us directly.

[![GitHub discussions][discussions-badge]][discussions] &emsp; [![Matrix room][matrix-badge]][matrix-room]

If for some reason you feel like you want a more private discussion,
please refer to the [`AUTHORS`](AUTHORS) file for contact information.

[discussions]: https://github.com/Sharparam/facti/discussions
[matrix-room]: https://matrix.to/#/#facti:sharparam.com
[discussions-badge]: https://img.shields.io/github/discussions/Sharparam/facti?logo=github
[matrix-badge]: https://img.shields.io/matrix/facti%3Asharparam.com?logo=matrix&label=%23facti%3Asharparam.com

## Rust

If you use [VS Code][vsc] (or derivatives) with [rust-analyzer][], the included
settings file will enable automatic formatting when you save files and paste
code.

When you open the project in VS Code it will suggest to install the relevant
extensions if you haven't already. If you're using other editors or IDEs you
will have to make sure to install [rust-analyzer][] in whatever way is
relevant for your environment.

In general, just format the code however `rustfmt` wants you to format it.

If you have questions about the formatting or suggestions on possible
improvements by deviating from the defaults, please start a
[discussion][discussions] about it and we can take it further from there!

All of this is of course verified by CI when you make a [pull request][pr],
so it's always possible to discuss and resolve issues at that stage.
But the more things you verify before making your PR, the smoother it will go!

[pr]: https://github.com/Sharparam/facti/pulls

### rustfmt

Follow whatever `rustfmt` says, and format your code with it.

If you don't have it, you can add it with `rustup`:

```
rustup component add rustfmt
```

Then run it on the code:

```
cargo fmt
```

(If you're using VS Code or other supported editors this will be done for you.)

### clippy

Make sure to run `cargo clippy` and fix any issues it finds. (No need to run
`cargo check` separately.)

If you don't have clippy, you can add it with `rustup`:

```
rustup component add clippy
```

### Tests

Add tests to verify new features! And make sure to run existing tests to make
sure nothing broke with your changes!

```
cargo test
```

Nobody is expecting 100% code coverage, of course, but try to add tests that
are relevant for your changes when you can.

[vsc]: https://code.visualstudio.com/
[rust-analyzer]: https://rust-analyzer.github.io/

[rustfmt]: https://github.com/rust-lang/rustfmt#rustfmt----
[clippy]: https://github.com/rust-lang/rust-clippy#clippy

## Changelogs

Please make sure to update the relevant changelog(s) if you contribute to Facti!

They can be found in the root of each crate:

 - [`crates/lib/CHANGELOG.md`](crates/lib/CHANGELOG.md)
 - [`crates/api/CHANGELOG.md`](crates/api/CHANGELOG.md)
 - [`crates/cli/CHANGELOG.md`](crates/cli/CHANGELOG.md)

Facti follows the [keep a changelog](https://keepachangelog.com/en/1.1.0/)
specification.
