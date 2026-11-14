//! Leaving the app ends the game: coming back, the player always finds the dino choice.

use bevy::prelude::*;

use crate::Screen;

pub fn plugin(app: &mut App) {
    page::watch();
    app.add_systems(Update, come_back);
}

fn come_back(screen: Res<State<Screen>>, mut next: ResMut<NextState<Screen>>) {
    // Read on every screen, so that a leave seen on the first screen is not acted on later.
    let left = page::left();
    if left && *screen.get() != Screen::Choose {
        next.set(Screen::Choose);
    }
}

#[cfg(target_arch = "wasm32")]
mod page {
    use std::sync::atomic::{AtomicBool, Ordering};

    use wasm_bindgen::{JsCast, closure::Closure};

    /// Set by the browser when the page is hidden: another app on top, the home screen, the screen turned off.
    static LEFT: AtomicBool = AtomicBool::new(false);

    pub fn watch() {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let watched = document.clone();
        let on_change = Closure::<dyn Fn()>::new(move || {
            if watched.hidden() {
                LEFT.store(true, Ordering::Relaxed);
            }
        });
        document.set_onvisibilitychange(Some(on_change.as_ref().unchecked_ref()));
        // The page keeps the listener for as long as it lives.
        on_change.forget();
    }

    pub fn left() -> bool {
        LEFT.swap(false, Ordering::Relaxed)
    }
}

/// On the Mac the window is a developer's, and switching to the editor must not end the game.
#[cfg(not(target_arch = "wasm32"))]
mod page {
    pub fn watch() {}

    pub fn left() -> bool {
        false
    }
}
