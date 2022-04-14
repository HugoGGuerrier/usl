use std::fmt::{Display, Formatter, write};

/// The enum that represent all the execution mode for USL
///
/// Modes :
///     * BUILD         - Build a USL project
///     * RUN           - Run an USL script
///     * CREATE        - Create an USL project
pub enum Mode {
    BUILD,
    RUN,
    CREATE
}

/// The display implementation for the USL execution mode
impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Mode::BUILD => "build",
                Mode::RUN => "run",
                Mode::CREATE => "create"
            }
        )
    }
}

/// The application configuration structure for the application
///
/// Fields :
///     * mode              - The USL execution mode
///     * arg_error         - If there is an error in the argument parsing
///     * help              - If the help is required
pub struct Config {
    pub mode: Mode,

    pub arg_error: bool,
    pub help: bool,
}

/// Implementation for the configuration structure
impl Config {

    /// Create a new application configuration with the default values
    ///
    /// Returns             - A new instance of the default configuration
    pub fn new() -> Config {
        Config {
            mode: Mode::BUILD,

            arg_error: false,
            help: false
        }
    }

}

/// Display implementation for the config structure
impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config[mode={}]", self.mode)
    }
}
