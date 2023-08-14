use std::io;

use clap::{Args, Command, CommandFactory};
use clap_complete::{Generator, Shell};
use tracing::debug;

/// Generate shell completions.
#[derive(Args, Debug)]
pub struct CompletionArgs {
    /// The shell to generate completions for.
    #[arg(value_enum)]
    pub shell: Shell,
}

impl CompletionArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut cmd = super::Cli::command();
        debug!("Generating completions for {:?}", self.shell);
        print_completions(self.shell, &mut cmd);

        Ok(())
    }
}

pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    clap_complete::generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    )
}
