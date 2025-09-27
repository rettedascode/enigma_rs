//! GUI-Implementierung mit egui/eframe für den Enigma-Simulator
//! 
//! Dieses Modul enthält die grafische Benutzeroberfläche mit allen
//! Konfigurationsmöglichkeiten und einer detaillierten Log-Anzeige.

use eframe::egui;
use log::Level;
use std::collections::VecDeque;
use crate::machine::{EnigmaMachine, factory};
use crate::utils::clean_text;

/// Maximale Anzahl der Log-Einträge in der GUI
const MAX_LOG_ENTRIES: usize = 1000;

/// Repräsentiert einen Log-Eintrag für die GUI
#[derive(Clone)]
pub struct LogEntry {
    pub level: Level,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Hauptanwendung für die GUI
pub struct EnigmaApp {
    /// Die Enigma-Maschine
    machine: Option<EnigmaMachine>,
    
    // GUI-Zustand
    input_text: String,
    output_text: String,
    
    // Konfiguration
    rotor_types: [String; 3],
    rotor_positions: [String; 3],
    ring_settings: [String; 3],
    reflector_type: String,
    plugboard_connections: String,
    
    // Log-Anzeige
    log_entries: VecDeque<LogEntry>,
    auto_scroll_log: bool,
    log_filter: String,
    
    // UI-Zustand
    show_config: bool,
    show_log: bool,
    operation_mode: OperationMode,
    
    // Verbesserte UI-Elemente
    selected_rotor_preset: String,
    dark_mode: bool,
    show_help: bool,
}

#[derive(PartialEq)]
enum OperationMode {
    Encrypt,
    Decrypt,
}

impl Default for EnigmaApp {
    fn default() -> Self {
        Self {
            machine: None,
            input_text: String::new(),
            output_text: String::new(),
            rotor_types: ["I".to_string(), "II".to_string(), "III".to_string()],
            rotor_positions: ["A".to_string(), "A".to_string(), "A".to_string()],
            ring_settings: ["A".to_string(), "A".to_string(), "A".to_string()],
            reflector_type: "B".to_string(),
            plugboard_connections: String::new(),
            log_entries: VecDeque::new(),
            auto_scroll_log: true,
            log_filter: String::new(),
            show_config: true,
            show_log: true,
            operation_mode: OperationMode::Encrypt,
            selected_rotor_preset: "Standard".to_string(),
            dark_mode: false,
            show_help: false,
        }
    }
}

impl EnigmaApp {
    /// Erstellt eine neue Enigma-GUI-Anwendung
    pub fn new() -> Self {
        let mut app = Self::default();
        app.initialize_machine();
        app
    }
    
    /// Wendet ein Rotor-Preset an
    fn apply_rotor_preset(&mut self, preset: &str) {
        match preset {
            "Standard" => {
                self.rotor_types = ["I".to_string(), "II".to_string(), "III".to_string()];
                self.rotor_positions = ["A".to_string(), "A".to_string(), "A".to_string()];
                self.ring_settings = ["A".to_string(), "A".to_string(), "A".to_string()];
                self.reflector_type = "B".to_string();
                self.plugboard_connections = String::new();
            }
            "Kriegsmarine" => {
                self.rotor_types = ["I".to_string(), "II".to_string(), "III".to_string()];
                self.rotor_positions = ["A".to_string(), "A".to_string(), "A".to_string()];
                self.ring_settings = ["A".to_string(), "A".to_string(), "A".to_string()];
                self.reflector_type = "B".to_string();
                self.plugboard_connections = "AB CD EF GH IJ KL".to_string();
            }
            "Luftwaffe" => {
                self.rotor_types = ["I".to_string(), "II".to_string(), "IV".to_string()];
                self.rotor_positions = ["A".to_string(), "A".to_string(), "A".to_string()];
                self.ring_settings = ["A".to_string(), "A".to_string(), "A".to_string()];
                self.reflector_type = "B".to_string();
                self.plugboard_connections = "AB CD EF".to_string();
            }
            _ => {}
        }
        self.selected_rotor_preset = preset.to_string();
        self.initialize_machine();
    }
    
