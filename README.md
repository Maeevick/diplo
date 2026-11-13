# Diplo

A Frogger-like for a young player, in 8-bit pixel art: lead a diplodocus, a triceratops or an ankylosaurus from the
bottom of the screen to the flowers at the top, one cell at a time, past velociraptors, atrociraptors, spinosauri,
T-rexes and giganotosauri. Trees and rocks block everyone, and carnivores turn round when they hit one.

- **Controls:** tap above, below, left or right of the dino; arrows on a keyboard.
- **Lives:** three drumsticks. A catch costs one and sends the dino back to the start of the level.
- **Score:** level 1 is worth 1, level 2 is worth 2, and so on.
- **Difficulty:** each level is taller, faster and more crowded than the last, up to a cap that keeps it playable; every
  level is generated from a seed and checked to be crossable before it is played.
- **Top 10:** three letters, arcade style, kept on the device. A fresh install starts with ten dinos at 1.

## Layout

| Path | What |
| --- | --- |
| `core/` | the rules, with no engine: grid, steps, lanes, catches, levels, leaderboard, all tested |
| `game/` | Bevy 0.20 drawing the core's state, the pixel art and font in `game/src/art.rs` |
| `src-tauri/` | the Tauri 2 shell that wraps the wasm build into an Android APK |

## Play and build

```sh
make help      # the targets
make dev       # a native window on the Mac
make web       # http://127.0.0.1:1420
make check     # formatting, lints, tests, coverage of the rules, licences
make android   # build the APK and install it on the tablet over USB
```

The Android toolchain (JDK 21, SDK, NDK 27, `adb`) is set up as in Grindstone's runbook. On the tablet: Developer
options on, USB debugging on, then plug it in and accept the prompt. The APK is a debug build signed with the Mac's debug
key, which is enough to install over USB and lets `chrome://inspect` look inside the game's WebView.

## Later

- A second art pass on the spinosaurus, the favourite.
- Sounds: steps, catches, a level fanfare (started after the first tap, as browsers require).
- A release APK signed with a personal keystore, and a smaller wasm.
- A bot playing thousands of seeded games to tune the difficulty curve from data rather than guesses.
- A hint on the first level showing where to go.
