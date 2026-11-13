//! The leaderboard kept between games: in the WebView's local storage on the tablet, in a file on the Mac.

use diplo_core::Leaderboard;

#[cfg(target_arch = "wasm32")]
const KEY: &str = "diplo.leaderboard";

#[cfg(target_arch = "wasm32")]
fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

#[cfg(target_arch = "wasm32")]
pub fn load() -> Leaderboard {
    storage()
        .and_then(|storage| storage.get_item(KEY).ok().flatten())
        .map_or_else(Leaderboard::fresh, |text| Leaderboard::from_text(&text))
}

#[cfg(target_arch = "wasm32")]
pub fn save(board: &Leaderboard) {
    if let Some(storage) = storage() {
        let _ = storage.set_item(KEY, &board.to_text());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn path() -> std::path::PathBuf {
    std::env::temp_dir().join("diplo-leaderboard.txt")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load() -> Leaderboard {
    std::fs::read_to_string(path()).map_or_else(
        |_| Leaderboard::fresh(),
        |text| Leaderboard::from_text(&text),
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save(board: &Leaderboard) {
    let _ = std::fs::write(path(), board.to_text());
}
