//! Reflector implementation for the Enigma machine
//!
//! The reflector is a fixed element that routes the signal back to the rotors.
//! It implements a fixed permutation of the alphabet.

use crate::utils::{index_to_letter, letter_to_index};
use log::trace;

/// Represents the Enigma reflector
#[derive(Debug, Clone)]
pub struct Reflector {
    /// Die Verdrahtung des Reflektors
    pub wiring: [usize; 26],
    /// Der Name des Reflektors (z.B. "A", "B", "C")
    pub name: String,
}

impl Reflector {
    /// Erstellt einen neuen Reflektor mit der angegebenen Verdrahtung
    ///
    /// # Arguments
    /// * `wiring` - Die Verdrahtung als String (z.B. "EJMZALYXVBWFCRQUONTSPIKHGD")
    /// * `name` - Der Name des Reflektors
    ///
    /// # Returns
    /// * `Result<Reflector, String>` - Der erstellte Reflektor oder ein Fehler
    pub fn new(wiring: &str, name: &str) -> Result<Self, String> {
        if wiring.len() != 26 {
            return Err("Verdrahtung muss genau 26 Zeichen lang sein".to_string());
        }

        let mut wiring_array = [0; 26];

        for (i, ch) in wiring.chars().enumerate() {
            let target = letter_to_index(ch)
                .ok_or_else(|| format!("Ungültiges Zeichen in Verdrahtung: {}", ch))?;
            wiring_array[i] = target;
        }

        // Validiere, dass es sich um eine echte Permutation handelt
        if !Self::is_valid_permutation(&wiring_array) {
            return Err("Verdrahtung muss eine gültige Permutation sein (jeder Buchstabe muss genau einmal als Ziel auftreten)".to_string());
        }

        Ok(Reflector {
            wiring: wiring_array,
            name: name.to_string(),
        })
    }

    /// Reflektiert ein Zeichen
    ///
    /// # Arguments
    /// * `input` - Das Eingabezeichen
    ///
    /// # Returns
    /// * Das reflektierte Zeichen
    pub fn reflect(&self, input: char) -> char {
        let input_index = letter_to_index(input).unwrap_or(0);
        let output_index = self.wiring[input_index];

        trace!(
            "Reflektor {}: {} -> {}",
            self.name,
            input,
            index_to_letter(output_index).unwrap_or('A')
        );

        index_to_letter(output_index).unwrap_or('A')
    }

    /// Checks whether the wiring is a valid permutation
    ///
    /// # Arguments
    /// * `wiring` - The wiring to check
    ///
    /// # Returns
    /// * `true` - If it is a valid permutation
    /// * `false` - If it is not a valid permutation
    fn is_valid_permutation(wiring: &[usize; 26]) -> bool {
        // Check that each index appears exactly once as a target
        let mut targets = [false; 26];
        for &target in wiring.iter() {
            if target >= 26 {
                return false;
            }
            if targets[target] {
                return false; // Doppeltes Ziel
            }
            targets[target] = true;
        }

        // Check that each index appears exactly once as a source
        for i in 0..26 {
            if !targets[i] {
                return false;
            }
        }

        true
    }
}

/// Vordefinierte historische Reflektoren
pub mod reflectors {
    use super::Reflector;

    /// Erstellt Reflektor A
    pub fn reflector_a() -> Result<Reflector, String> {
        Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD", "A")
    }

    /// Erstellt Reflektor B
    pub fn reflector_b() -> Result<Reflector, String> {
        Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", "B")
    }

    /// Erstellt Reflektor C
    pub fn reflector_c() -> Result<Reflector, String> {
        Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", "C")
    }

    /// Returns all available reflectors
    pub fn available_reflectors() -> Vec<(&'static str, fn() -> Result<Reflector, String>)> {
        vec![
            ("A", reflector_a as fn() -> Result<Reflector, String>),
            ("B", reflector_b as fn() -> Result<Reflector, String>),
            ("C", reflector_c as fn() -> Result<Reflector, String>),
        ]
    }
}
