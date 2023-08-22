use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use clap::{Args, CommandFactory, Parser, Subcommand};

use facti::__xtask::Cli;
use xshell::{cmd, Shell};

fn main() -> Result<()> {
    cwd_to_workspace_root()?;

    let cli = BuildCli::parse();

    match cli.task {
        Tasks::Man => {
            let out_dir = PathBuf::from("target/assets/man");
            fs::create_dir_all(&out_dir)?;
            let cmd = Cli::command();
            gen_manpages(&out_dir, &cmd, None)?;
        }
        Tasks::Dist(dist_args) => {
            let verbose = if cli.verbose { Some("--verbose") } else { None };
            let cargo = &if cli.cross {
                "cross".to_owned()
            } else {
                cli.cargo.unwrap_or_else(|| "cargo".to_owned())
            };
            let sh = Shell::new()?;
            dbg!(cmd!(sh, "pwd").read()?);

            eprintln!("Clean dist folder");
            cmd!(sh, "rm -rf target/dist").run()?;

            let target = &dist_args.target;

            eprintln!("Build facti");
            cmd!(
                sh,
                "{cargo} build {verbose...} --profile dist --all-features --locked --target {target} --target-dir target/dist --package facti"
            )
            .run()?;
        }
    }

    Ok(())
}

fn gen_manpages(out_dir: &Path, cmd: &clap::Command, current_name: Option<&str>) -> Result<()> {
    let cmd_name = if let Some(current_name) = current_name {
        format!("{}-{}", current_name, cmd.get_name())
    } else {
        cmd.get_name().to_owned()
    };
    eprintln!("Generating manpage for {}", cmd_name);
    let file_name = format!("{}.1", cmd_name);
    let out_path = out_dir.join(file_name);

    let man = clap_mangen::Man::new(cmd.to_owned());
    let mut buffer: Vec<u8> = vec![];
    man.render(&mut buffer)
        .with_context(|| format!("Failed to render command {} to buffer", cmd_name))?;

    eprintln!("Writing rendered manpage to {}", out_path.display());
    fs::write(&out_path, buffer).with_context(|| {
        format!(
            "Failed to write rendered man page for {} to file {}",
            cmd_name,
            out_path.display()
        )
    })?;

    for subcommand in cmd.get_subcommands() {
        gen_manpages(out_dir, subcommand, Some(&cmd_name))?;
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(bin_name = "cargo xtask")]
struct BuildCli {
    #[arg(long)]
    pub verbose: bool,

    #[arg(long, env = "CARGO", global = true)]
    pub cargo: Option<String>,

    #[arg(long, global = true)]
    pub cross: bool,

    #[command(subcommand)]
    pub task: Tasks,
}

#[derive(Subcommand, Debug)]
enum Tasks {
    /// Generate manpages for Facti.
    Man,

    /// `cargo-dist` lookalike.
    ///
    /// We use our own until there's cross-platform build support
    /// in the actual cargo-dist.
    Dist(DistArgs),
}

#[derive(Args, Debug)]
struct DistArgs {
    #[arg(long)]
    pub target: String,

    #[arg(raw = true)]
    pub cargo_args: Vec<String>,
}

// Shamelessly stolen^Wcopied from cargo itself:
// https://github.com/rust-lang/cargo/blob/e5e68c4093af9de3f80e9427b979fa5a0d8361cc/crates/xtask-build-man/src/main.rs#L78-L82
fn cwd_to_workspace_root() -> Result<()> {
    let pkg_root = std::env!("CARGO_MANIFEST_DIR");
    let ws_root = format!("{pkg_root}/../..");
    eprintln!("Performing CWD to workspace root {}", ws_root);
    env::set_current_dir(ws_root).context("Failed to CWD to workspace root")?;
    let pwd = env::current_dir().context("Failed to get PWD after change")?;
    eprintln!("Now in: {}", pwd.display());
    Ok(())
}
