# Rust Best Practices Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align the `oled-wallpaper-automator` codebase with Rust best practices to ensure high-quality lints, optimized memory use, and production safety.

**Architecture:** Group and clean up imports, eliminate unnecessary clones/heap allocations in iterator chains, document unsafe blocks with safety guidelines, and introduce a unit testing module using a temporary file library. We will configure project-level clippy rules in `Cargo.toml` to prevent future regressions.

**Tech Stack:** Rust 2024, Cargo, anyhow, windows-rs, rand, scraper, tempfile

---

### Task 1: Build Configuration & Lint Enforcement

**Files:**
- Modify: [Cargo.toml](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/Cargo.toml)

- [ ] **Step 1: Add workspace-grade lints and dev-dependencies**

Edit `Cargo.toml` to append the lint configurations and `tempfile` dependency at the end:

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

- [ ] **Step 2: Verify compilation and download dependencies**

Run: `cargo check`
Expected: Succeeds and fetches the `tempfile` crate.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add lint configurations and tempfile dev-dependency"
```

---

### Task 2: Fix Clippy Warning and Group Imports

**Files:**
- Modify: [src/main.rs](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/src/main.rs:1-12)

- [ ] **Step 1: Clean imports and replace split().last()**

Modify `src/main.rs` imports block and the file extraction logic to match standard organization:

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

And update line 186:
```rust
let file_name = url.split('/').next_back().unwrap_or("wallpaper.png");
```

- [ ] **Step 2: Run clippy to verify compile and clean lints**

Run: `cargo clippy --all-targets --all-features --locked -- -D warnings`
Expected: Clines build cleanly with zero warnings or errors.

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "refactor: group imports and fix clippy double-ended-iterator-last warning"
```

---

### Task 3: Optimize Scraper Iterators & Remove Allocations

**Files:**
- Modify: [src/main.rs](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/src/main.rs:124-170)

- [ ] **Step 1: Remove Vec collection from pagination and implement declarative img url mapping**

In `get_wallpaper_links()`, change the pagination extraction (avoiding vec collection):
```rust
    let max_pages = if let Some(second_to_last) = document.select(&pagination_selector).rev().nth(1) {
        let text = second_to_last.text().collect::<String>();
        text.trim().parse::<u32>().unwrap_or(40)
    } else {
        40
    };
    println!("Detected max pages: {}", max_pages);
```

And update the image links collection loop to use map/filter-map:
```rust
    let links: Vec<String> = page_document
        .select(&img_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| {
            if href.starts_with('/') {
                format!("https://4kwallpapers.com{}", href)
            } else {
                href.to_string()
            }
        })
        .collect();
```

- [ ] **Step 2: Verify clippy**

Run: `cargo clippy --all-targets --all-features --locked -- -D warnings`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "perf: optimize scrapers iterators to prevent unnecessary allocations"
```

---

### Task 4: Optimize Reference Borrowing & Remove unwrap()

**Files:**
- Modify: [src/main.rs](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/src/main.rs:63-79)

- [ ] **Step 1: Replace filtered links collection and remove unwrap()**

In `run()`, adjust the chosen link step so we do not clone the whole list of filtered links, and remove the unwrap:
```rust
    // 4. Filter links using history
    let selected_url = {
        let mut rng = rand::thread_rng();
        let filtered_links: Vec<&String> = links
            .iter()
            .filter(|link| !history.contains(*link))
            .collect();

        if let Some(&link) = filtered_links.choose(&mut rng) {
            link.clone()
        } else {
            println!("All scraped wallpapers are in history. Selecting a random one anyway.");
            links.choose(&mut rng).context("No wallpaper links found at all")?.clone()
        }
    };
```

- [ ] **Step 2: Verify clippy**

Run: `cargo clippy --all-targets --all-features --locked -- -D warnings`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "refactor: optimize borrowing for selected link and remove unwrap"
```

---

### Task 5: Add Safety Documentation to Unsafe Blocks

**Files:**
- Modify: [src/main.rs](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/src/main.rs:20-22) and [src/main.rs](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/src/main.rs:205-217)

- [ ] **Step 1: Add SAFETY: comments to main() and download_and_set_wallpaper()**

Update the unsafe block in `main()`:
```rust
    // SAFETY: The COM library must be initialized on the current thread before executing COM operations.
    // We ignore the error result in case the COM library is already initialized on this thread.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
```

Update the unsafe block in `download_and_set_wallpaper()`:
```rust
    // SAFETY: COM must be initialized on this thread (guaranteed by main).
    // CoCreateInstance and interface methods are safe if given valid parameters.
    unsafe {
        // Create the IDesktopWallpaper instance
        let wallpaper_mgr: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
            .map_err(|e| anyhow!("Failed to create DesktopWallpaper COM instance: {}", e))?;

        // Set the wallpaper style to Span
        wallpaper_mgr.SetPosition(DWPOS_SPAN)
            .map_err(|e| anyhow!("Failed to set wallpaper style to Span: {}", e))?;

        // Set the wallpaper path (None sets it for all monitors)
        wallpaper_mgr.SetWallpaper(None, &path_hstring)
            .map_err(|e| anyhow!("Failed to set wallpaper: {}", e))?;
    }
```

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --all-targets --all-features --locked -- -D warnings`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "docs: add safety comments to unsafe COM operations"
```

---

### Task 6: Add Automated Testing

**Files:**
- Modify: [src/main.rs](file:///c:/Users/Mumukshu/Projects/oled-wallpaper-automator/src/main.rs) (append to the end of file)

- [ ] **Step 1: Append test module to the end of main.rs**

Add the unit tests module verifying history loading, saving, and history truncation limits:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn load_history_should_return_empty_when_file_not_found() {
        let path = Path::new("non_existent_file_xyz_123.txt");
        let history = load_history(path).unwrap();
        assert!(history.is_empty(), "Expected empty history list");
    }

    #[test]
    fn save_and_load_history_should_persist_and_retrieve_correctly() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        
        let original_history = vec![
            "https://example.com/1.png".to_string(),
            "https://example.com/2.png".to_string(),
        ];
        
        save_history(path, &original_history).unwrap();
        let loaded = load_history(path).unwrap();
        
        assert_eq!(loaded, original_history, "Loaded history does not match saved history");
    }
}
```

- [ ] **Step 2: Run cargo test**

Run: `cargo test`
Expected: 2 tests passed

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "test: add unit tests for history loading and saving"
```
