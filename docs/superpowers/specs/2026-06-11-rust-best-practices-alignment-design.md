# 🌌 OLED Wallpaper Automator - Rust Best Practices Alignment Spec

## Goal

Align the `oled-wallpaper-automator` codebase with the `rust-best-practices` guidelines. This includes adding linting discipline, resolving clippy errors, optimizing memory and iterator usage, documenting `unsafe` operations with `SAFETY` comments, and adding automated tests to ensure production safety and regression prevention.

## Proposed Changes

### Cargo.toml

Configure package-level lints and add `tempfile` as a dev-dependency to facilitate safe testing:

```toml
[lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[lints.clippy]
all = { level = "deny", priority = 10 }
redundant_clone = { level = "deny", priority = 9 }
manual_while_let_some = { level = "deny", priority = 4 }
pedantic = { level = "warn", priority = 3 }

[dev-dependencies]
tempfile = "3.8"
```

### src/main.rs

#### 1. Imports Organization
Group imports according to the best practice (standard library first, then external crates):

```rust
// std
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

// external crates
use anyhow::{anyhow, Context, Result};
use rand::seq::SliceRandom;
use scraper::{Html, Selector};
use windows::core::HSTRING;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper, DWPOS_SPAN};
```

#### 2. Iterator & Allocation Optimizations
- **File Name extraction**: Replace `.last()` with `.next_back()` to solve the clippy error:
  ```rust
  let file_name = url.split('/').next_back().unwrap_or("wallpaper.png");
  ```
- **Pagination search**: Replace the vector heap allocation with a lazy iterator:
  ```rust
  let max_pages = if let Some(second_to_last) = document.select(&pagination_selector).rev().nth(1) {
      let text = second_to_last.text().collect::<String>();
      text.trim().parse::<u32>().unwrap_or(40)
  } else {
      40
  };
  ```
- **Declarative parsing**: Use `filter_map` and `map` for links extraction.
- **Link selection**: Collect references (`Vec<&String>`) and only clone the single chosen link to avoid cloning the whole list of links.

#### 3. Production Safety
- Remove `.unwrap()` call when selecting a filtered link.
- Add `SAFETY:` comments to explain why the `unsafe` Windows COM calls are sound.

#### 4. Automated Tests
Add a `tests` module testing the logic of `load_history` and `save_history`.

## Verification Plan

### Automated Tests
- Run `cargo test` to execute all unit tests.
- Run `cargo clippy --all-targets --all-features --locked -- -D warnings` to verify no clippy warnings or linting violations remain.
- Run `cargo check` to verify the build compiles cleanly.
