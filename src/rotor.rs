//! Rotor implementation for the Enigma machine
//!
//! This module defines the structure and functionality of Enigma rotors.
//! Each rotor has a wiring, a ring setting, and a position.

use crate::utils::{index_to_letter, letter_to_index};
use log::{debug, trace};

/// Represents a single Enigma rotor
#[derive(Debug, Clone)]
pub struct Rotor {
    /// The rotor's wiring (substitution of A-Z)
    pub wiring: [usize; 26],
    /// The reverse wiring for backward direction
    pub reverse_wiring: [usize; 26],
    /// The ring setting
    pub ring_setting: usize,
    /// The current position
    pub position: usize,
    /// The letter at the notch (for advancement)
    pub notch: usize,
    /// The name of the rotor (e.g. "I", "II", "III")
    pub name: String,
}

impl Rotor {
    /// Creates a new rotor with the specified parameters
    ///
    /// # Arguments
    /// * `wiring` - The wiring as a string (e.g. "EKMFLGDQVZNTOWYHXUSPAIBRCJ")
    /// * `notch` - The notch letter
    /// * `name` - The name of the rotor
    /// * `ring_setting` - The ring setting (0-25)
    /// * `position` - The position (0-25)
    ///
    /// # Returns
    /// * `Result<Rotor, String>` - The created rotor or an error
    pub fn new(
        wiring: &str,
        notch: char,
        name: &str,
        ring_setting: usize,
        position: usize,
    ) -> Result<Self, String> {
        if wiring.len() != 26 {
            return Err("Wiring must be exactly 26 characters long".to_string());
        }

        if ring_setting >= 26 || position >= 26 {
            return Err("Ring setting and position must be between 0 and 25".to_string());
        }

        let notch_index = letter_to_index(notch)
            .ok_or_else(|| format!("Ungültiger Kerbenbuchstabe: {}", notch))?;

        let mut wiring_array = [0; 26];
        let mut reverse_wiring = [0; 26];

        for (i, ch) in wiring.chars().enumerate() {
            let target = letter_to_index(ch)
                .ok_or_else(|| format!("Ungültiges Zeichen in Verdrahtung: {}", ch))?;
            wiring_array[i] = target;
            reverse_wiring[target] = i;
        }

        Ok(Rotor {
            wiring: wiring_array,
            reverse_wiring,
            ring_setting,
            position,
            notch: notch_index,
            name: name.to_string(),
        })
    }

    /// Encrypts a character in forward direction
    ///
    /// # Arguments
    /// * `input` - Das Eingabezeichen
    ///
    /// # Returns
    /// * The encrypted character
    pub fn forward(&self, input: char) -> char {
        let input_index = letter_to_index(input).unwrap_or(0) as i32;
        let position = self.position as i32;
        let ring_setting = self.ring_setting as i32;
        let adjusted_input = ((input_index + position - ring_setting + 26) % 26) as usize;
        let output_index =
            ((self.wiring[adjusted_input] as i32 + ring_setting - position + 26) % 26) as usize;

        trace!(
            "Rotor {} forward: {} -> {} (pos: {}, ring: {})",
            self.name,
            input,
            index_to_letter(output_index).unwrap_or('A'),
            self.position,
            self.ring_setting
        );

        index_to_letter(output_index).unwrap_or('A')
    }

    /// Encrypts a character in backward direction
    ///
    /// # Arguments
    /// * `input` - Das Eingabezeichen
    ///
    /// # Returns
    /// * The encrypted character
    pub fn backward(&self, input: char) -> char {
        let input_index = letter_to_index(input).unwrap_or(0) as i32;
        let position = self.position as i32;
        let ring_setting = self.ring_setting as i32;
        let adjusted_input = ((input_index + position - ring_setting + 26) % 26) as usize;
        let output_index = ((self.reverse_wiring[adjusted_input] as i32 + ring_setting - position
            + 26)
            % 26) as usize;

        trace!(
            "Rotor {} backward: {} -> {} (pos: {}, ring: {})",
            self.name,
            input,
            index_to_letter(output_index).unwrap_or('A'),
            self.position,
            self.ring_setting
        );

        index_to_letter(output_index).unwrap_or('A')
    }