    /// Rendert einen schönen Header
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(egui::RichText::new("🔐 Enigma-Simulator").size(24.0).color(egui::Color32::from_rgb(70, 130, 180)));
            ui.add_space(5.0);
            ui.label(egui::RichText::new("Historische Verschlüsselungsmaschine").italics().color(egui::Color32::GRAY));
            ui.add_space(10.0);
        });
    }
    
    /// Generiert zufällige Rotorpositionen
    fn generate_random_rotor_positions(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for i in 0..3 {
            let random_letter = (b'A' + rng.gen_range(0..26)) as char;
            self.rotor_positions[i] = random_letter.to_string();
        }
        
        self.add_log_entry(Level::Info, &format!(
            "Zufällige Rotorpositionen generiert: {} {} {}",
            self.rotor_positions[0], self.rotor_positions[1], self.rotor_positions[2]
        ));
    }
    
    /// Generiert zufällige Ringstellungen
    fn generate_random_ring_settings(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for i in 0..3 {
            let random_letter = (b'A' + rng.gen_range(0..26)) as char;
            self.ring_settings[i] = random_letter.to_string();
        }
        
        self.add_log_entry(Level::Info, &format!(
            "Zufällige Ringstellungen generiert: {} {} {}",
            self.ring_settings[0], self.ring_settings[1], self.ring_settings[2]
        ));
    }
    
    /// Generiert zufällige Steckerbrett-Verbindungen
    fn generate_random_plugboard(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut connections = Vec::new();
        let mut used = std::collections::HashSet::new();
        
        // Generiere 5-10 zufällige Verbindungen
        let num_connections = rng.gen_range(5..=10);
        
        for _ in 0..num_connections {
            let mut first = (b'A' + rng.gen_range(0..26)) as char;
            while used.contains(&first) {
                first = (b'A' + rng.gen_range(0..26)) as char;
            }
            
            let mut second = (b'A' + rng.gen_range(0..26)) as char;
            while second == first || used.contains(&second) {
                second = (b'A' + rng.gen_range(0..26)) as char;
            }
            
            used.insert(first);
            used.insert(second);
            connections.push(format!("{}{}", first, second));
        }
        
        self.plugboard_connections = connections.join(" ");
        
        self.add_log_entry(Level::Info, &format!(
            "Zufällige Steckerbrett-Verbindungen generiert: {}",
            self.plugboard_connections
        ));
    }
    
    /// Rendert die Preset-Auswahl
    fn render_preset_selector(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("⚙️ Konfigurations-Presets").size(16.0));
            ui.horizontal(|ui| {
                ui.label("Preset:");
                egui::ComboBox::from_id_source("preset_selector")
                    .selected_text(&self.selected_rotor_preset)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_rotor_preset, "Standard".to_string(), "Standard");
                        ui.selectable_value(&mut self.selected_rotor_preset, "Kriegsmarine".to_string(), "Kriegsmarine");
                        ui.selectable_value(&mut self.selected_rotor_preset, "Luftwaffe".to_string(), "Luftwaffe");
                    });
                
                if ui.button("Anwenden").clicked() {
                    let preset = self.selected_rotor_preset.clone();
                    self.apply_rotor_preset(&preset);
                }
            });
        });
    }
    
    /// Initialisiert die Enigma-Maschine mit den aktuellen Einstellungen
    fn initialize_machine(&mut self) {
        match self.create_machine_from_config() {
            Ok(machine) => {
                self.machine = Some(machine);
                self.add_log_entry(Level::Info, "Enigma-Maschine erfolgreich initialisiert");
            }
            Err(e) => {
                self.add_log_entry(Level::Error, &format!("Fehler bei Maschineninitialisierung: {}", e));
            }
        }
    }
    
    /// Erstellt eine Enigma-Maschine basierend auf der aktuellen Konfiguration
    fn create_machine_from_config(&self) -> Result<EnigmaMachine, String> {
        let rotor_positions = [
            self.rotor_positions[0].chars().next().unwrap_or('A'),
            self.rotor_positions[1].chars().next().unwrap_or('A'),
            self.rotor_positions[2].chars().next().unwrap_or('A'),
        ];
        
        let ring_settings = [
            self.ring_settings[0].chars().next().unwrap_or('A'),
            self.ring_settings[1].chars().next().unwrap_or('A'),
            self.ring_settings[2].chars().next().unwrap_or('A'),
        ];
        
        factory::create_custom_machine(
            [&self.rotor_types[0], &self.rotor_types[1], &self.rotor_types[2]],
            rotor_positions,
            ring_settings,
            &self.reflector_type,
            &self.plugboard_connections,
        )
    }
    
    /// Fügt einen Log-Eintrag hinzu
    fn add_log_entry(&mut self, level: Level, message: &str) {
        let entry = LogEntry {
            level,
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        };
        
        self.log_entries.push_back(entry);
        
        // Begrenze die Anzahl der Log-Einträge
        if self.log_entries.len() > MAX_LOG_ENTRIES {
            self.log_entries.pop_front();
        }
    }
    
    /// Führt die Verschlüsselung/Entschlüsselung durch
    fn process_text(&mut self) {
        if self.input_text.trim().is_empty() {
            self.add_log_entry(Level::Warn, "Kein Text zur Verarbeitung eingegeben");
            return;
        }
        
        if self.machine.is_none() {
            self.add_log_entry(Level::Error, "Enigma-Maschine ist nicht initialisiert");
            return;
        }
        
        let clean_input = clean_text(&self.input_text);
        
        self.add_log_entry(Level::Info, &format!(
            "Starte {} von: '{}'",
            if self.operation_mode == OperationMode::Encrypt { "Verschlüsselung" } else { "Entschlüsselung" },
            clean_input
        ));
        
        // Konfigurationsinfo vor der Maschinenverarbeitung sammeln
        let config_info = if let Some(ref machine) = self.machine {
            machine.get_configuration_info()
        } else {
            "Keine Maschine verfügbar".to_string()
        };
        
        self.add_log_entry(Level::Info, &format!("Maschinenkonfiguration:\n{}", config_info));
        
        // Jetzt die Maschine verwenden
        let machine = self.machine.as_mut().unwrap();
        let result = match self.operation_mode {
            OperationMode::Encrypt => machine.encrypt(&clean_input),
            OperationMode::Decrypt => machine.decrypt(&clean_input),
        };
        
        self.output_text = result;
        self.add_log_entry(Level::Info, &format!("Verarbeitung abgeschlossen: '{}'", self.output_text));
    }
    
    /// Rendert die Hauptkonfigurationsseite
    fn render_config_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_source("config_scroll")
            .show(ui, |ui| {
                ui.heading(egui::RichText::new("🔧 Enigma-Konfiguration").size(18.0).color(egui::Color32::from_rgb(70, 130, 180)));
                
                ui.add_space(15.0);
                
                // Preset-Auswahl
                self.render_preset_selector(ui);
                
                ui.add_space(10.0);
                
                // Rotor-Auswahl
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🌀 Rotoren").size(16.0));
                    ui.horizontal(|ui| {
                        for i in 0..3 {
                            ui.vertical(|ui| {
                                let rotor_colors = [
                                    egui::Color32::from_rgb(220, 20, 60),   // Rot
                                    egui::Color32::from_rgb(0, 128, 0),     // Grün
                                    egui::Color32::from_rgb(30, 144, 255),  // Blau
                                ];
                                ui.label(egui::RichText::new(format!("Rotor {}", i + 1))
                                    .color(rotor_colors[i])
                                    .size(14.0));
                                egui::ComboBox::from_id_source(format!("rotor_{}", i))
                                    .selected_text(&self.rotor_types[i])
                                    .show_ui(ui, |ui| {
                                        for rotor in ["I", "II", "III", "IV", "V"] {
                                            ui.selectable_value(&mut self.rotor_types[i], rotor.to_string(), rotor);
                                        }
                                    });
                            });
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Rotorpositionen
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("📍 Rotorpositionen").size(16.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🎲 Zufällig").clicked() {
                                self.generate_random_rotor_positions();
                            }
                        });
                    });
                    ui.horizontal(|ui| {
                        for i in 0..3 {
                            ui.vertical(|ui| {
                                let rotor_colors = [
                                    egui::Color32::from_rgb(220, 20, 60),
                                    egui::Color32::from_rgb(0, 128, 0),
                                    egui::Color32::from_rgb(30, 144, 255),
                                ];
                                ui.label(egui::RichText::new(format!("Position {}", i + 1))
                                    .color(rotor_colors[i])
                                    .size(12.0));
                                let pos_edit = egui::TextEdit::singleline(&mut self.rotor_positions[i])
                                    .char_limit(1)
                                    .desired_width(60.0);
                                ui.add(pos_edit);
                            });
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Ringstellungen
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🔘 Ringstellungen").size(16.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🎲 Zufällig").clicked() {
                                self.generate_random_ring_settings();
                            }
                        });
                    });
                    ui.horizontal(|ui| {
                        for i in 0..3 {
                            ui.vertical(|ui| {
                                let rotor_colors = [
                                    egui::Color32::from_rgb(220, 20, 60),
                                    egui::Color32::from_rgb(0, 128, 0),
                                    egui::Color32::from_rgb(30, 144, 255),
                                ];
                                ui.label(egui::RichText::new(format!("Ring {}", i + 1))
                                    .color(rotor_colors[i])
                                    .size(12.0));
                                let ring_edit = egui::TextEdit::singleline(&mut self.ring_settings[i])
                                    .char_limit(1)
                                    .desired_width(60.0);
                                ui.add(ring_edit);
                            });
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Reflektor
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🪞 Reflektor").size(16.0));
                    egui::ComboBox::from_id_source("reflector")
                        .selected_text(&self.reflector_type)
                        .show_ui(ui, |ui| {
                            for reflector in ["A", "B", "C"] {
                                ui.selectable_value(&mut self.reflector_type, reflector.to_string(), reflector);
                            }
                        });
                });
                
                ui.add_space(10.0);
                
                // Steckerbrett
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🔌 Steckerbrett-Verbindungen").size(16.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🎲 Zufällig").clicked() {
                                self.generate_random_plugboard();
                            }
                        });
                    });
                    ui.label(egui::RichText::new("Format: AB CD EF (Buchstabenpaare durch Leerzeichen getrennt)").italics().color(egui::Color32::GRAY));
                    ui.text_edit_multiline(&mut self.plugboard_connections);
                });
                
                ui.add_space(15.0);
                
                // Zufalls-Button und Konfiguration anwenden
                ui.horizontal_centered(|ui| {
                    if ui.add(egui::Button::new(egui::RichText::new("🎲 Alles zufällig").size(16.0))
                        .fill(egui::Color32::from_rgb(138, 43, 226))).clicked() {
                        self.generate_random_rotor_positions();
                        self.generate_random_ring_settings();
                        self.generate_random_plugboard();
                        self.add_log_entry(Level::Info, "Komplette zufällige Konfiguration generiert!");
                    }
                    
                    ui.add_space(10.0);
                    
                    if ui.add(egui::Button::new(egui::RichText::new("✅ Konfiguration anwenden").size(16.0))
                        .fill(egui::Color32::from_rgb(0, 128, 0))).clicked() {
                        self.initialize_machine();
                    }
                });
            });
    }
    
    /// Rendert die Textverarbeitungsseite
    fn render_text_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("📝 Text-Verarbeitung").size(18.0).color(egui::Color32::from_rgb(70, 130, 180)));
        
        ui.add_space(10.0);
        
        // Modus-Auswahl mit schöneren Buttons
        ui.group(|ui| {
            ui.label(egui::RichText::new("🎯 Verarbeitungsmodus").size(16.0));
            ui.horizontal(|ui| {
                let encrypt_selected = self.operation_mode == OperationMode::Encrypt;
                let decrypt_selected = self.operation_mode == OperationMode::Decrypt;
                
                let encrypt_color = if encrypt_selected { 
                    egui::Color32::from_rgb(0, 128, 0) 
                } else { 
                    egui::Color32::GRAY 
                };
                
                let decrypt_color = if decrypt_selected { 
                    egui::Color32::from_rgb(220, 20, 60) 
                } else { 
                    egui::Color32::GRAY 
                };
                
                if ui.add(egui::Button::new(egui::RichText::new("🔒 Verschlüsseln").color(encrypt_color))
                    .fill(if encrypt_selected { egui::Color32::from_rgba_premultiplied(0, 128, 0, 50) } else { egui::Color32::TRANSPARENT }))
                    .clicked() {
                    self.operation_mode = OperationMode::Encrypt;
                }
                
                if ui.add(egui::Button::new(egui::RichText::new("🔓 Entschlüsseln").color(decrypt_color))
                    .fill(if decrypt_selected { egui::Color32::from_rgba_premultiplied(220, 20, 60, 50) } else { egui::Color32::TRANSPARENT }))
                    .clicked() {
                    self.operation_mode = OperationMode::Decrypt;
                }
            });
        });
        
        ui.add_space(15.0);
        
        // Eingabefeld
        ui.group(|ui| {
            ui.label(egui::RichText::new("📥 Eingabetext").size(16.0));
            ui.text_edit_multiline(&mut self.input_text);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Zeichen: ").color(egui::Color32::GRAY));
                ui.label(egui::RichText::new(format!("{}", self.input_text.len())).color(egui::Color32::GRAY));
                
                if ui.button("🗑️ Löschen").clicked() {
                    self.input_text.clear();
                }
            });
        });
        
        ui.add_space(15.0);
        
        // Verarbeitungs-Button
        ui.horizontal_centered(|ui| {
            let button_text = match self.operation_mode {
                OperationMode::Encrypt => "🔒 Verschlüsseln",
                OperationMode::Decrypt => "🔓 Entschlüsseln",
            };
            
            let button_color = match self.operation_mode {
                OperationMode::Encrypt => egui::Color32::from_rgb(0, 128, 0),
                OperationMode::Decrypt => egui::Color32::from_rgb(220, 20, 60),
            };
            
            if ui.add(egui::Button::new(egui::RichText::new(button_text).size(16.0))
                .fill(button_color)
                .min_size(egui::Vec2::new(150.0, 40.0))).clicked() {
                self.process_text();
            }
        });
        
        ui.add_space(15.0);
        
        // Ausgabefeld
        ui.group(|ui| {
            ui.label(egui::RichText::new("📤 Ergebnis").size(16.0));
            ui.add(egui::TextEdit::multiline(&mut self.output_text).interactive(false));
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Zeichen: ").color(egui::Color32::GRAY));
                ui.label(egui::RichText::new(format!("{}", self.output_text.len())).color(egui::Color32::GRAY));
                
                if ui.button("📋 Kopieren").clicked() {
                    ui.output_mut(|o| o.copied_text = self.output_text.clone());
                }
            });
        });
        
        ui.add_space(15.0);
        
        // Aktuelle Rotorpositionen anzeigen
        if let Some(machine) = &self.machine {
            ui.group(|ui| {
                ui.label(egui::RichText::new("🌀 Aktuelle Rotorpositionen").size(16.0));
                let positions = machine.get_rotor_positions();
                let ring_settings = machine.get_ring_settings();
                
                ui.horizontal(|ui| {
                    for i in 0..3 {
                        let rotor_colors = [
                            egui::Color32::from_rgb(220, 20, 60),
                            egui::Color32::from_rgb(0, 128, 0),
                            egui::Color32::from_rgb(30, 144, 255),
                        ];
                        
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(format!("Rotor {}", i + 1))
                                .color(rotor_colors[i])
                                .size(12.0));
                            ui.label(egui::RichText::new(format!("Pos: {}", positions[i]))
                                .color(rotor_colors[i])
                                .size(14.0));
                            ui.label(egui::RichText::new(format!("Ring: {}", ring_settings[i]))
                                .color(egui::Color32::GRAY)
                                .size(12.0));
                        });
                    }
                });
            });
        }
    }
    
    /// Rendert das Log-Panel
    fn render_log_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("📊 Verarbeitungs-Log").size(18.0).color(egui::Color32::from_rgb(70, 130, 180)));
        
        ui.add_space(10.0);
        
        // Log-Controls
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.auto_scroll_log, "📜 Auto-Scroll");
                ui.separator();
                ui.label("🔍 Filter:");
                ui.text_edit_singleline(&mut self.log_filter);
                ui.separator();
                
                if ui.button("🗑️ Log löschen").clicked() {
                    self.log_entries.clear();
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Einträge: {}", self.log_entries.len()));
                });
            });
        });
        
        ui.add_space(5.0);
        
        // Log-Anzeige
        egui::ScrollArea::vertical()
            .id_source("log_scroll")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let filtered_entries: Vec<_> = if self.log_filter.is_empty() {
                    self.log_entries.iter().collect()
                } else {
                    self.log_entries.iter()
                        .filter(|entry| entry.message.to_lowercase().contains(&self.log_filter.to_lowercase()))
                        .collect()
                };
                
                let entry_count = filtered_entries.len();
                
                for entry in &filtered_entries {
                    let (color, icon) = match entry.level {
                        Level::Error => (egui::Color32::from_rgb(220, 20, 60), "❌"),
                        Level::Warn => (egui::Color32::from_rgb(255, 165, 0), "⚠️"),
                        Level::Info => (egui::Color32::from_rgb(70, 130, 180), "ℹ️"),
                        Level::Debug => (egui::Color32::from_rgb(128, 128, 128), "🐛"),
                        Level::Trace => (egui::Color32::from_rgb(105, 105, 105), "🔍"),
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(icon).size(12.0));
                        ui.label(egui::RichText::new(&format!("[{}]", entry.timestamp.format("%H:%M:%S")))
                            .color(egui::Color32::GRAY));
                        ui.label(egui::RichText::new(&format!("{}: {}", entry.level, entry.message))
                            .color(color));
                    });
                }
                
                if entry_count == 0 && !self.log_entries.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label(egui::RichText::new("🔍 Keine Einträge gefunden").color(egui::Color32::GRAY));
                    });
                }
                
                if self.auto_scroll_log && ui.available_height() > 0.0 {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                }
            });
    }
}

