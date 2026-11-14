//! Where the port keeps the player's files: saved games (and later the
//! high scores) in the platform's data directory, and its settings
//! ([`crate::settings`]) in the config directory. The
//! original writes its saves into the game's own `MAPS\` directory; the
//! port leaves the game's files alone and only reads from there. In the
//! browser both are [`WEB_STORAGE`], where the browser's storage is
//! mounted (`crate::fs`, `docs/web.md`).

use std::path::{Path, PathBuf};

/// Where the browser build mounts the browser's storage: its data and
/// config directory.
pub const WEB_STORAGE: &str = "/storage";

/// The directory names, (spaced, dashed): the port's, and the ones it
/// used before it was renamed, which [`renamed`] moves.
const NAME: (&str, &str) = ("Gadget Generator", "gadget-generator");
const OLD_NAME: (&str, &str) = ("Widget Workshop", "widget-workshop");

/// `parent`'s `new`, after moving the one
/// named `old` there when only that exists (the port's directories
/// before the rename). Best effort: if the move fails, the new one is
/// used regardless.
fn renamed(parent: &Path, new: &str, old: &str) -> PathBuf {
    let dir = parent.join(new);
    let legacy = parent.join(old);
    if !dir.exists() && legacy.is_dir() {
        let _ = std::fs::rename(&legacy, &dir);
    }
    dir
}

/// The directory the port's own files go in: `$XDG_DATA_HOME/gadget-generator`
/// (or `~/.local/share/gadget-generator`) on Linux and the other Unixes,
/// `~/Library/Application Support/Gadget Generator` on macOS and
/// `%APPDATA%\Gadget Generator` on Windows (the old `widget-workshop` or
/// `Widget Workshop` moved there). `None` if the environment
/// doesn't say where the home directory is. [`WEB_STORAGE`] in the
/// browser.
#[must_use]
pub fn data_dir() -> Option<PathBuf> {
    if cfg!(target_arch = "wasm32") {
        return Some(PathBuf::from(WEB_STORAGE));
    }
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(target_os = "windows") {
        var("APPDATA").map(|d| renamed(&d, NAME.0, OLD_NAME.0))
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|h| renamed(&h.join("Library/Application Support"), NAME.0, OLD_NAME.0))
    } else {
        var("XDG_DATA_HOME")
            .or_else(|| var("HOME").map(|h| h.join(".local/share")))
            .map(|d| renamed(&d, NAME.1, OLD_NAME.1))
    }
}

/// Where saved games go: `MAPS` in [`data_dir`], like the game's own
/// `MAPS\`.
#[must_use]
pub fn save_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("MAPS"))
}

/// The directory the port's settings go in: `$XDG_CONFIG_HOME/gadget-generator`
/// (or `~/.config/gadget-generator`) on Linux and the other Unixes,
/// `~/Library/Preferences/Gadget Generator` on macOS and
/// `%APPDATA%\Gadget Generator` on Windows (the old ones moved there, as
/// for [`data_dir`]). `None` if the environment
/// doesn't say where the home directory is. [`WEB_STORAGE`] in the
/// browser.
#[must_use]
pub fn config_dir() -> Option<PathBuf> {
    if cfg!(target_arch = "wasm32") {
        return Some(PathBuf::from(WEB_STORAGE));
    }
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(target_os = "windows") {
        var("APPDATA").map(|d| renamed(&d, NAME.0, OLD_NAME.0))
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|h| renamed(&h.join("Library/Preferences"), NAME.0, OLD_NAME.0))
    } else {
        var("XDG_CONFIG_HOME")
            .or_else(|| var("HOME").map(|h| h.join(".config")))
            .map(|d| renamed(&d, NAME.1, OLD_NAME.1))
    }
}
