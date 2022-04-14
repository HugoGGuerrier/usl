mod config;
mod builder;
mod interpreter;
mod creator;

use std::env;
use crate::config::{Config, Mode};
use crate::creator::run;

// ===== Program arguments functions =====

/// From the arguments vector, get the corresponding USL mode
///
/// Parameters :
///     * args          - The program arguments
///
/// Returns             - The execution mode of USL
fn get_mode(args: &Vec<String>) -> Option<Mode> {
    return if args.len() >= 2 {
        match &*args[1] {
            "build" => Some(Mode::BUILD),
            "run" => Some(Mode::RUN),
            "create" => Some(Mode::CREATE),
            _ => None
        }
    } else {
        None
    }
}

/// Parse an argument for all the execution modes and change the application config according to it
/// Return if the argument was parsed
///
/// Parameters :
///     * args          - A vector of string of the program arguments
///     * config        - The application configuration
///
/// Returns             - If the argument parsing worked
fn parse_arg_common(arg: &str, config: &mut Config) -> bool {
    false
}

/// Parse the arguments for the build mode and change the application config according to it
///
/// Parameters :
///     * args          - A vector of string of the program arguments
///     * config        - The application configuration
fn parse_args_build(args: &Vec<String>, config: &mut Config) {
    // TODO : Parse the arguments for the build mode
}

/// Parse the arguments for the run mode and change the application config according to it
///
/// Parameters :
///     * args          - A vector of string of the program arguments
///     * config        - The application configuration
fn parse_args_run(args: &Vec<String>, config: &mut Config) {
    // TODO : Parse the arguments for the run mode
}

/// Parse the arguments for the create mode and change the application config according to it
///
/// Parameters :
///     * args          - A vector of string of the program arguments
///     * config        - The application configuration
fn parse_args_create(args: &Vec<String>, config: &mut Config) {
    // TODO : Parse the arguments for the create mode
}

// ===== Helping messages functions =====

/// Display the help for the mode selection
fn mode_help() {
    println!("Unknown USL execution mode");
    println!("Usage : usl [build|run|create] [OPTIONS]");
}

/// Display the help for the build mode
fn build_help() {
    println!("TODO : Display the help for the build mode");
}

/// Display the help for the run mode
fn run_help() {
    println!("TODO : Display the help for the run mode");
}

/// Display the help for the create mode
fn create_help() {
    println!("TODO : Display the help for the create mode");
}

// ===== Main functions =====

/// The program main entry point
fn main() {
    // Create the program values
    let mut config: Config = Config::new();

    // Get the arguments of the user
    let arguments: Vec<String> = env::args().collect();

    // Get the application mode in the arguments
    let execution_mode = get_mode(&arguments);
    match execution_mode {
        None => {
            mode_help();
            return;
        }
        Some(mode) => config.mode = mode
    };

    // According to the execution mode, parse the arguments
    match config.mode {
        Mode::BUILD => {}
        Mode::RUN => {}
        Mode::CREATE => {}
    };

    // Display the help if needed
    if config.help {
        match config.mode {
            Mode::BUILD => {}
            Mode::RUN => {}
            Mode::CREATE => {}
        };
        return;
    };

    // Verify if the argument parsing failed
    if config.arg_error {
        eprintln!("Error in the argument parsing for the {} mode !", config.mode);
        eprintln!("Use -h to display the help message");
        return;
    }

    // Finally run the correct mode with the parameters
    match config.mode {
        Mode::BUILD => builder::run(&config),
        Mode::RUN => interpreter::run(&config),
        Mode::CREATE => creator::run(&config)
    }
}