impl eframe::App for EnigmaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Verbesserte Menüleiste
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("📁 Datei", |ui| {
                    if ui.button("💾 Konfiguration speichern").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("📂 Konfiguration laden").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🚪 Beenden").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("👁️ Ansicht", |ui| {
                    ui.checkbox(&mut self.show_config, "⚙️ Konfiguration");
                    ui.checkbox(&mut self.show_log, "📊 Log");
                    ui.separator();
                    ui.checkbox(&mut self.dark_mode, "🌙 Dunkler Modus");
                });
                
                ui.menu_button("❓ Hilfe", |ui| {
                    ui.checkbox(&mut self.show_help, "📖 Hilfe anzeigen");
                    ui.separator();
                    if ui.button("ℹ️ Über Enigma-Simulator").clicked() {
                        ui.close_menu();
                    }
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("➖").clicked() {
                        // Minimize (nicht implementiert)
                    }
                });
            });
        });
        
        // Hilfe-Panel
        if self.show_help {
            egui::Window::new("📖 Hilfe")
                .open(&mut self.show_help)
                .show(ctx, |ui| {
                    ui.label("🔐 Enigma-Simulator - Hilfe");
                    ui.separator();
                    ui.label("1. Wählen Sie ein Konfigurations-Preset oder konfigurieren Sie manuell");
                    ui.label("2. Geben Sie Ihren Text ein");
                    ui.label("3. Wählen Sie Verschlüsseln oder Entschlüsseln");
                    ui.label("4. Klicken Sie auf Verarbeiten");
                    ui.separator();
                    ui.label("💡 Tipp: Das Log zeigt jeden Schritt der Verschlüsselung");
                });
        }
        
        // Hauptinhalt mit Header
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            
            // Responsive Layout
            if ui.available_width() > 800.0 {
                // Horizontales Layout für große Bildschirme
                ui.horizontal(|ui| {
                    // Textverarbeitung (immer sichtbar)
                    ui.vertical(|ui| {
                        self.render_text_panel(ui);
                    });
                    
                    if self.show_config {
                        ui.vertical(|ui| {
                            self.render_config_panel(ui);
                        });
                    }
                    
                    if self.show_log {
                        ui.vertical(|ui| {
                            self.render_log_panel(ui);
                        });
                    }
                });
            } else {
                // Vertikales Layout für kleine Bildschirme
                ui.vertical(|ui| {
                    // Textverarbeitung (immer sichtbar)
                    self.render_text_panel(ui);
                    
                    if self.show_config {
                        ui.add_space(10.0);
                        self.render_config_panel(ui);
                    }
                    
                    if self.show_log {
                        ui.add_space(10.0);
                        self.render_log_panel(ui);
                    }
                });
            }
        });
        
        // Status-Bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔐 Enigma-Simulator v1.0").color(egui::Color32::GRAY));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(machine) = &self.machine {
                        let positions = machine.get_rotor_positions();
                        ui.label(egui::RichText::new(format!("Positionen: {} {} {}", positions[0], positions[1], positions[2])).color(egui::Color32::GRAY));
                    }
                });
            });
        });
        
        // Automatisches Update für Animationen
        ctx.request_repaint();
    }
}

/// Benutzerdefinierte Log-Appender für die GUI
pub struct GuiLogAppender {
    log_entries: std::sync::Arc<std::sync::Mutex<VecDeque<LogEntry>>>,
}

impl GuiLogAppender {
    pub fn new() -> Self {
        Self {
            log_entries: std::sync::Arc::new(std::sync::Mutex::new(VecDeque::new())),
        }
    }
    
    pub fn get_log_entries(&self) -> std::sync::Arc<std::sync::Mutex<VecDeque<LogEntry>>> {
        self.log_entries.clone()
    }
}

impl log::Log for GuiLogAppender {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let entry = LogEntry {
                level: record.level(),
                message: format!("{}", record.args()),
                timestamp: chrono::Utc::now(),
            };
            
            if let Ok(mut entries) = self.log_entries.lock() {
                entries.push_back(entry);
                if entries.len() > MAX_LOG_ENTRIES {
                    entries.pop_front();
                }
            }
        }
    }
    
    fn flush(&self) {}
}