    /// Dreht den Rotor um eine Position weiter
    ///
    /// # Returns
    /// * `true` - If the rotor has passed the notch (trigger advancement)
    /// * `false` - Normale Drehung
    pub fn step(&mut self) -> bool {
        let was_at_notch = self.position == self.notch;
        self.position = (self.position + 1) % 26;

        debug!(
            "Rotor {} gedreht: neue Position {}",
            self.name, self.position
        );
        was_at_notch
    }

    /// Setzt die Position des Rotors
    ///
    /// # Arguments
    /// * `position` - Die neue Position (0-25)
    pub fn set_position(&mut self, position: usize) {
        if position < 26 {
            self.position = position;
            debug!("Rotor {} Position gesetzt auf {}", self.name, position);
        }
    }

    /// Setzt die Ringstellung des Rotors
    ///
    /// # Arguments
    /// * `ring_setting` - Die neue Ringstellung (0-25)
    pub fn set_ring_setting(&mut self, ring_setting: usize) {
        if ring_setting < 26 {
            self.ring_setting = ring_setting;
            debug!(
                "Rotor {} Ringstellung gesetzt auf {}",
                self.name, ring_setting
            );
        }
    }

    /// Returns the current position as a letter
    ///
    /// # Returns
    /// * Der Buchstabe der aktuellen Position
    pub fn get_position_char(&self) -> char {
        index_to_letter(self.position).unwrap_or('A')
    }

    /// Returns the ring setting as a letter
    ///
    /// # Returns
    /// * Der Buchstabe der Ringstellung
    pub fn get_ring_setting_char(&self) -> char {
        index_to_letter(self.ring_setting).unwrap_or('A')
    }
}

/// Vordefinierte historische Rotoren
pub mod rotors {
    use super::Rotor;

    /// Erstellt Rotor I
    pub fn rotor_i(ring_setting: usize, position: usize) -> Result<Rotor, String> {
        Rotor::new(
            "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
            'Q',
            "I",
            ring_setting,
            position,
        )
    }

    /// Erstellt Rotor II
    pub fn rotor_ii(ring_setting: usize, position: usize) -> Result<Rotor, String> {
        Rotor::new(
            "AJDKSIRUXBLHWTMCQGZNPYFVOE",
            'E',
            "II",
            ring_setting,
            position,
        )
    }

    /// Erstellt Rotor III
    pub fn rotor_iii(ring_setting: usize, position: usize) -> Result<Rotor, String> {
        Rotor::new(
            "BDFHJLCPRTXVZNYEIWGAKMUSQO",
            'V',
            "III",
            ring_setting,
            position,
        )
    }

    /// Erstellt Rotor IV
    pub fn rotor_iv(ring_setting: usize, position: usize) -> Result<Rotor, String> {
        Rotor::new(
            "ESOVPZJAYQUIRHXLNFTGKDCMWB",
            'J',
            "IV",
            ring_setting,
            position,
        )
    }

    /// Erstellt Rotor V
    pub fn rotor_v(ring_setting: usize, position: usize) -> Result<Rotor, String> {
        Rotor::new(
            "VZBRGITYUPSDNHLXAWMJQOFECK",
            'Z',
            "V",
            ring_setting,
            position,
        )
    }

    /// Returns all available rotors
    pub fn available_rotors() -> Vec<(&'static str, fn(usize, usize) -> Result<Rotor, String>)> {
        vec![
            ("I", rotor_i as fn(usize, usize) -> Result<Rotor, String>),
            ("II", rotor_ii as fn(usize, usize) -> Result<Rotor, String>),
            (
                "III",
                rotor_iii as fn(usize, usize) -> Result<Rotor, String>,
            ),
            ("IV", rotor_iv as fn(usize, usize) -> Result<Rotor, String>),
            ("V", rotor_v as fn(usize, usize) -> Result<Rotor, String>),
        ]
    }
}
