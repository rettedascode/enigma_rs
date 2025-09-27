# Enigma-Simulator in Rust

Ein vollständiger Enigma-Simulator in Rust mit sowohl einer grafischen Benutzeroberfläche (GUI) als auch einem Command-Line-Interface (CLI). Dieser Simulator implementiert die historische Enigma-Maschine mit allen wichtigen Komponenten: Rotoren, Reflektor und Steckerbrett.

## 🚀 Features

- **Vollständige Enigma-Implementierung** mit historisch korrekten Rotoren und Reflektoren
- **Grafische Benutzeroberfläche** mit egui/eframe für intuitive Bedienung
- **Command-Line-Interface** für automatisierte Verarbeitung
- **Detailliertes Logging** mit Schritt-für-Schritt-Nachverfolgung der Verschlüsselung
- **Konfigurierbare Komponenten** (Rotoren, Reflektor, Steckerbrett)
- **Umfassende Tests** für Verifikation der Korrektheit

## 📋 Voraussetzungen

- Rust 1.70 oder höher
- Windows, Linux oder macOS

## 🛠️ Installation und Build

1. **Repository klonen:**
```bash
git clone <repository-url>
cd enigma_rs
```

2. **Dependencies installieren:**
```bash
cargo build
```

3. **Tests ausführen:**
```bash
cargo test
```

4. **Anwendung starten:**
```bash
# GUI-Modus (Standard)
cargo run

# CLI-Modus
cargo run -- --cli
```

## 🖥️ Grafische Benutzeroberfläche (GUI)

Die GUI startet standardmäßig und bietet eine intuitive Bedienung:

### Hauptfunktionen

1. **Textverarbeitung:**
   - Eingabefeld für zu verarbeitenden Text
   - Auswahl zwischen Verschlüsselung und Entschlüsselung
   - Sofortige Anzeige des Ergebnisses
   - Anzeige der aktuellen Rotorpositionen

2. **Konfiguration:**
   - **Rotoren:** Auswahl aus 5 historischen Rotoren (I, II, III, IV, V)
   - **Rotorpositionen:** Einstellung der Grundstellung jedes Rotors (A-Z)
   - **Ringstellungen:** Konfiguration der Ringstellung (A-Z)
   - **Reflektor:** Auswahl zwischen Reflektoren A, B und C
   - **Steckerbrett:** Konfiguration von Buchstabenverbindungen

3. **Log-Anzeige:**
   - Detaillierte Protokollierung aller Verschlüsselungsschritte
   - Farbkodierte Log-Level (Info, Warn, Error)
   - Auto-Scroll-Funktion
   - Log-Löschfunktion

### GUI-Bedienung

1. **Konfiguration anpassen:** Wählen Sie die gewünschten Rotoren, Positionen und Verbindungen
2. **"Konfiguration anwenden" klicken:** Die Maschine wird mit den neuen Einstellungen initialisiert
3. **Text eingeben:** Geben Sie den zu verarbeitenden Text ein
4. **Modus wählen:** Verschlüsseln oder Entschlüsseln
5. **"Verarbeiten" klicken:** Das Ergebnis wird angezeigt und im Log protokolliert

## 💻 Command-Line-Interface (CLI)

Das CLI ermöglicht die automatisierte Verarbeitung von Texten:

### Grundlegende Verwendung

```bash
# Verschlüsseln
cargo run -- encrypt "HELLO WORLD" --positions "ABC" --rings "DEF"

# Entschlüsseln
cargo run -- decrypt "ENCRYPTED TEXT" --positions "ABC" --rings "DEF"
```

### CLI-Optionen

#### Verschlüsselung (`encrypt`)
```bash
cargo run -- encrypt <TEXT> [OPTIONEN]
```

#### Entschlüsselung (`decrypt`)
```bash
cargo run -- decrypt <TEXT> [OPTIONEN]
```

