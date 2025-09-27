//! Enigma-Maschine Hauptimplementierung
//! 
//! Dieses Modul enthält die Hauptlogik der Enigma-Maschine, die alle Komponenten
//! (Rotoren, Reflektor, Steckerbrett) zusammenführt.

use log::{debug, info, trace};
use crate::rotor::Rotor;
use crate::reflector::Reflector;
use crate::plugboard::Plugboard;
use crate::utils::{letter_to_index, clean_text};

/// Repräsentiert eine vollständige Enigma-Maschine
#[derive(Debug)]
pub struct EnigmaMachine {
    /// Die drei Rotoren (links, mitte, rechts)
    pub rotors: [Rotor; 3],
    /// Der Reflektor
    pub reflector: Reflector,
    /// Das Steckerbrett
    pub plugboard: Plugboard,
}

impl EnigmaMachine {
    /// Erstellt eine neue Enigma-Maschine mit den angegebenen Komponenten
    /// 
    /// # Arguments
    /// * `rotors` - Array der drei Rotoren
    /// * `reflector` - Der Reflektor
    /// * `plugboard` - Das Steckerbrett
    /// 
    /// # Returns
    /// * Eine neue Enigma-Maschine
    pub fn new(rotors: [Rotor; 3], reflector: Reflector, plugboard: Plugboard) -> Self {
        EnigmaMachine {
            rotors,
            reflector,
            plugboard,
        }
    }
    
    /// Verschlüsselt einen einzelnen Buchstaben
    /// 
    /// # Arguments
    /// * `input` - Das zu verschlüsselnde Zeichen
    /// 
    /// # Returns
    /// * Das verschlüsselte Zeichen
    pub fn encrypt_char(&mut self, input: char) -> char {
        debug!("=== Verschlüsselung von '{}' ===", input);
        
        // 1. Steckerbrett (Vorwärts)
        let after_plugboard = self.plugboard.process(input);
        trace!("Nach Steckerbrett (vorwärts): {} -> {}", input, after_plugboard);
        
        // 2. Rotoren drehen (vor der Verschlüsselung)
        self.step_rotors();
        
        // 3. Durch die Rotoren (vorwärts)
        let mut signal = after_plugboard;
        for (i, rotor) in self.rotors.iter().enumerate() {
            signal = rotor.forward(signal);
            trace!("Nach Rotor {} (vorwärts): {}", i + 1, signal);
        }
        
        // 4. Reflektor
        signal = self.reflector.reflect(signal);
        trace!("Nach Reflektor: {}", signal);
        
        // 5. Durch die Rotoren (rückwärts)
        for (i, rotor) in self.rotors.iter().rev().enumerate() {
            signal = rotor.backward(signal);
            trace!("Nach Rotor {} (rückwärts): {}", 3 - i, signal);
        }
        
        // 6. Steckerbrett (Rückwärts)
        let final_output = self.plugboard.process(signal);
        trace!("Nach Steckerbrett (rückwärts): {} -> {}", signal, final_output);
        
        debug!("=== Verschlüsselung abgeschlossen: {} -> {} ===", input, final_output);
        final_output
    }
    
    /// Verschlüsselt einen kompletten Text
    /// 
    /// # Arguments
    /// * `text` - Der zu verschlüsselnde Text
    /// 
    /// # Returns
    /// * Der verschlüsselte Text
    pub fn encrypt(&mut self, text: &str) -> String {
        info!("Starte Verschlüsselung von: '{}'", text);
        let clean_input = clean_text(text);
        info!("Bereinigter Input: '{}'", clean_input);
        
        let mut result = String::new();
        for (i, ch) in clean_input.chars().enumerate() {
            let encrypted = self.encrypt_char(ch);
            result.push(encrypted);
            
            if i > 0 && i % 5 == 4 {
                result.push(' ');
            }
        }
        
        info!("Verschlüsselung abgeschlossen: '{}'", result);
        result
    }
    
