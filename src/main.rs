//! Enigma Simulator - Main Program
//!
//! This program implements a complete Enigma machine with both
//! a graphical user interface and a command-line interface.

// Use modules from the library
use enigma_rs::gui;
use enigma_rs::machine::factory;
use enigma_rs::utils::clean_text;

use clap::{Parser, Subcommand};
use env_logger::{Builder, Env};
use log::{error, info};

/// CLI arguments for the Enigma simulator
#[derive(Parser)]
#[command(name = "enigma_rs")]
#[command(about = "An Enigma simulator in Rust")]
#[command(version)]
struct Cli {
    /// Start CLI interface instead of GUI
    #[arg(long)]
    cli: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

/// CLI subcommands
#[derive(Subcommand)]
enum Commands {
    /// Encrypts a text
    Encrypt {
        /// The text to encrypt
        text: String,

        /// Rotor positions (e.g. "ABC")
        #[arg(short = 'P', long, default_value = "AAA")]
        positions: String,

        /// Ring settings (e.g. "ABC")
        #[arg(short, long, default_value = "AAA")]
        rings: String,

        /// Plugboard connections (e.g. "AB CD EF")
        #[arg(short, long)]
        plugboard: Option<String>,

        /// Rotor types (e.g. "I,II,III")
        #[arg(short = 'R', long, default_value = "I,II,III")]
        rotors: String,

        /// Reflector type
        #[arg(short = 'F', long, default_value = "B")]
        reflector: String,
    },

    /// Decrypts a text
    Decrypt {
        /// The text to decrypt
        text: String,

        /// Rotor positions (e.g. "ABC")
        #[arg(short = 'P', long, default_value = "AAA")]
        positions: String,

        /// Ring settings (e.g. "ABC")
        #[arg(short, long, default_value = "AAA")]
        rings: String,

        /// Plugboard connections (e.g. "AB CD EF")
        #[arg(short, long)]
        plugboard: Option<String>,

        /// Rotor types (e.g. "I,II,III")
        #[arg(short = 'R', long, default_value = "I,II,III")]
        rotors: String,

        /// Reflector type
        #[arg(short = 'F', long, default_value = "B")]
        reflector: String,
    },
}

/// CLI handler for encryption
fn handle_encrypt(
    text: String,
    positions: String,
    rings: String,
    plugboard: Option<String>,
    rotors: String,
    reflector: String,
) -> Result<(), String> {
    info!("Starting CLI encryption");

    let rotor_positions = parse_positions(&positions)?;
    let ring_settings = parse_positions(&rings)?;
    let rotor_types = parse_rotors(&rotors)?;
    let plugboard_connections = plugboard.unwrap_or_default();

    let mut machine = factory::create_custom_machine(
        rotor_types,
        rotor_positions,
        ring_settings,
        &reflector,
        &plugboard_connections,
    )?;

    let clean_input = clean_text(&text);
    info!("Encrypting: '{}'", clean_input);

    let result = machine.encrypt(&clean_input);
    println!("Result: {}", result);

    Ok(())
}

/// CLI handler for decryption
fn handle_decrypt(
    text: String,
    positions: String,
    rings: String,
    plugboard: Option<String>,
    rotors: String,
    reflector: String,
) -> Result<(), String> {
    info!("Starting CLI decryption");

    let rotor_positions = parse_positions(&positions)?;
    let ring_settings = parse_positions(&rings)?;
    let rotor_types = parse_rotors(&rotors)?;
    let plugboard_connections = plugboard.unwrap_or_default();

    let mut machine = factory::create_custom_machine(
        rotor_types,
        rotor_positions,
        ring_settings,
        &reflector,
        &plugboard_connections,
    )?;

    let clean_input = clean_text(&text);
    info!("Decrypting: '{}'", clean_input);

    let result = machine.decrypt(&clean_input);
    println!("Result: {}", result);

    Ok(())
}

/// Parses rotor positions from a string
fn parse_positions(positions: &str) -> Result<[char; 3], String> {
    if positions.len() != 3 {
        return Err("Position string must be exactly 3 characters long".to_string());
    }

    let chars: Vec<char> = positions.chars().collect();
    if !chars.iter().all(|&c| c.is_ascii_alphabetic()) {
        return Err("Position string may only contain letters".to_string());
    }

    Ok([chars[0], chars[1], chars[2]])
}

/// Parses rotor types from a string
fn parse_rotors(rotors: &str) -> Result<[&str; 3], String> {
    let parts: Vec<&str> = rotors.split(',').collect();
    if parts.len() != 3 {
        return Err("Rotor string must contain exactly 3 types (comma-separated)".to_string());
    }

    for &rotor in &parts {
        if !["I", "II", "III", "IV", "V"].contains(&rotor) {
            return Err(format!("Invalid rotor type: {}", rotor));
        }
    }

    Ok([parts[0], parts[1], parts[2]])
}

/// Starts the GUI application
fn start_gui() -> Result<(), eframe::Error> {
    info!("Starting GUI application");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Enigma Simulator",
        options,
        Box::new(|_cc| Box::new(gui::EnigmaApp::new())),
    )
}

/// Main function
fn main() {
    let cli = Cli::parse();

    // Initialize logger
    let env = Env::default().filter_or("RUST_LOG", if cli.verbose { "debug" } else { "info" });
    Builder::from_env(env)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{}] {}: {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    info!("Enigma simulator started");

    // Start GUI or CLI
    if cli.cli || cli.command.is_some() {
        // CLI mode
        match cli.command {
            Some(Commands::Encrypt {
                text,
                positions,
                rings,
                plugboard,
                rotors,
                reflector,
            }) => {
                if let Err(e) = handle_encrypt(text, positions, rings, plugboard, rotors, reflector)
                {
                    error!("Encryption error: {}", e);
                    std::process::exit(1);
                }
            }
            Some(Commands::Decrypt {
                text,
                positions,
                rings,
                plugboard,
                rotors,
                reflector,
            }) => {
                if let Err(e) = handle_decrypt(text, positions, rings, plugboard, rotors, reflector)
                {
                    error!("Decryption error: {}", e);
                    std::process::exit(1);
                }
            }
            None => {
                println!("No command specified. Use --help for help.");
                std::process::exit(1);
            }
        }
    } else {
        // GUI mode (default)
        if let Err(e) = start_gui() {
            error!("GUI error: {}", e);
            std::process::exit(1);
        }
    }

    info!("Enigma simulator ended");
}
