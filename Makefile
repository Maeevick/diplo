SHELL := bash
.SHELLFLAGS := -euo pipefail -c
.DEFAULT_GOAL := help

.PHONY: help dev web test check mutants android

APK := src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
WIRING := (game|src-tauri)/

help: ## list the targets
	@grep -E '^[a-zA-Z0-9_-]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{printf "  %-10s %s\n", $$1, $$2}'

dev: ## play on the Mac, in a native window, arrows or clicks
	cargo run -p diplo-game

web: ## play in a browser at http://127.0.0.1:1420, rebuilt on every change
	cd game && trunk serve

test: ## every rule of the game
	cargo test --workspace

check: ## formatting, lints on the Mac and in wasm, the rules' tests, no uncovered rule, and the dependencies' licences, as before a commit
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo clippy -p diplo-game --target wasm32-unknown-unknown -- -D warnings
	cargo llvm-cov -p diplo-core --ignore-filename-regex '$(WIRING)' --fail-uncovered-lines 0 --summary-only
	cargo deny check licenses

mutants: ## whether the rules' tests bite, at the end of a slice
	cargo mutants -p diplo-core --jobs 4

android: ## build the APK and install it on the tablet plugged over USB, keeping the leaderboard
	cargo tauri android build --apk --debug --target aarch64
	adb install -r $(APK)
