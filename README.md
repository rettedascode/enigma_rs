# Enigma Simulator in Rust

A complete Enigma simulator in Rust with both a graphical user interface (GUI) and a command-line interface (CLI). This simulator implements the historical Enigma machine with all essential components: rotors, reflector, and plugboard.

## 🚀 Features

- **Complete Enigma Implementation** with historically accurate rotors and reflectors
- **Graphical User Interface** with egui/eframe for intuitive operation
- **Command-Line Interface** for automated processing
- **Detailed Logging** with step-by-step encryption tracking
- **Configurable Components** (rotors, reflector, plugboard)
- **Comprehensive Tests** for correctness verification

## 📋 Prerequisites

- Rust 1.70 or higher
- Windows, Linux, or macOS

## 🛠️ Installation and Build

1. **Clone the repository:**
```bash
git clone <repository-url>
cd enigma_rs
```

2. **Install dependencies:**
```bash
cargo build
```

3. **Run tests:**
```bash
cargo test
```

4. **Start the application:**
```bash
# GUI mode (default)
cargo run

# CLI mode
cargo run -- --cli
```

## 🖥️ Graphical User Interface (GUI)

The GUI starts by default and provides intuitive operation:

### Main Features

1. **Text Processing:**
   - Input field for text to be processed
   - Choice between encryption and decryption
   - Immediate display of results
   - Display of current rotor positions

2. **Configuration:**
   - **Rotors:** Selection from 5 historical rotors (I, II, III, IV, V)
   - **Rotor Positions:** Setting the initial position of each rotor (A-Z)
   - **Ring Settings:** Configuration of ring setting (A-Z)
   - **Reflector:** Choice between reflectors A, B, and C
   - **Plugboard:** Configuration of letter connections

3. **Log Display:**
   - Detailed logging of all encryption steps
   - Color-coded log levels (Info, Warn, Error)
   - Auto-scroll function
   - Log clear function

### GUI Operation

1. **Adjust Configuration:** Select desired rotors, positions, and connections
2. **Click "Apply Configuration":** The machine is initialized with the new settings
3. **Enter Text:** Enter the text to be processed
4. **Choose Mode:** Encrypt or decrypt
5. **Click "Process":** The result is displayed and logged

## 💻 Command-Line Interface (CLI)

The CLI enables automated text processing:

### Basic Usage

```bash
# Encrypt
cargo run -- encrypt "HELLO WORLD" --positions "ABC" --rings "DEF"

# Decrypt
cargo run -- decrypt "ENCRYPTED TEXT" --positions "ABC" --rings "DEF"
```

### CLI Options

#### Encryption (`encrypt`)
```bash
cargo run -- encrypt <TEXT> [OPTIONS]
```

#### Decryption (`decrypt`)
```bash
cargo run -- decrypt <TEXT> [OPTIONS]
```

#### Available Options:
- `--positions, -P`: Rotor positions (e.g. "ABC") [Default: "AAA"]
- `--rings, -r`: Ring settings (e.g. "DEF") [Default: "AAA"]
- `--plugboard, -p`: Plugboard connections (e.g. "AB CD EF")
- `--rotors, -R`: Rotor types (e.g. "I,II,III") [Default: "I,II,III"]
- `--reflector, -F`: Reflector type (A, B, or C) [Default: "B"]
- `--verbose, -v`: Detailed log output

### CLI Examples

```bash
# Simple encryption
cargo run -- encrypt "SECRET MESSAGE"

# With custom settings
cargo run -- encrypt "HELLO" \
  --positions "XYZ" \
  --rings "ABC" \
  --rotors "II,IV,V" \
  --reflector "C" \
  --plugboard "AB CD EF GH"

# Decryption with same settings
cargo run -- decrypt "ENCRYPTED" \
  --positions "XYZ" \
  --rings "ABC" \
  --rotors "II,IV,V" \
  --reflector "C" \
  --plugboard "AB CD EF GH"

# With detailed logging
cargo run -- --verbose encrypt "TEST" --positions "AAA"
```

## 🔧 Technical Details

### Project Structure

```
src/
├── main.rs           # Entry point (GUI/CLI switch)
├── machine.rs        # Enigma main logic
├── rotor.rs          # Rotor implementation
├── reflector.rs      # Reflector implementation
├── plugboard.rs      # Plugboard implementation
├── gui.rs            # GUI with egui/eframe
└── utils.rs          # Utility functions

tests/
└── enigma_test.rs    # Comprehensive tests

README.md             # This file
Cargo.toml           # Dependencies and configuration
```

### Enigma Algorithm

The Enigma machine works in the following steps:

1. **Plugboard (Forward):** Exchange letters according to configured rules
2. **Rotor Rotation:** Automatic advancement of rotors before each processing
3. **Rotors (Forward):** Signal passes through the three rotors from right to left
4. **Reflector:** Signal is reflected back
5. **Rotors (Backward):** Signal passes through the rotors from left to right
6. **Plugboard (Backward):** Another exchange according to plugboard rules

### Rotor Rotation

- **Right Rotor:** Rotates with each character
- **Middle Rotor:** Rotates when the right rotor is at the notch
- **Left Rotor:** Rotates when the middle rotor is at the notch
- **Double Step:** Middle rotor rotates additionally when it itself is at the notch

### Historical Rotors

| Rotor | Wiring | Notch |
|-------|--------|-------|
| I     | EKMFLGDQVZNTOWYHXUSPAIBRCJ | Q |
| II    | AJDKSIRUXBLHWTMCQGZNPYFVOE | E |
| III   | BDFHJLCPRTXVZNYEIWGAKMUSQO | V |
| IV    | ESOVPZJAYQUIRHXLNFTGKDCMWB | J |
| V     | VZBRGITYUPSDNHLXAWMJQOFECK | Z |

