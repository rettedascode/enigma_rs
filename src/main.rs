//! Enigma-Simulator - Hauptprogramm
//! 
//! Dieses Programm implementiert eine vollständige Enigma-Maschine mit sowohl
//! einer grafischen Benutzeroberfläche als auch einem Command-Line-Interface.

// Verwende die Module aus der Library
use enigma_rs::machine::factory;
use enigma_rs::utils::clean_text;
use enigma_rs::gui;

use clap::{Parser, Subcommand};
use log::{info, error};
use env_logger::{Builder, Env};


/// CLI-Argumente für den Enigma-Simulator
#[derive(Parser)]
#[command(name = "enigma_rs")]
#[command(about = "Ein Enigma-Simulator in Rust")]
#[command(version)]
struct Cli {
    /// Startet das CLI-Interface statt der GUI
    #[arg(long)]
    cli: bool,
    
    /// Verbose Ausgabe aktivieren
    #[arg(short, long)]
    verbose: bool,
    
    /// Subkommandos
    #[command(subcommand)]
    command: Option<Commands>,
}

/// CLI-Subkommandos
#[derive(Subcommand)]
enum Commands {
    /// Verschlüsselt einen Text
    Encrypt {
        /// Der zu verschlüsselnde Text
        text: String,
        
        /// Rotorpositionen (z.B. "ABC")
        #[arg(short = 'P', long, default_value = "AAA")]
        positions: String,
        
        /// Ringstellungen (z.B. "ABC")
        #[arg(short, long, default_value = "AAA")]
        rings: String,
        
        /// Steckerbrett-Verbindungen (z.B. "AB CD EF")
        #[arg(short, long)]
        plugboard: Option<String>,
        
        /// Rotor-Typen (z.B. "I,II,III")
        #[arg(short = 'R', long, default_value = "I,II,III")]
        rotors: String,
        
        /// Reflektor-Typ
        #[arg(short = 'F', long, default_value = "B")]
        reflector: String,
    },
    
    /// Entschlüsselt einen Text
    Decrypt {
        /// Der zu entschlüsselnde Text
        text: String,
        
        /// Rotorpositionen (z.B. "ABC")
        #[arg(short = 'P', long, default_value = "AAA")]
        positions: String,
        
        /// Ringstellungen (z.B. "ABC")
        #[arg(short, long, default_value = "AAA")]
        rings: String,
        
        /// Steckerbrett-Verbindungen (z.B. "AB CD EF")
        #[arg(short, long)]
        plugboard: Option<String>,
        
        /// Rotor-Typen (z.B. "I,II,III")
        #[arg(short = 'R', long, default_value = "I,II,III")]
        rotors: String,
        
        /// Reflektor-Typ
        #[arg(short = 'F', long, default_value = "B")]
        reflector: String,
    },
}

/// CLI-Handler für Verschlüsselung
fn handle_encrypt(
    text: String,
    positions: String,
    rings: String,
    plugboard: Option<String>,
    rotors: String,
    reflector: String,
) -> Result<(), String> {
    info!("Starte CLI-Verschlüsselung");
    
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
    info!("Verschlüssele: '{}'", clean_input);
    
    let result = machine.encrypt(&clean_input);
    println!("Ergebnis: {}", result);
    
    Ok(())
}

/// CLI-Handler für Entschlüsselung
fn handle_decrypt(
    text: String,
    positions: String,
    rings: String,
    plugboard: Option<String>,
    rotors: String,
    reflector: String,
) -> Result<(), String> {
    info!("Starte CLI-Entschlüsselung");
    
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
    info!("Entschlüssele: '{}'", clean_input);
    
    let result = machine.decrypt(&clean_input);
    println!("Ergebnis: {}", result);
    
    Ok(())
}

/// Parst Rotorpositionen aus einem String
fn parse_positions(positions: &str) -> Result<[char; 3], String> {
    if positions.len() != 3 {
        return Err("Positions-String muss genau 3 Zeichen lang sein".to_string());
    }
    
    let chars: Vec<char> = positions.chars().collect();
    if !chars.iter().all(|&c| c.is_ascii_alphabetic()) {
        return Err("Positions-String darf nur Buchstaben enthalten".to_string());
    }
    
    Ok([chars[0], chars[1], chars[2]])
}

/// Parst Rotortypen aus einem String
fn parse_rotors(rotors: &str) -> Result<[&str; 3], String> {
    let parts: Vec<&str> = rotors.split(',').collect();
    if parts.len() != 3 {
        return Err("Rotor-String muss genau 3 Typen enthalten (durch Komma getrennt)".to_string());
    }
    
    for &rotor in &parts {
        if !["I", "II", "III", "IV", "V"].contains(&rotor) {
            return Err(format!("Ungültiger Rotortyp: {}", rotor));
        }
    }
    
    Ok([parts[0], parts[1], parts[2]])
}

/// Startet die GUI-Anwendung
fn start_gui() -> Result<(), eframe::Error> {
    info!("Starte GUI-Anwendung");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Enigma-Simulator",
        options,
        Box::new(|_cc| Box::new(gui::EnigmaApp::new())),
    )
}

/// Hauptfunktion
fn main() {
    let cli = Cli::parse();
    
    // Logger initialisieren
    let env = Env::default().filter_or("RUST_LOG", if cli.verbose { "debug" } else { "info" });
    Builder::from_env(env)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, "[{}] {}: {}", 
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
    
    info!("Enigma-Simulator gestartet");
    
    // GUI oder CLI starten
    if cli.cli || cli.command.is_some() {
        // CLI-Modus
        match cli.command {
            Some(Commands::Encrypt { text, positions, rings, plugboard, rotors, reflector }) => {
                if let Err(e) = handle_encrypt(text, positions, rings, plugboard, rotors, reflector) {
                    error!("Verschlüsselungsfehler: {}", e);
                    std::process::exit(1);
                }
            }
            Some(Commands::Decrypt { text, positions, rings, plugboard, rotors, reflector }) => {
                if let Err(e) = handle_decrypt(text, positions, rings, plugboard, rotors, reflector) {
                    error!("Entschlüsselungsfehler: {}", e);
                    std::process::exit(1);
                }
            }
            None => {
                println!("Kein Kommando angegeben. Verwende --help für Hilfe.");
                std::process::exit(1);
            }
        }
    } else {
        // GUI-Modus (Standard)
        if let Err(e) = start_gui() {
            error!("GUI-Fehler: {}", e);
            std::process::exit(1);
        }
    }
    
    info!("Enigma-Simulator beendet");
}
