//! Utility functions for the Enigma simulator
//!
//! This module contains various utility functions for the Enigma machine,
//! such as alphabet conversion and validation.

/// Converts a letter (A-Z) to an index (0-25)
///
/// # Arguments
/// * `letter` - The letter (A-Z)
///
/// # Returns
/// * `Some(index)` - The corresponding index (0-25)
/// * `None` - If the letter is invalid
pub fn letter_to_index(letter: char) -> Option<usize> {
    if letter.is_ascii_alphabetic() {
        Some((letter.to_ascii_uppercase() as u8 - b'A') as usize)
    } else {
        None
    }
}

/// Converts an index (0-25) to a letter (A-Z)
///
/// # Arguments
/// * `index` - The index (0-25)
///
/// # Returns
/// * `Some(letter)` - The corresponding letter (A-Z)
/// * `None` - If the index is invalid
pub fn index_to_letter(index: usize) -> Option<char> {
    if index < 26 {
        Some((b'A' + index as u8) as char)
    } else {
        None
    }
}

/// Validates text to ensure it only contains valid letters
///
/// # Arguments
/// * `text` - The text to validate
///
/// # Returns
/// * `true` - If the text only contains letters
/// * `false` - If the text contains invalid characters
pub fn is_valid_text(text: &str) -> bool {
    text.chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_whitespace())
}

/// Cleans text by keeping only letters
///
/// # Arguments
/// * `text` - The text to clean
///
/// # Returns
/// * The cleaned text (only letters, in uppercase)
pub fn clean_text(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Creates a random key for the plugboard
///
/// # Returns
/// * A string with randomly connected letter pairs
pub fn generate_random_plugboard() -> String {
    use rand::Rng;
    use std::collections::HashSet;

    let mut rng = rand::thread_rng();
    let mut used = HashSet::new();
    let mut connections = Vec::new();

    let letters: Vec<char> = (b'A'..=b'Z').map(|b| b as char).collect();

    for &letter in &letters {
        if used.contains(&letter) {
            continue;
        }

        let available: Vec<char> = letters
            .iter()
            .filter(|&&l| !used.contains(&l))
            .copied()
            .collect();

        if available.len() > 1 {
            let partner = available[rng.gen_range(0..available.len())];
            if partner != letter {
                connections.push(format!("{}{}", letter, partner));
                used.insert(letter);
                used.insert(partner);
            }
        }
    }

    connections.join(" ")
}