    /// Entschlüsselt einen Text (gleiche Logik wie Verschlüsselung)
    /// 
    /// # Arguments
    /// * `text` - Der zu entschlüsselnde Text
    /// 
    /// # Returns
    /// * Der entschlüsselte Text
    pub fn decrypt(&mut self, text: &str) -> String {
        info!("Starte Entschlüsselung von: '{}'", text);
        let clean_input = clean_text(text);
        info!("Bereinigter Input: '{}'", clean_input);
        
        let mut result = String::new();
        for (i, ch) in clean_input.chars().enumerate() {
            let decrypted = self.encrypt_char(ch); // Gleiche Logik!
            result.push(decrypted);
            
            if i > 0 && i % 5 == 4 {
                result.push(' ');
            }
        }
        
        info!("Entschlüsselung abgeschlossen: '{}'", result);
        result
    }
    
    /// Dreht die Rotoren entsprechend der Enigma-Regeln
    fn step_rotors(&mut self) {
        // Rechter Rotor dreht sich immer
        let right_rotor_notched = self.rotors[2].step();
        
        // Mittlerer Rotor dreht sich, wenn der rechte an der Kerbe ist
        let middle_rotor_notched = if right_rotor_notched {
            self.rotors[1].step()
        } else {
            // Oder wenn der mittlere selbst an der Kerbe ist (Doppelschritt)
            if self.rotors[1].position == self.rotors[1].notch {
                self.rotors[1].step()
            } else {
                false
            }
        };
        
        // Linker Rotor dreht sich, wenn der mittlere an der Kerbe ist
        if middle_rotor_notched {
            self.rotors[0].step();
        }
        
        trace!(
            "Rotorenpositionen: {} {} {}",
            self.rotors[0].get_position_char(),
            self.rotors[1].get_position_char(),
            self.rotors[2].get_position_char()
        );
    }
    
    /// Setzt die Rotorpositionen
    /// 
    /// # Arguments
    /// * `positions` - Array der drei Positionen (links, mitte, rechts)
    pub fn set_rotor_positions(&mut self, positions: [char; 3]) {
        for (i, &pos) in positions.iter().enumerate() {
            if let Some(index) = letter_to_index(pos) {
                self.rotors[i].set_position(index);
            }
        }
        info!("Rotorpositionen gesetzt auf: {} {} {}", positions[0], positions[1], positions[2]);
    }
    
    /// Setzt die Ringstellungen
    /// 
    /// # Arguments
    /// * `ring_settings` - Array der drei Ringstellungen (links, mitte, rechts)
    pub fn set_ring_settings(&mut self, ring_settings: [char; 3]) {
        for (i, &ring) in ring_settings.iter().enumerate() {
            if let Some(index) = letter_to_index(ring) {
                self.rotors[i].set_ring_setting(index);
            }
        }
        info!("Ringstellungen gesetzt auf: {} {} {}", ring_settings[0], ring_settings[1], ring_settings[2]);
    }
    
    /// Gibt die aktuellen Rotorpositionen zurück
    /// 
    /// # Returns
    /// * Array der aktuellen Positionen
    pub fn get_rotor_positions(&self) -> [char; 3] {
        [
            self.rotors[0].get_position_char(),
            self.rotors[1].get_position_char(),
            self.rotors[2].get_position_char(),
        ]
    }
    
    /// Gibt die aktuellen Ringstellungen zurück
    /// 
    /// # Returns
    /// * Array der aktuellen Ringstellungen
    pub fn get_ring_settings(&self) -> [char; 3] {
        [
            self.rotors[0].get_ring_setting_char(),
            self.rotors[1].get_ring_setting_char(),
            self.rotors[2].get_ring_setting_char(),
        ]
    }
    
    /// Gibt Informationen über die Maschinenkonfiguration zurück
    /// 
    /// # Returns
    /// * String mit Konfigurationsinformationen
    pub fn get_configuration_info(&self) -> String {
        format!(
            "Rotoren: {} {} {}\nRingstellungen: {} {} {}\nPositionen: {} {} {}\nReflektor: {}\nSteckerbrett: {}",
            self.rotors[0].name,
            self.rotors[1].name,
            self.rotors[2].name,
            self.get_ring_settings()[0],
            self.get_ring_settings()[1],
            self.get_ring_settings()[2],
            self.get_rotor_positions()[0],
            self.get_rotor_positions()[1],
            self.get_rotor_positions()[2],
            self.reflector.name,
            self.plugboard.get_connections_string()
        )
    }
}