#### Verfügbare Optionen:
- `--positions, -P`: Rotorpositionen (z.B. "ABC") [Standard: "AAA"]
- `--rings, -r`: Ringstellungen (z.B. "DEF") [Standard: "AAA"]
- `--plugboard, -p`: Steckerbrett-Verbindungen (z.B. "AB CD EF")
- `--rotors, -R`: Rotortypen (z.B. "I,II,III") [Standard: "I,II,III"]
- `--reflector, -F`: Reflektortyp (A, B, oder C) [Standard: "B"]
- `--verbose, -v`: Detaillierte Log-Ausgabe

### CLI-Beispiele

```bash
# Einfache Verschlüsselung
cargo run -- encrypt "SECRET MESSAGE"

# Mit benutzerdefinierten Einstellungen
cargo run -- encrypt "HELLO" \
  --positions "XYZ" \
  --rings "ABC" \
  --rotors "II,IV,V" \
  --reflector "C" \
  --plugboard "AB CD EF GH"

# Entschlüsselung mit gleichen Einstellungen
cargo run -- decrypt "ENCRYPTED" \
  --positions "XYZ" \
  --rings "ABC" \
  --rotors "II,IV,V" \
  --reflector "C" \
  --plugboard "AB CD EF GH"

# Mit detailliertem Logging
cargo run -- --verbose encrypt "TEST" --positions "AAA"
```

## 🔧 Technische Details

### Projektstruktur

```
src/
├── main.rs           # Einstiegspunkt (GUI/CLI-Switch)
├── machine.rs        # Enigma-Hauptlogik
├── rotor.rs          # Rotor-Implementierung
├── reflector.rs      # Reflektor-Implementierung
├── plugboard.rs      # Steckerbrett-Implementierung
├── gui.rs            # GUI mit egui/eframe
└── utils.rs          # Hilfsfunktionen

tests/
└── enigma_test.rs    # Umfassende Tests

README.md             # Diese Datei
Cargo.toml           # Dependencies und Konfiguration
```

### Enigma-Algorithmus

Die Enigma-Maschine funktioniert in folgenden Schritten:

1. **Steckerbrett (Vorwärts):** Austausch der Buchstaben nach konfigurierten Regeln
2. **Rotor-Drehung:** Automatische Weiterleitung der Rotoren vor jeder Verarbeitung
3. **Rotoren (Vorwärts):** Signal durchläuft die drei Rotoren von rechts nach links
4. **Reflektor:** Signal wird zurückgespiegelt
5. **Rotoren (Rückwärts):** Signal durchläuft die Rotoren von links nach rechts
6. **Steckerbrett (Rückwärts):** Erneuter Austausch nach Steckerbrett-Regeln

### Rotor-Drehung

- **Rechter Rotor:** Dreht sich bei jedem Zeichen
- **Mittlerer Rotor:** Dreht sich, wenn der rechte an der Kerbe ist
- **Linker Rotor:** Dreht sich, wenn der mittlere an der Kerbe ist
- **Doppelschritt:** Mittlerer Rotor dreht sich zusätzlich, wenn er selbst an der Kerbe ist

### Historische Rotoren

| Rotor | Verdrahtung | Kerbe |
|-------|-------------|-------|
| I     | EKMFLGDQVZNTOWYHXUSPAIBRCJ | Q |
| II    | AJDKSIRUXBLHWTMCQGZNPYFVOE | E |
| III   | BDFHJLCPRTXVZNYEIWGAKMUSQO | V |
| IV    | ESOVPZJAYQUIRHXLNFTGKDCMWB | J |
| V     | VZBRGITYUPSDNHLXAWMJQOFECK | Z |

### Reflektoren

| Reflektor | Verdrahtung |
|-----------|-------------|
| A         | EJMZALYXVBWFCRQUONTSPIKHGD |
| B         | YRUHQSLDPXNGOKMIEBFZCWVJAT |
| C         | FVPJIAOYEDRZXWGCTKUQSBNMHL |