### Reflectors

| Reflector | Wiring |
|-----------|--------|
| A         | EJMZALYXVBWFCRQUONTSPIKHGD |
| B         | YRUHQSLDPXNGOKMIEBFZCWVJAT |
| C         | FVPJIAOYEDRZXWGCTKUQSBNMHL |

## 📊 Logging System

The project uses the `log`/`env_logger` ecosystem for structured log output:

### Log Levels
- **INFO:** General information about processing steps
- **DEBUG:** Detailed information about configuration
- **TRACE:** Step-by-step encryption details
- **WARN:** Warnings for unusual situations
- **ERROR:** Errors and problems

### Log Integration in GUI
- All log entries are displayed in the GUI
- Color coding by log level
- Timestamps for each entry
- Auto-scroll and manual navigation possible

### CLI Logging
- INFO level by default
- `--verbose` for DEBUG/TRACE levels
- Timestamps in all output

## 🧪 Tests

The project contains comprehensive tests that ensure the correctness of the implementation:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific tests
cargo test test_encrypt_decrypt_symmetry
```

### Test Coverage
- **Symmetry Tests:** Encryption and decryption must be identical
- **Configuration Tests:** Various rotor, reflector, and plugboard configurations
- **Rotor Rotation:** Correct advancement and positioning
- **Longer Texts:** Processing of multiple characters
- **Edge Cases:** Special configurations and inputs

## 🔍 Debugging and Development

### Enable Detailed Logging
```bash
# Automatically active in GUI
# In CLI with --verbose
cargo run -- --verbose encrypt "TEST"

# Set environment variable
RUST_LOG=trace cargo run
```

### Development
```bash
# Debug build
cargo build

# Release build
cargo build --release

# With optimizations
cargo run --release
```

## 📚 Extensions

The project is structured to make extensions easy:

### Adding New Rotors
1. Define wiring in `rotor.rs`
2. Create factory function
3. Register in `available_rotors()`

### Adding New Reflectors
1. Define wiring in `reflector.rs`
2. Create factory function
3. Register in `available_reflectors()`

### GUI Extensions
- New configuration options in `gui.rs`
- Additional display panels
- Export/import of configurations

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Submit a pull request

## 📚 Sources and References

### Historical Documentation

- **Enigma Machine Specifications**: [Cryptomuseum.com](https://www.cryptomuseum.com/crypto/enigma/index.htm)
- **Enigma Rotor Wiring Tables**: [Wikipedia - Enigma Rotor Details](https://en.wikipedia.org/wiki/Enigma_rotor_details)
- **Enigma Reflector Specifications**: [Cipher Machines and Cryptology](http://users.telenet.be/d.rijmenants/en/enigmaspec.htm)
- **Historical Enigma Configurations**: [Tony Sale's Codes and Ciphers](https://www.codesandciphers.org.uk/enigma/index.htm)

### Technical Implementation

- **Rust Programming Language**: [rust-lang.org](https://www.rust-lang.org/)
- **egui GUI Framework**: [github.com/emilk/egui](https://github.com/emilk/egui)
- **eframe GUI Backend**: [docs.rs/eframe](https://docs.rs/eframe/)
- **clap CLI Framework**: [github.com/clap-rs/clap](https://github.com/clap-rs/clap)
- **log/env_logger Logging**: [docs.rs/log](https://docs.rs/log/)

### Algorithm and Cryptography

- **Enigma Algorithm Explanation**: [Practical Cryptography](https://practicalcryptography.com/ciphers/enigma-cipher/)
- **Enigma Stepping Mechanism**: [Crypto Stack Exchange](https://crypto.stackexchange.com/questions/281/can-someone-explain-the-enigma-algorithm)
- **Rotor Carry-Over Logic**: [Enigma Machine Mechanics](https://www.cryptomuseum.com/crypto/enigma/working.htm)

### Educational Resources

- **Understanding Enigma**: [Khan Academy Cryptography](https://www.khanacademy.org/computing/computer-science/cryptography)
- **Historical Context**: [Imperial War Museums](https://www.iwm.org.uk/history/how-alan-turing-cracked-the-enigma-code)
- **Bletchley Park Resources**: [bletchleypark.org.uk](https://bletchleypark.org.uk/)

### Code References

- **Rust Best Practices**: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)
- **egui Documentation**: [docs.rs/egui](https://docs.rs/egui/)
- **Conventional Commits**: [conventionalcommits.org](https://www.conventionalcommits.org/)

### Historical Data

The rotor wiring and reflector specifications used in this simulator are based on historically documented Enigma machines:

- **Rotors I-V**: Standard Wehrmacht rotors with authentic wiring
- **Reflectors A, B, C**: Original Wehrmacht reflectors
- **Ring Settings and Notches**: Historically correct configurations

### Image Sources and Icons

- **Emoji Icons**: Unicode Standard Emoji (used in GUI)
- **Enigma Machine Images**: Public Domain Historical Photographs

---

## 📄 License

This project is licensed under the MIT License.

## 🏛️ Historical Context

The Enigma machine was an electromechanical encryption device used by the German Wehrmacht during World War II. Its decryption by the Allies was a decisive factor in the course of the war.

This simulator is an educational implementation to illustrate the workings of the Enigma machine and the fundamentals of cryptography.

---

**Note:** This simulator is intended exclusively for educational purposes and historical studies. The algorithms implemented here are historical and not suitable for modern security applications.
