//! Tests für den Enigma-Simulator
//!
//! Diese Tests überprüfen die grundlegende Funktionalität der Enigma-Maschine,
//! insbesondere die Symmetrie von Verschlüsselung und Entschlüsselung.

use enigma_rs::machine::{factory, EnigmaMachine};
use enigma_rs::utils::clean_text;

/// Testet die grundlegende Verschlüsselung und Entschlüsselung
#[test]
fn test_encrypt_decrypt_symmetry() {
    // Erstelle eine Standard-Enigma-Maschine
    let mut machine = factory::create_standard_machine(
        ['A', 'A', 'A'], // Positionen
        ['A', 'A', 'A'], // Ringstellungen
        "AB CD EF",      // Steckerbrett
    )
    .expect("Maschine sollte erstellt werden können");

    let original_text = "HELLO WORLD";
    let clean_original = clean_text(&original_text);

    // Verschlüssele den Text
    let encrypted = machine.encrypt(&clean_original);

    // Setze die Maschine zurück auf die ursprünglichen Positionen
    machine.set_rotor_positions(['A', 'A', 'A']);

    // Entschlüssele den Text
    let decrypted = machine.decrypt(&encrypted);

    // Der entschlüsselte Text sollte dem ursprünglichen entsprechen
    assert_eq!(clean_text(&decrypted), clean_original);
}

/// Testet verschiedene Rotorpositionen
#[test]
fn test_different_positions() {
    let mut machine = factory::create_standard_machine(
        ['B', 'C', 'D'], // Verschiedene Positionen
        ['A', 'A', 'A'], // Ringstellungen
        "",              // Kein Steckerbrett
    )
    .expect("Maschine sollte erstellt werden können");

    let text = "TEST";
    let encrypted = machine.encrypt(text);

    // Setze zurück
    machine.set_rotor_positions(['B', 'C', 'D']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Testet verschiedene Ringstellungen
#[test]
fn test_different_ring_settings() {
    let mut machine = factory::create_standard_machine(
        ['A', 'A', 'A'], // Positionen
        ['B', 'C', 'D'], // Verschiedene Ringstellungen
        "",              // Kein Steckerbrett
    )
    .expect("Maschine sollte erstellt werden können");

    let text = "ENIGMA";
    let encrypted = machine.encrypt(text);

    // Setze zurück
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Testet verschiedene Rotortypen
#[test]
fn test_different_rotor_types() {
    let mut machine = factory::create_custom_machine(
        ["II", "IV", "V"], // Verschiedene Rotortypen
        ['A', 'A', 'A'],   // Positionen
        ['A', 'A', 'A'],   // Ringstellungen
        "B",               // Reflektor
        "",                // Kein Steckerbrett
    )
    .expect("Maschine sollte erstellt werden können");

    let text = "ROTORS";
    let encrypted = machine.encrypt(text);

    // Setze zurück
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Testet verschiedene Reflektoren
#[test]
fn test_different_reflectors() {
    // Teste Reflektor A
    let mut machine_a = factory::create_custom_machine(
        ["I", "II", "III"],
        ['A', 'A', 'A'],
        ['A', 'A', 'A'],
        "A",
        "",
    )
    .expect("Maschine sollte erstellt werden können");

    let text = "REFLECTOR";
    let encrypted_a = machine_a.encrypt(text);

    // Teste Reflektor B
    let mut machine_b = factory::create_custom_machine(
        ["I", "II", "III"],
        ['A', 'A', 'A'],
        ['A', 'A', 'A'],
        "B",
        "",
    )
    .expect("Maschine sollte erstellt werden können");

    let encrypted_b = machine_b.encrypt(text);

    // Verschiedene Reflektoren sollten verschiedene Ergebnisse liefern
    assert_ne!(encrypted_a, encrypted_b);

    // Aber beide sollten symmetrisch sein
    machine_a.set_rotor_positions(['A', 'A', 'A']);
    machine_b.set_rotor_positions(['A', 'A', 'A']);

    let decrypted_a = machine_a.decrypt(&encrypted_a);
    let decrypted_b = machine_b.decrypt(&encrypted_b);

    assert_eq!(clean_text(&decrypted_a), clean_text(text));
    assert_eq!(clean_text(&decrypted_b), clean_text(text));
}

/// Testet das Steckerbrett
#[test]
fn test_plugboard() {
    let mut machine = factory::create_standard_machine(
        ['A', 'A', 'A'],
        ['A', 'A', 'A'],
        "AB CD EF", // Steckerbrett-Verbindungen
    )
    .expect("Maschine sollte erstellt werden können");

    let text = "PLUGBOARD";
    let encrypted = machine.encrypt(text);

    // Setze zurück
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Testet längere Texte
#[test]
fn test_longer_text() {
    let mut machine =
        factory::create_standard_machine(['A', 'A', 'A'], ['A', 'A', 'A'], "AB CD EF GH IJ")
            .expect("Maschine sollte erstellt werden können");

    let long_text = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
    let encrypted = machine.encrypt(long_text);

    // Setze zurück
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(long_text));
}

/// Testet die Rotor-Drehung
#[test]
fn test_rotor_stepping() {
    let mut machine = factory::create_standard_machine(['A', 'A', 'A'], ['A', 'A', 'A'], "")
        .expect("Maschine sollte erstellt werden können");

    let char1 = machine.encrypt_char('A');
    let char2 = machine.encrypt_char('A');

    // Gleiche Eingabe sollte nach Rotor-Drehung verschiedene Ausgaben liefern
    assert_ne!(char1, char2);
}

/// Testet die Konfigurationsinformationen
#[test]
fn test_configuration_info() {
    let machine = factory::create_standard_machine(['B', 'C', 'D'], ['E', 'F', 'G'], "AB CD")
        .expect("Maschine sollte erstellt werden können");

    let config = machine.get_configuration_info();

    // Überprüfe, dass alle wichtigen Informationen enthalten sind
    assert!(config.contains("I II III")); // Rotortypen
    assert!(config.contains("E F G")); // Ringstellungen
    assert!(config.contains("B C D")); // Positionen
    assert!(config.contains("B")); // Reflektor
    assert!(config.contains("AB CD")); // Steckerbrett
}
