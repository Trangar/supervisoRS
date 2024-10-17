use std::path::PathBuf;

use platform_dirs::AppDirs;

pub mod export;

pub fn find_factorio_install_dir() -> Option<PathBuf> {
    // Based on https://wiki.factorio.com/Application_directory#Application_directory
    for dir in [
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Factorio\\bin\\x64",
        "C:\\Program Files\\Factorio\\bin\\x64",
    ] {
        if PathBuf::from(dir).join("Factorio.exe").exists() {
            return Some(PathBuf::from(dir));
        }
    }

    for dir in [
        // macos
        "~/Library/Application Support/Steam/steamapps/common/Factorio.app/Contents",
        "/Applications/Factorio.app/Contents",
        // linux
        "~/.steam/steam/steamapps/common/Factorio/bin/x64",
        "~/.factorio/bin/x64",
    ] {
        let dir = shellexpand::tilde(dir).to_string();
        if PathBuf::from(&dir).join("Factorio").exists() {
            return Some(PathBuf::from(dir));
        }
    }

    None
}

pub fn find_factorio_config_dir() -> Option<PathBuf> {
    // Based on https://wiki.factorio.com/Application_directory#User_data_directory
    let user_dirs = AppDirs::new(Some("Factorio"), false).unwrap();
    let config_dir = user_dirs.config_dir;

    if config_dir.join("saves").exists() {
        return Some(config_dir);
    }

    for dir in ["~/Library/Application Support/factorio", "~/.factorio"] {
        let dir = shellexpand::tilde(dir).to_string();
        if PathBuf::from(&dir).join("saves").exists() {
            return Some(PathBuf::from(dir));
        }
    }

    None
}