/// Factory-Funktionen für häufige Enigma-Konfigurationen
pub mod factory {
    use super::*;
    use crate::rotor::rotors::{rotor_i, rotor_ii, rotor_iii, rotor_iv, rotor_v};
    use crate::reflector::reflectors::{reflector_a, reflector_b, reflector_c};
    
    /// Erstellt eine Standard-Enigma-Maschine (Rotoren I, II, III, Reflektor B)
    /// 
    /// # Arguments
    /// * `rotor_positions` - Die Rotorpositionen [links, mitte, rechts]
    /// * `ring_settings` - Die Ringstellungen [links, mitte, rechts]
    /// * `plugboard_connections` - Die Steckerbrett-Verbindungen
    /// 
    /// # Returns
    /// * `Result<EnigmaMachine, String>` - Die erstellte Maschine oder ein Fehler
    pub fn create_standard_machine(
        rotor_positions: [char; 3],
        ring_settings: [char; 3],
        plugboard_connections: &str,
    ) -> Result<EnigmaMachine, String> {
        let rotors = [
            rotor_i(ring_settings[0] as usize - b'A' as usize, rotor_positions[0] as usize - b'A' as usize)?,
            rotor_ii(ring_settings[1] as usize - b'A' as usize, rotor_positions[1] as usize - b'A' as usize)?,
            rotor_iii(ring_settings[2] as usize - b'A' as usize, rotor_positions[2] as usize - b'A' as usize)?,
        ];
        
        let reflector = reflector_b()?;
        let plugboard = Plugboard::from_string(plugboard_connections)?;
        
        Ok(EnigmaMachine::new(rotors, reflector, plugboard))
    }
    
    /// Erstellt eine Enigma-Maschine mit benutzerdefinierten Rotoren
    /// 
    /// # Arguments
    /// * `rotor_types` - Array der Rotortypen ["I", "II", "III"]
    /// * `rotor_positions` - Die Rotorpositionen
    /// * `ring_settings` - Die Ringstellungen
    /// * `reflector_type` - Der Reflektortyp ("A", "B", oder "C")
    /// * `plugboard_connections` - Die Steckerbrett-Verbindungen
    /// 
    /// # Returns
    /// * `Result<EnigmaMachine, String>` - Die erstellte Maschine oder ein Fehler
    pub fn create_custom_machine(
        rotor_types: [&str; 3],
        rotor_positions: [char; 3],
        ring_settings: [char; 3],
        reflector_type: &str,
        plugboard_connections: &str,
    ) -> Result<EnigmaMachine, String> {
        let _rotor_creators = [
            rotor_i, rotor_ii, rotor_iii, rotor_iv, rotor_v
        ];
        
        let mut rotors = Vec::new();
        for rotor_type in rotor_types.iter() {
            let creator = match *rotor_type {
                "I" => rotor_i,
                "II" => rotor_ii,
                "III" => rotor_iii,
                "IV" => rotor_iv,
                "V" => rotor_v,
                _ => return Err(format!("Unbekannter Rotortyp: {}", rotor_type)),
            };
            
            let ring_idx = rotor_positions.len() - 1 - rotors.len();
            let pos_idx = ring_idx;
            rotors.push(creator(
                ring_settings[pos_idx] as usize - b'A' as usize,
                rotor_positions[pos_idx] as usize - b'A' as usize
            )?);
        }
        
        let reflector = match reflector_type {
            "A" => reflector_a(),
            "B" => reflector_b(),
            "C" => reflector_c(),
            _ => return Err(format!("Unbekannter Reflektortyp: {}", reflector_type)),
        }?;
        
        let plugboard = Plugboard::from_string(plugboard_connections)?;
        
        Ok(EnigmaMachine::new([rotors[0].clone(), rotors[1].clone(), rotors[2].clone()], reflector, plugboard))
    }
}
