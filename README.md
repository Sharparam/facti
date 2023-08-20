# Facti &ensp; [![crates.io][cratesio-badge]][cratesio] [![docs.rs][docsrs-badge]][docsrs] [![Build status][build-badge]][build] [![Audit status][audit-badge]][audit]

A CLI tool to interact with [Factorio][factorio] [mods][factorio-mods] both locally and via the [API][factorio-api] and aid in mod development.

> [!IMPORTANT]
>
> Facti is in its early stages and very much a work in progress and highly
> experimental.
>
> Any commands and APIs are subject to change while it's still in pre-release.

Quick links: [Installation](#installation) &mdash; [Building](#building) &mdash; [Usage](#usage) &mdash; [Configuration](#configuration)

## Contributing

[![GitHub discussions][discussions-badge]][discussions] &emsp; [![Matrix room][matrix-badge]][matrix-room]

Contributors are very welcome!

If you want to discuss the project you can do so in [the discussions on GitHub][discussions] or join the [Matrix room][matrix-room].

## Installation

### Cargo

You can install Facti using [`cargo binstall`][cargo-binstall]:

```sh
cargo binstall facti
```

This will fetch and install binaries from [the latest release of Facti][latest-release]
automatically.

If you don't want to use `binstall` you can also use regular [`cargo install`][cargo-install]:

```sh
cargo install facti
```

Which will download, compile, and install Facti from [crates.io][cratesio].

[cargo-binstall]: https://github.com/cargo-bins/cargo-binstall
[cargo-install]: https://doc.rust-lang.org/cargo/commands/cargo-install.html

<!--
### Arch Linux

Facti is available on the [AUR][aur] in different versions:

 * **[`facti`][aur-facti]:** *(Recommended)* Builds from source using the latest stable release.
 * **[`facti-bin`][aur-facti-bin]:** Installs the [latest stable pre-built binaries][latest-release].
 * **[`facti-git`][aur-facti-git]:** Builds from source using the latest commit on the [`main` branch][main].

Use your favourite AUR helper to install it, for example:

#### [aurutils][]

```sh
aur sync facti
sudo pacman -Syu facti
```

#### [paru][]

```sh
paru facti
```

#### [yay][]

```sh
yay facti
```

[aur]: https://aur.archlinux.org
[aur-facti]: https://aur.archlinux.org/packages/facti
[aur-facti-bin]: https://aur.archlinux.org/packages/facti-bin
[aur-facti-git]: https://aur.archlinux.org/packages/facti-git
[paru]: https://github.com/morganamilo/paru#readme
[yay]: https://github.com/Jguer/yay#readme
[aurutils]: https://github.com/AladW/aurutils#readme
-->

### Pre-built binaries

If all else fails, you can manually download and use pre-built binaries from [the latest release of Facti][latest-release].

## Building

Facti is written in [Rust][], so you'll have to [install Rust][rust-install] to build it.

After you have it set up, it's as simple as these steps:

```sh
git clone https://github.com/Sharparam/facti.git
cd facti

cargo build --workspace --release
# Or if you want all available features:
cargo build --workspace --release --all-features

# If you want to build just the facti binary:
cargo build --package facti --release

# The finished binary will be in target/release where you can run it directly:
./target/release/facti --help

# Or copy it somewhere like your local bin directory:
cp target/release/facti ~/.local/bin
```

### Running tests

Facti has several tests that can be run with [`cargo test`][cargo-test]:

```sh
cargo test --workspace --all-features
```

[cargo-test]: https://doc.rust-lang.org/cargo/commands/cargo-test.html

### Features

Facti has a few optional features that can be enabled when building, these are:

 * **`ron`:** Enables support for the [RON][] format when converting between formats.

   E.g. `facti changelog convert --to ron changelog.txt` to convert a changelog to [RON][].

 * **`sexpr`:** Enables support for the [S-expression][sexpr] format when converting between formats.

   E.g. `facti changelog convert --to sexpr changelog.txt` to convert a changelog to [S-expressions][sexpr].

 * **`yaml`:** Enables support for the [YAML][] format when converting between formats.

   E.g. `facti changelog convert --to yaml changelog.txt` to convert a changelog to [YAML][].

[ron]: https://github.com/ron-rs/ron#readme
[sexpr]: https://en.wikipedia.org/wiki/S-expression
[yaml]: https://yaml.org

### Manpages

You can generate manpages for Facti using the `man` task in `xtask`:

```sh
cargo xtask man
```

Currently this generates the manpages for the *debug* build, so you'll probably
want to instead run the CI version of `xtask`:

```sh
cargo xtask-ci man
```

The finished manpages will be in `target/assets/man`.

We're working on a nicer way to work with xtask that will not require two
different aliases.

### Completions

Generating shell completions is done with the `facti` binary itself,
and supports completions for [Bash][], [Zsh][], [fish][], [Elvish][], and [PowerShell][].

```sh
facti completion bash
facti completion zsh
facti completion fish
facti completion elvish
facti completion powershell
```

[bash]: https://www.gnu.org/software/bash
[zsh]: https://www.zsh.org
[fish]: https://fishshell.com
[elvish]: https://elv.sh
[powershell]: https://docs.microsoft.com/en-us/powershell

## Usage

For more help on the CLI commands, you can run `facti help <command>`
or `facti <command> --help`.

> [!WARNING]
>
> Commands outlined here are still experimental and subject to change.
> Additionally, some commands may not have been implemented yet.

### Expected mod layout

This tool expects your mod to be organized a certain way, as shown by the diagram below.

(The `.git` folder is just to show where the root of the Git repo is.)

```
my-mod/
├── .git/
└── src/
    ├── locale/
    │   └── en/
    │       └── mod.cfg
    ├── info.json
    ├── changelog.txt
    ├── control.lua
    ├── data.lua
    └── thumbnail.png
```

### Create new mod

Facti can bootstrap a new mod for you, placing some placeholder files and
setting up the expected folder structure:

```sh
facti new [mod-name]
```

When invoked without a name, it will set it up in the current directory,
if it is empty, using the directory name as the mod name.

### Packaging

You can use facti to package your mod for distribution to the mod portal
(or elsewhere) by using `facti pack`:

```sh
facti pack
```

If this command is used inside a Git repo, it will check to make sure the
project layout matches the one describes under [Expected mod layout](#expected-mod-layout).

If the current directory is not a Git repo, but contains an `info.json` file,
it will treat that as the mod directory.

To override the mod directory, pass it as an argument to `pack`:

```sh
facti pack cool/path/to/mod
```

Passing an explicit directory will disable Git repository detection and fail
if the specified directory does not contain an `info.json` file.

### Mod Portal

You can interact with the [Factorio mod portal][factorio-mods] via facti by using the `facti portal` command.

Here are some examples:

```sh
# Search for mods that match the name "cybersyn-combinator"
facti portal search cybersyn-combinator

# Show information about the cybersyn-combinator mod
facti portal show cybersyn-combinator

# Show more detailed information about the cybersyn-combinator mod
facti portal show --full cybersyn-combinator

# By default, deprecated mods are excluded from search
# if you want to show them you have to specify the --deprecated flag
facti portal search --deprecated

# You can enable JSON output by supplying the --json flag.
# In a non-interactive context, this is the default
# and can be negated with --no-json
facti portal --json show --full cybersyn-combinator

# Checks if all listed mods are compatible with each other.
# Currently this just makes sure none of the mods have each other listed
# as "incompatible"
facti portal check cybersyn-combinator cybersyn

# By default the above command will check against the latest version of each
# mod, to check a specific version you can include a version requirement
facti portal check cybersyn-combinator@0.6.0 cybersyn@1.0.2
```

## Configuration

Some commands like uploading mod packages to the mod portal require the use
of an API key.

To obtain this, you must generate an API key on your [Factorio profile page][factorio-profile].

> [!IMPORTANT]
>
> Your API key is *highly sensitive*, do not store it anywhere other people than
> you might get access to it.
>
> Facti maintainers will **NEVER** ask for your API key.
>
> Anyone who gets access to your API key can publish and/or modify your mods
> without your approval, depending on the permissions on the key.

To make use of all features in facti, you must enable all three usage checkboxes:

 * ModPortal: Upload Mods
 * ModPortal: Publish Mods
 * ModPortal: Edit Mods

Of course, if you know you will not use some of these, you can disable them
to avoid granting more permissions than necessary.

You can either provide the API key with every invocation of facti like so:

```sh
# Directly as a parameter
facti --api-key <your-api-key> ...

# from stdin to avoid it showing up in shell history
$ facti --api-key-stdin ...
Please input your Factorio API key to continue:
>

# read it from a file
$ facti --api-key-file <path-to-file> ...
```

Or save it in facti's configuration file:

```toml
[factorio]
api-key = "<your-api-key>"
```

facti will look for its configuration file in the following places and in this
order:

 1. `$XDG_CONFIG_HOME/facti/config.toml`
 2. `$HOME/.config/facti/config.toml`

You can manually specify the location of the config file when invoking facti:

```sh
facti --config <path-to-config-file> ...
```

You can also supply the API key via environment variables:

```sh
export FACTI_FACTORIO_API_KEY="<your-api-key>"
facti ...
```

As well as the path to the config file:

```sh
export FACTI_CONFIG="<path-to-config-file>"
facti ...
```

### Configuration hierarchy

As a rule, the most "direct" application of a setting is the one that will be in
effect.

Essentially, this means settings are resolved in this order:

 1. Command line arguments
 2. Environment variables
 3. Configuration file

The first one encountered "wins".


For API keys specifially, there is also a priority within the different ways to
supply it:

 1. Direct value

     * `--api-key` command line option
     * `FACTI_FACTORIO_API_KEY` environment variable
     * `api-key` setting in config file
  2. From standard input (stdin) with `--api-key-stdin`
  3. From file

      * `--api-key-file` command line option
      * `FACTI_FACTORIO_API_KEY_FILE` environment variable
      * `api-key-file` setting in config file

The first one encountered wins, with respect to the primary hierarchy of
CLI arguments vs environment variables vs config file.

For example: If your config file has a value for `api-key`, and
the environment variable `FACTI_FACTORIO_API_KEY_FILE` is set,
and you also specify `--api-key-stdin`, then the stdin method will win,
because it was specified via command line argument, which is in the top primary
priority.


## License

Copyright © 2023 by [Adam Hellberg][sharparam].

This Source Code Form is subject to the terms of the
[Mozilla Public License, v. 2.0][mpl-2.0].
If a copy of the MPL was not distributed with this file,
You can obtain one at <http://mozilla.org/MPL/2.0/>.

[sharparam]: https://sharparam.com
[mpl-2.0]: http://mozilla.org/MPL/2.0/

[cratesio]: https://crates.io/crates/facti
[librs]: https://lib.rs/crates/facti
[docsrs]: https://docs.rs/facti
[cratesio-badge]: https://img.shields.io/crates/v/facti?logo=rust
[docsrs-badge]: https://img.shields.io/docsrs/facti/latest?logo=docsdotrs

[build]: https://github.com/Sharparam/facti/actions/workflows/test.yml?query=branch%3Amain
[audit]: https://github.com/Sharparam/facti/actions/workflows/audit.yml?query=branch%3Amain
[build-badge]: https://img.shields.io/github/actions/workflow/status/Sharparam/facti/test.yml?logo=github
[audit-badge]: https://img.shields.io/github/actions/workflow/status/Sharparam/facti/audit.yml?logo=github&label=audit

[discussions]: https://github.com/Sharparam/facti/discussions
[matrix-room]: https://matrix.to/#/#facti:sharparam.com
[discussions-badge]: https://img.shields.io/github/discussions/Sharparam/facti?logo=github
[matrix-badge]: https://img.shields.io/matrix/facti%3Asharparam.com?logo=matrix&label=%23facti%3Asharparam.com

[main]: https://github.com/Sharparam/facti/tree/main
[latest-release]: https://github.com/Sharparam/facti/releases/latest

[rust]: https://www.rust-lang.org
[rust-install]: https://www.rust-lang.org/tools/install

[factorio]: https://factorio.com
[factorio-api]: https://wiki.factorio.com/Factorio_HTTP_API_usage_guidelines
[factorio-mods]: https://mods.factorio.com/
[factorio-profile]: https://factorio.com/profile