## 📊 Logging-System

Das Projekt nutzt das `log`/`env_logger`-Ökosystem für strukturierte Log-Ausgaben:

### Log-Level
- **INFO:** Allgemeine Informationen über Verarbeitungsschritte
- **DEBUG:** Detaillierte Informationen über Konfiguration
- **TRACE:** Schritt-für-Schritt-Verschlüsselungsdetails
- **WARN:** Warnungen bei ungewöhnlichen Situationen
- **ERROR:** Fehler und Probleme

### Log-Integration in GUI
- Alle Log-Einträge werden in der GUI angezeigt
- Farbkodierung nach Log-Level
- Zeitstempel für jeden Eintrag
- Auto-Scroll und manuelle Navigation möglich

### CLI-Logging
- Standardmäßig INFO-Level
- `--verbose` für DEBUG/TRACE-Level
- Zeitstempel in allen Ausgaben

## 🧪 Tests

Das Projekt enthält umfassende Tests, die die Korrektheit der Implementierung sicherstellen:

```bash
# Alle Tests ausführen
cargo test

# Tests mit Ausgabe
cargo test -- --nocapture

# Spezifische Tests
cargo test test_encrypt_decrypt_symmetry
```

### Test-Abdeckung
- **Symmetrie-Tests:** Ver- und Entschlüsselung müssen identisch sein
- **Konfiguration-Tests:** Verschiedene Rotor-, Reflektor- und Steckerbrett-Konfigurationen
- **Rotor-Drehung:** Korrekte Weiterleitung und Positionierung
- **Längere Texte:** Verarbeitung von mehreren Zeichen
- **Grenzfälle:** Spezielle Konfigurationen und Eingaben

## 🔍 Debugging und Entwicklung

### Detailliertes Logging aktivieren
```bash
# In der GUI automatisch aktiv
# Im CLI mit --verbose
cargo run -- --verbose encrypt "TEST"

# Umgebungsvariable setzen
RUST_LOG=trace cargo run
```

### Entwicklung
```bash
# Debug-Build
cargo build

# Release-Build
cargo build --release

# Mit Optimierungen
cargo run --release
```

## 📚 Erweiterungen

Das Projekt ist so strukturiert, dass Erweiterungen einfach möglich sind:

### Neue Rotoren hinzufügen
1. Verdrahtung in `rotor.rs` definieren
2. Factory-Funktion erstellen
3. In `available_rotors()` registrieren

### Neue Reflektoren hinzufügen
1. Verdrahtung in `reflector.rs` definieren
2. Factory-Funktion erstellen
3. In `available_reflectors()` registrieren

### GUI-Erweiterungen
- Neue Konfigurationsoptionen in `gui.rs`
- Zusätzliche Anzeige-Panels
- Export/Import von Konfigurationen

## 🤝 Beitragen

Beiträge sind willkommen! Bitte:

1. Fork des Repositories erstellen
2. Feature-Branch erstellen
3. Tests für neue Funktionalität schreiben
4. Pull Request einreichen

## 📄 Lizenz

Dieses Projekt steht unter der MIT-Lizenz.

## 🏛️ Historischer Kontext

Die Enigma-Maschine war eine elektromechanische Verschlüsselungsmaschine, die während des Zweiten Weltkriegs von der deutschen Wehrmacht verwendet wurde. Ihre Entschlüsselung durch die Alliierten war ein entscheidender Faktor für den Kriegsverlauf.

Dieser Simulator ist eine pädagogische Implementierung zur Veranschaulichung der Funktionsweise der Enigma-Maschine und der Grundlagen der Kryptographie.

---

**Hinweis:** Dieser Simulator dient ausschließlich Bildungszwecken und historischen Studien. Die hier implementierten Algorithmen sind historisch und nicht für moderne Sicherheitsanwendungen geeignet.
