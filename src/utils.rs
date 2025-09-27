//! Hilfsfunktionen für den Enigma-Simulator
//! 
//! Dieses Modul enthält verschiedene Utility-Funktionen für die Enigma-Maschine,
//! wie z.B. Alphabet-Konvertierung und Validierung.

/// Konvertiert einen Buchstaben (A-Z) zu einem Index (0-25)
/// 
/// # Arguments
/// * `letter` - Der Buchstabe (A-Z)
/// 
/// # Returns
/// * `Some(index)` - Der entsprechende Index (0-25)
/// * `None` - Wenn der Buchstabe ungültig ist
pub fn letter_to_index(letter: char) -> Option<usize> {
    if letter.is_ascii_alphabetic() {
        Some((letter.to_ascii_uppercase() as u8 - b'A') as usize)
    } else {
        None
    }
}

/// Konvertiert einen Index (0-25) zu einem Buchstaben (A-Z)
/// 
/// # Arguments
/// * `index` - Der Index (0-25)
/// 
/// # Returns
/// * `Some(letter)` - Der entsprechende Buchstabe (A-Z)
/// * `None` - Wenn der Index ungültig ist
pub fn index_to_letter(index: usize) -> Option<char> {
    if index < 26 {
        Some((b'A' + index as u8) as char)
    } else {
        None
    }
}

/// Validiert einen Text, um sicherzustellen, dass er nur gültige Buchstaben enthält
/// 
/// # Arguments
/// * `text` - Der zu validierende Text
/// 
/// # Returns
/// * `true` - Wenn der Text nur Buchstaben enthält
/// * `false` - Wenn der Text ungültige Zeichen enthält
pub fn is_valid_text(text: &str) -> bool {
    text.chars().all(|c| c.is_ascii_alphabetic() || c.is_whitespace())
}

/// Bereinigt einen Text, indem nur Buchstaben beibehalten werden
/// 
/// # Arguments
/// * `text` - Der zu bereinigende Text
/// 
/// # Returns
/// * Der bereinigte Text (nur Buchstaben, in Großbuchstaben)
pub fn clean_text(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Erstellt einen zufälligen Schlüssel für das Steckerbrett
/// 
/// # Returns
/// * Ein String mit zufällig verbundenen Buchstabenpaaren
pub fn generate_random_plugboard() -> String {
    use std::collections::HashSet;
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let mut used = HashSet::new();
    let mut connections = Vec::new();
    
    let letters: Vec<char> = (b'A'..=b'Z').map(|b| b as char).collect();
    
    for &letter in &letters {
        if used.contains(&letter) {
            continue;
        }
        
        let available: Vec<char> = letters.iter()
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
