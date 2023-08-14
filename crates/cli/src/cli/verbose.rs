use indoc::indoc;

#[derive(clap::Args, Debug, Clone)]
pub struct Verbosity<L: LogLevel = ErrorLevel> {
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
        help = L::verbose_help(),
        long_help = L::verbose_long_help()
    )]
    verbose: u8,

    #[arg(
        long,
        short = 'q',
        action = clap::ArgAction::Count,
        global = true,
        help = L::quiet_help(),
        long_help = L::quiet_long_help(),
        conflicts_with = "verbose"
    )]
    quiet: u8,

    #[arg(skip)]
    _phantom: std::marker::PhantomData<L>,
}

impl<L: LogLevel> Verbosity<L> {
    #[allow(dead_code)]
    pub fn new(verbose: u8, quiet: u8) -> Self {
        Verbosity {
            verbose,
            quiet,
            _phantom: std::marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn log_level(&self) -> Option<tracing::Level> {
        level_enum(self.verbosity())
    }

    pub fn log_level_filter(&self) -> tracing::level_filters::LevelFilter {
        level_enum(self.verbosity())
            .map(|l| l.into())
            .unwrap_or(tracing::level_filters::LevelFilter::OFF)
    }

    pub fn is_given(&self) -> bool {
        self.verbose > 0 || self.quiet > 0
    }

    #[allow(dead_code)]
    pub fn is_silent(&self) -> bool {
        self.log_level().is_none()
    }

    fn verbosity(&self) -> i8 {
        level_value(L::default()) - (self.quiet as i8) + (self.verbose as i8)
    }
}

fn level_value(level: Option<tracing::Level>) -> i8 {
    match level {
        None => -1,
        Some(tracing::Level::ERROR) => 0,
        Some(tracing::Level::WARN) => 1,
        Some(tracing::Level::INFO) => 2,
        Some(tracing::Level::DEBUG) => 3,
        Some(tracing::Level::TRACE) => 4,
    }
}

fn level_enum(verbosity: i8) -> Option<tracing::Level> {
    match verbosity {
        i8::MIN..=-1 => None,
        0 => Some(tracing::Level::ERROR),
        1 => Some(tracing::Level::WARN),
        2 => Some(tracing::Level::INFO),
        3 => Some(tracing::Level::DEBUG),
        4..=i8::MAX => Some(tracing::Level::TRACE),
    }
}

pub trait LogLevel {
    fn default() -> Option<tracing::Level>;

    fn verbose_help() -> Option<&'static str> {
        Some("More output per occurrence")
    }

    fn verbose_long_help() -> Option<&'static str> {
        Some(indoc! {"
            Specifies the amount of verbosity desired.

            The more times this option is specified, the more verbose the output
            will become.

            Specifically, they relate as follows:
                -v       Show warnings
                -vv      Show info
                -vvv     Show debug
                -vvvv    Show trace

            Specifying this option more than four times has no further effect.
        "})
    }

    fn quiet_help() -> Option<&'static str> {
        Some("Silences output")
    }

    fn quiet_long_help() -> Option<&'static str> {
        Some(indoc! {"
            Silences all (logging) output.

            Specifying this option more than once has no effect.
        "})
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    fn default() -> Option<tracing::Level> {
        Some(tracing::Level::ERROR)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    fn default() -> Option<tracing::Level> {
        Some(tracing::Level::WARN)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    fn default() -> Option<tracing::Level> {
        Some(tracing::Level::INFO)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct DebugLevel;

impl LogLevel for DebugLevel {
    fn default() -> Option<tracing::Level> {
        Some(tracing::Level::DEBUG)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct TraceLevel;

impl LogLevel for TraceLevel {
    fn default() -> Option<tracing::Level> {
        Some(tracing::Level::TRACE)
    }
}
