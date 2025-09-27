//! Rotor-Implementierung für die Enigma-Maschine
//!
//! Dieses Modul definiert die Struktur und Funktionalität der Enigma-Rotoren.
//! Jeder Rotor hat eine Verdrahtung, eine Ringstellung und eine Grundstellung.

use crate::utils::{index_to_letter, letter_to_index};
use log::{debug, trace};

/// Repräsentiert einen einzelnen Enigma-Rotor
#[derive(Debug, Clone)]
pub struct Rotor {
    /// Die Verdrahtung des Rotors (Substitution von A-Z)
    pub wiring: [usize; 26],
    /// Die Umkehr-Verdrahtung für die Rückwärtsrichtung
    pub reverse_wiring: [usize; 26],
    /// Die Ringstellung (Ringstellung)
    pub ring_setting: usize,
    /// Die aktuelle Grundstellung
    pub position: usize,
    /// Der Buchstabe an der Kerbe (für die Weiterleitung)
    pub notch: usize,
    /// Der Name des Rotors (z.B. "I", "II", "III")
    pub name: String,
}

impl Rotor {
    /// Erstellt einen neuen Rotor mit den angegebenen Parametern
    ///
    /// # Arguments
    /// * `wiring` - Die Verdrahtung als String (z.B. "EKMFLGDQVZNTOWYHXUSPAIBRCJ")
    /// * `notch` - Der Kerbenbuchstabe
    /// * `name` - Der Name des Rotors
    /// * `ring_setting` - Die Ringstellung (0-25)
    /// * `position` - Die Grundstellung (0-25)
    ///
    /// # Returns
    /// * `Result<Rotor, String>` - Der erstellte Rotor oder ein Fehler
    pub fn new(
        wiring: &str,
        notch: char,
        name: &str,
        ring_setting: usize,
        position: usize,
    ) -> Result<Self, String> {
        if wiring.len() != 26 {
            return Err("Verdrahtung muss genau 26 Zeichen lang sein".to_string());
        }

        if ring_setting >= 26 || position >= 26 {
            return Err("Ringstellung und Position müssen zwischen 0 und 25 liegen".to_string());
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

    /// Verschlüsselt ein Zeichen in Vorwärtsrichtung
    ///
    /// # Arguments
    /// * `input` - Das Eingabezeichen
    ///
    /// # Returns
    /// * Das verschlüsselte Zeichen
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

    /// Verschlüsselt ein Zeichen in Rückwärtsrichtung
    ///
    /// # Arguments
    /// * `input` - Das Eingabezeichen
    ///
    /// # Returns
    /// * Das verschlüsselte Zeichen
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
    /// * `true` - Wenn der Rotor an der Kerbe vorbeigedreht ist (Weiterleitung auslösen)
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

    /// Gibt die aktuelle Position als Buchstabe zurück
    ///
    /// # Returns
    /// * Der Buchstabe der aktuellen Position
    pub fn get_position_char(&self) -> char {
        index_to_letter(self.position).unwrap_or('A')
    }

    /// Gibt die Ringstellung als Buchstabe zurück
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

    /// Gibt alle verfügbaren Rotoren zurück
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
