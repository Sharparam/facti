# facti (CLI)

Rust CLI tool to help with Factorio mod development.

Very much a **work in progress** and highly **experimental**.

## Expected mod layout

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

## Usage

For more help on the CLI commands, you can run `facti help <command>`
or `facti <command> --help`.

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

### API

You can interact with the Factorio APIs via facti by using the `facti api` command.

Here are some examples:

```sh
# search for mods that match the name "cybersyn-combinator"
facti api search cybersyn-combinator

# show information about the cybersyn-combinator mod
facti api show cybersyn-combinator

# show more detailed information about the cybersyn-combinator mod
facti api show --full cybersyn-combinator

# by default, deprecated mods are excluded from search
# if you want to show them you have to specify the --deprecated flag
facti api search --deprecated

# you can enable JSON output by supplying the --json flag
# in a non-interactive context, this is the default
# and can be negated with --no-json
facti api show --full --json cybersyn-combinator

# checks if all listed mods are compatible with each other
# currently this just makes sure none of the mods have each other listed
# as "incompatible"
facti api check cybersyn-combinator cybersyn

# by default the above command will check against the latest version of each
# mod, to check a specific version you can include a version requirement
facti api check cybersyn-combinator@0.6.0 cybersyn@1.0.2
```

## Configuration

Some commands like uploading mod packages to the mod portal require the use
of an API key.

To obtain this, you must generated an API key on your [Factorio profile page][factorio-profile].

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

[factorio-profile]: https://factorio.com/profile
