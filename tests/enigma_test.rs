use enigma_rs::machine::factory;
use enigma_rs::utils::clean_text;

#[test]
fn test_encrypt_decrypt_symmetry() {
    // Create a standard Enigma machine
    let mut machine = factory::create_standard_machine(
        ['A', 'A', 'A'], // Positions
        ['A', 'A', 'A'], // Ring settings
        "AB CD EF",      // Plugboard
    )
    .expect("Machine should be creatable");

    let original_text = "HELLO WORLD";
    let clean_original = clean_text(&original_text);

    // Encrypt the text
    let encrypted = machine.encrypt(&clean_original);

    // Reset the machine to the original positions
    machine.set_rotor_positions(['A', 'A', 'A']);

    // Decrypt the text
    let decrypted = machine.decrypt(&encrypted);

    // The decrypted text should match the original
    assert_eq!(clean_text(&decrypted), clean_original);
}

/// Tests different rotor positions
#[test]
fn test_different_positions() {
    let mut machine = factory::create_standard_machine(
        ['B', 'C', 'D'], // Different positions
        ['A', 'A', 'A'], // Ring settings
        "",              // No plugboard
    )
    .expect("Machine should be creatable");

    let text = "TEST";
    let encrypted = machine.encrypt(text);

    // Reset
    machine.set_rotor_positions(['B', 'C', 'D']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Tests different ring settings
#[test]
fn test_different_ring_settings() {
    let mut machine = factory::create_standard_machine(
        ['A', 'A', 'A'], // Positions
        ['B', 'C', 'D'], // Different ring settings
        "",              // No plugboard
    )
    .expect("Machine should be creatable");

    let text = "ENIGMA";
    let encrypted = machine.encrypt(text);

    // Reset
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Tests different rotor types
#[test]
fn test_different_rotor_types() {
    let mut machine = factory::create_custom_machine(
        ["II", "IV", "V"], // Different rotor types
        ['A', 'A', 'A'],   // Positions
        ['A', 'A', 'A'],   // Ring settings
        "B",               // Reflector
        "",                // No plugboard
    )
    .expect("Machine should be creatable");

    let text = "ROTORS";
    let encrypted = machine.encrypt(text);

    // Reset
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Tests different reflectors
#[test]
fn test_different_reflectors() {
    // Test reflector A
    let mut machine_a = factory::create_custom_machine(
        ["I", "II", "III"],
        ['A', 'A', 'A'],
        ['A', 'A', 'A'],
        "A",
        "",
    )
    .expect("Machine should be creatable");

    let text = "REFLECTOR";
    let encrypted_a = machine_a.encrypt(text);

    // Test reflector B
    let mut machine_b = factory::create_custom_machine(
        ["I", "II", "III"],
        ['A', 'A', 'A'],
        ['A', 'A', 'A'],
        "B",
        "",
    )
    .expect("Machine should be creatable");

    let encrypted_b = machine_b.encrypt(text);

    // Different reflectors should produce different results
    assert_ne!(encrypted_a, encrypted_b);

    // But both should be symmetric
    machine_a.set_rotor_positions(['A', 'A', 'A']);
    machine_b.set_rotor_positions(['A', 'A', 'A']);

    let decrypted_a = machine_a.decrypt(&encrypted_a);
    let decrypted_b = machine_b.decrypt(&encrypted_b);

    assert_eq!(clean_text(&decrypted_a), clean_text(text));
    assert_eq!(clean_text(&decrypted_b), clean_text(text));
}

/// Tests the plugboard
#[test]
fn test_plugboard() {
    let mut machine = factory::create_standard_machine(
        ['A', 'A', 'A'],
        ['A', 'A', 'A'],
        "AB CD EF", // Plugboard-Verbindungen
    )
    .expect("Machine should be creatable");

    let text = "PLUGBOARD";
    let encrypted = machine.encrypt(text);

    // Reset
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(text));
}

/// Tests longer texts
#[test]
fn test_longer_text() {
    let mut machine =
        factory::create_standard_machine(['A', 'A', 'A'], ['A', 'A', 'A'], "AB CD EF GH IJ")
            .expect("Machine should be creatable");

    let long_text = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
    let encrypted = machine.encrypt(long_text);

    // Reset
    machine.set_rotor_positions(['A', 'A', 'A']);

    let decrypted = machine.decrypt(&encrypted);
    assert_eq!(clean_text(&decrypted), clean_text(long_text));
}

/// Tests rotor stepping
#[test]
fn test_rotor_stepping() {
    let mut machine = factory::create_standard_machine(['A', 'A', 'A'], ['A', 'A', 'A'], "")
        .expect("Machine should be creatable");

    let char1 = machine.encrypt_char('A');
    let char2 = machine.encrypt_char('A');

    // Same input should produce different outputs after rotor stepping
    assert_ne!(char1, char2);
}

/// Tests configuration information
#[test]
fn test_configuration_info() {
    let machine = factory::create_standard_machine(['B', 'C', 'D'], ['E', 'F', 'G'], "AB CD")
        .expect("Machine should be creatable");

    let config = machine.get_configuration_info();

    // Check that all important information is included
    assert!(config.contains("I II III")); // Rotor types
    assert!(config.contains("E F G")); // Ring settings
    assert!(config.contains("B C D")); // Positions
    assert!(config.contains("B")); // Reflector
    assert!(config.contains("AB CD")); // Plugboard
}
