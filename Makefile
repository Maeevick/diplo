SHELL := bash
.SHELLFLAGS := -euo pipefail -c
.DEFAULT_GOAL := help

.PHONY: help dev web test check mutants android android-push bump

APK := src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
WIRING := (game|src-tauri)/
# Where the version lives: the crates', and the app's, which names the APK's version.
VERSIONED := Cargo.toml Cargo.lock src-tauri/tauri.conf.json

help: ## list the targets
	@grep -E '^[a-zA-Z0-9_-]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{printf "  %-13s %s\n", $$1, $$2}'

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

android-push: ## build the APK and copy it to the tablet's Downloads, for devices that refuse installs over USB: tap it there
	cargo tauri android build --apk --debug --target aarch64
	adb push $(APK) /sdcard/Download/diplo.apk
	@# A pushed file stays out of the media index the Files app lists, until it is scanned.
	adb shell am broadcast -a android.intent.action.MEDIA_SCANNER_SCAN_FILE -d file:///sdcard/Download/diplo.apk > /dev/null
	@echo "on the tablet: Files, Downloads, diplo.apk"

bump: ## every crate and the app at VERSION, committed then tagged vVERSION, never pushed: make bump VERSION=0.2.0
	@test -n "$(VERSION)" || { echo "which version? make bump VERSION=0.2.0"; exit 2; }
	@! git rev-parse --quiet --verify "refs/tags/v$(VERSION)" > /dev/null || { echo "v$(VERSION) is already tagged"; exit 2; }
	@git diff --quiet HEAD -- $(VERSIONED) || { echo "$(VERSIONED) carry changes of their own: commit or stash them first"; exit 2; }
	@VERSION="$(VERSION)" perl -0pi -e 's/(\[workspace\.package\]\nversion = )"[^"]*"/$$1"$$ENV{VERSION}"/' Cargo.toml
	@VERSION="$(VERSION)" perl -pi -e 's/^(\s*"version": )"[^"]*"/$$1"$$ENV{VERSION}"/' src-tauri/tauri.conf.json
	@cargo update --workspace --offline --quiet
	@git commit --quiet -m "chore: every crate and the app at $(VERSION)" -- $(VERSIONED)
	@git tag -a "v$(VERSION)" -m "diplo $(VERSION)"
	@echo "every crate and the app at $(VERSION), tagged v$(VERSION); push both with: git push --follow-tags"
