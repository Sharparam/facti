use std::{
    fmt::{self, Display},
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::{Args, Subcommand, ValueEnum, ValueHint};
use facti_lib::changelog::Changelog;

#[derive(Args, Debug)]
pub struct ChangelogArgs {
    #[command(subcommand)]
    pub command: ChangelogCommands,
}

#[derive(Subcommand, Debug)]
pub enum ChangelogCommands {
    /// Convert mod changelogs between different formats.
    Convert(ChangelogConvertArgs),
}

#[derive(Args, Debug)]
pub struct ChangelogConvertArgs {
    pub input: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value_t = Default::default())]
    pub from: ChangelogFormat,

    #[arg(short, long, value_enum, default_value_t = Default::default())]
    pub to: ChangelogFormat,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub output: Option<PathBuf>,
}

#[derive(Default, ValueEnum, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ChangelogFormat {
    #[default]
    #[value(alias = "default")]
    Factorio,
    Json,
    #[cfg(feature = "yaml")]
    Yaml,
    Toml,
    #[cfg(feature = "ron")]
    Ron,
    #[cfg(feature = "sexpr")]
    Sexpr,
    #[cfg(feature = "sexpr")]
    Elisp,
}

impl ChangelogArgs {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            ChangelogCommands::Convert(args) => args.run(),
        }
    }
}

impl ChangelogConvertArgs {
    fn run(&self) -> Result<()> {
        self.convert()?;

        Ok(())
    }

    fn convert(&self) -> Result<()> {
        let changelog = self.read()?;
        self.write(changelog)?;

        Ok(())
    }

    fn read(&self) -> Result<Changelog> {
        let mut reader = self.reader()?;

        match self.from {
            ChangelogFormat::Factorio => {
                let mut buffer = String::new();
                reader.read_to_string(&mut buffer)?;
                Changelog::parse(&buffer).context("Converting from default Factorio format failed")
            }
            ChangelogFormat::Json => {
                serde_json::from_reader(reader).context("Converting from JSON failed")
            }
            #[cfg(feature = "yaml")]
            ChangelogFormat::Yaml => {
                serde_yaml::from_reader(reader).context("Converting from YAML failed")
            }
            ChangelogFormat::Toml => {
                let mut buffer = String::new();
                reader.read_to_string(&mut buffer)?;
                toml::from_str(&buffer).context("Converting from TOML failed")
            }
            #[cfg(feature = "ron")]
            ChangelogFormat::Ron => {
                ron::de::from_reader(reader).context("Converting from RON failed")
            }
            #[cfg(feature = "sexpr")]
            ChangelogFormat::Sexpr => {
                serde_lexpr::from_reader(reader).context("Converting from S-expressions failed")
            }
            #[cfg(feature = "sexpr")]
            ChangelogFormat::Elisp => {
                serde_lexpr::from_reader_custom(reader, serde_lexpr::parse::Options::elisp())
                    .context("Converting from Emacs Lisp failed")
            }
        }
    }

    fn write(&self, changelog: Changelog) -> Result<()> {
        let mut writer = self.writer()?;

        match self.to {
            ChangelogFormat::Factorio => {
                let content = changelog.to_string_sorted()?;
                writer.write_all(content.as_bytes())?;
            }
            ChangelogFormat::Json => {
                serde_json::to_writer_pretty(writer, &changelog)?;
            }
            #[cfg(feature = "yaml")]
            ChangelogFormat::Yaml => {
                serde_yaml::to_writer(writer, &changelog)?;
            }
            ChangelogFormat::Toml => {
                let content = toml::to_string_pretty(&changelog)?;
                writer.write_all(content.as_bytes())?;
            }
            #[cfg(feature = "ron")]
            ChangelogFormat::Ron => {
                ron::ser::to_writer_pretty(writer, &changelog, Default::default())?;
            }
            #[cfg(feature = "sexpr")]
            ChangelogFormat::Sexpr => {
                serde_lexpr::to_writer(writer, &changelog)?;
            }
            #[cfg(feature = "sexpr")]
            ChangelogFormat::Elisp => serde_lexpr::to_writer_custom(
                writer,
                &changelog,
                serde_lexpr::print::Options::elisp(),
            )?,
        }

        Ok(())
    }

    fn reader(&self) -> Result<Box<dyn BufRead>> {
        match &self.input {
            Some(path) => Ok(Box::new(BufReader::new(File::open(path)?))),
            None => Ok(Box::new(io::stdin().lock())),
        }
    }

    fn writer(&self) -> Result<BufWriter<Box<dyn Write>>> {
        match &self.output {
            Some(path) => Ok(BufWriter::new(Box::new(File::create(path)?))),
            None => Ok(BufWriter::new(Box::new(io::stdout().lock()))),
        }
    }
}

impl Display for ChangelogFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ChangelogFormat::*;
        match self {
            Factorio => f.write_str("Factorio changelog format"),
            Json => f.write_str("JSON"),
            #[cfg(feature = "yaml")]
            Yaml => f.write_str("YAML"),
            Toml => f.write_str("TOML"),
            #[cfg(feature = "ron")]
            Ron => f.write_str("RON"),
            #[cfg(feature = "sexpr")]
            Sexpr => f.write_str("S-expressions"),
            #[cfg(feature = "sexpr")]
            Elisp => f.write_str("Emacs Lisp"),
        }
    }
}
