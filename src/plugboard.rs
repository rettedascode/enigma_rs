//! Steckerbrett-Implementierung für die Enigma-Maschine
//! 
//! Das Steckerbrett ermöglicht das Vertauschen von Buchstabenpaaren vor und nach
//! der Verarbeitung durch die Rotoren.

use log::trace;
use crate::utils::{letter_to_index, index_to_letter};

/// Repräsentiert das Enigma-Steckerbrett
#[derive(Debug, Clone)]
pub struct Plugboard {
    /// Die Verbindungen des Steckerbretts (jeder Index zeigt auf den verbundenen Buchstaben)
    connections: [Option<usize>; 26],
    /// Die Anzahl der aktiven Verbindungen
    pub connection_count: usize,
}

impl Plugboard {
    /// Erstellt ein neues, leeres Steckerbrett
    /// 
    /// # Returns
    /// * Ein neues Steckerbrett ohne Verbindungen
    pub fn new() -> Self {
        Plugboard {
            connections: [None; 26],
            connection_count: 0,
        }
    }
    
    /// Erstellt ein Steckerbrett aus einem Verbindungsstring
    /// 
    /// # Arguments
    /// * `connections` - String mit Verbindungen (z.B. "AB CD EF")
    /// 
    /// # Returns
    /// * `Result<Plugboard, String>` - Das erstellte Steckerbrett oder ein Fehler
    pub fn from_string(connections: &str) -> Result<Self, String> {
        let mut plugboard = Plugboard::new();
        
        if connections.trim().is_empty() {
            return Ok(plugboard);
        }
        
        for connection in connections.split_whitespace() {
            if connection.len() != 2 {
                return Err(format!("Verbindung '{}' muss genau 2 Zeichen lang sein", connection));
            }
            
            let chars: Vec<char> = connection.chars().collect();
            let first = chars[0];
            let second = chars[1];
            
            if !first.is_ascii_alphabetic() || !second.is_ascii_alphabetic() {
                return Err(format!("Verbindung '{}' darf nur Buchstaben enthalten", connection));
            }
            
            plugboard.add_connection(first, second)?;
        }
        
        Ok(plugboard)
    }
    
    /// Fügt eine Verbindung zwischen zwei Buchstaben hinzu
    /// 
    /// # Arguments
    /// * `first` - Der erste Buchstabe
    /// * `second` - Der zweite Buchstabe
    /// 
    /// # Returns
    /// * `Result<(), String>` - Erfolg oder Fehler
    pub fn add_connection(&mut self, first: char, second: char) -> Result<(), String> {
        if first == second {
            return Err("Ein Buchstabe kann nicht mit sich selbst verbunden werden".to_string());
        }
        
        let first_index = letter_to_index(first)
            .ok_or_else(|| format!("Ungültiger Buchstabe: {}", first))?;
        let second_index = letter_to_index(second)
            .ok_or_else(|| format!("Ungültiger Buchstabe: {}", second))?;
        
        // Überprüfe, ob einer der Buchstaben bereits verbunden ist
        if self.connections[first_index].is_some() {
            return Err(format!("Buchstabe {} ist bereits verbunden", first));
        }
        if self.connections[second_index].is_some() {
            return Err(format!("Buchstabe {} ist bereits verbunden", second));
        }
        
        // Füge die Verbindung hinzu
        self.connections[first_index] = Some(second_index);
        self.connections[second_index] = Some(first_index);
        self.connection_count += 1;
        
        trace!("Steckerbrett-Verbindung hinzugefügt: {} <-> {}", first, second);
        Ok(())
    }
    
    /// Entfernt eine Verbindung zwischen zwei Buchstaben
    /// 
    /// # Arguments
    /// * `first` - Der erste Buchstabe der zu entfernenden Verbindung
    /// 
    /// # Returns
    /// * `Result<(), String>` - Erfolg oder Fehler
    pub fn remove_connection(&mut self, first: char) -> Result<(), String> {
        let first_index = letter_to_index(first)
            .ok_or_else(|| format!("Ungültiger Buchstabe: {}", first))?;
        
        if let Some(second_index) = self.connections[first_index] {
            self.connections[first_index] = None;
            self.connections[second_index] = None;
            self.connection_count -= 1;
            
            let second = index_to_letter(second_index).unwrap_or('A');
            trace!("Steckerbrett-Verbindung entfernt: {} <-> {}", first, second);
            Ok(())
        } else {
            Err(format!("Keine Verbindung für Buchstabe {} gefunden", first))
        }
    }
    
    /// Verarbeitet ein Zeichen durch das Steckerbrett
    /// 
    /// # Arguments
    /// * `input` - Das Eingabezeichen
    /// 
    /// # Returns
    /// * Das verarbeitete Zeichen
    pub fn process(&self, input: char) -> char {
        let input_index = letter_to_index(input).unwrap_or(0);
        
        if let Some(output_index) = self.connections[input_index] {
            let output = index_to_letter(output_index).unwrap_or(input);
            trace!("Steckerbrett: {} -> {}", input, output);
            output
        } else {
            trace!("Steckerbrett: {} -> {} (keine Verbindung)", input, input);
            input
        }
    }
    
    /// Gibt alle aktiven Verbindungen als String zurück
    /// 
    /// # Returns
    /// * String mit allen Verbindungen (z.B. "AB CD EF")
    pub fn get_connections_string(&self) -> String {
        let mut connections = Vec::new();
        let mut used = [false; 26];
        
        for (i, &connection) in self.connections.iter().enumerate() {
            if let Some(target) = connection {
                if !used[i] && !used[target] {
                    let first = index_to_letter(i).unwrap_or('A');
                    let second = index_to_letter(target).unwrap_or('A');
                    connections.push(format!("{}{}", first, second));
                    used[i] = true;
                    used[target] = true;
                }
            }
        }
        
        connections.join(" ")
    }
    
    /// Gibt die Anzahl der aktiven Verbindungen zurück
    /// 
    /// # Returns
    /// * Die Anzahl der Verbindungen
    pub fn get_connection_count(&self) -> usize {
        self.connection_count
    }
    
    /// Überprüft, ob ein Buchstabe verbunden ist
    /// 
    /// # Arguments
    /// * `letter` - Der zu überprüfende Buchstabe
    /// 
    /// # Returns
    /// * `true` - Wenn der Buchstabe verbunden ist
    /// * `false` - Wenn der Buchstabe nicht verbunden ist
    pub fn is_connected(&self, letter: char) -> bool {
        if let Some(index) = letter_to_index(letter) {
            self.connections[index].is_some()
        } else {
            false
        }
    }
    
    /// Löscht alle Verbindungen
    pub fn clear(&mut self) {
        self.connections = [None; 26];
        self.connection_count = 0;
        trace!("Steckerbrett geleert");
    }
}
