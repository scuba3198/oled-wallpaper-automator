# OLED Wallpaper Automator

A lightweight Windows utility written in **Rust** that automatically downloads high‑resolution, OLED‑optimized wallpapers from [4kwallpapers.com](https://4kwallpapers.com/oled-wallpapers/) and sets them as your desktop background.

## Features
- **OLED‑Optimized** – Retrieves only wallpapers from the OLED category for better contrast on self‑emissive displays.
- **Random Variety** – Randomly selects a page and wallpaper each run.
- **History Tracking** – Stores the last 50 wallpaper URLs in `.wallpaper_history` to avoid repeats.
- **Clean Storage** – Removes previously downloaded files from the `wallpapers/` folder before saving a new one.
- **Wallpaper Scaling** – Uses the Windows `IDesktopWallpaper` COM interface with *Fill* scaling, ideal for a single display.
- **Network Resilience** – Retries up to three times with a 60‑second delay on failure.
- **Task Scheduler Integration** – Includes a PowerShell script to schedule a daily refresh.
- **Rust Best‑Practice Ready** – Crate‑level documentation, `#![deny(missing_docs)]`, and passes `cargo clippy -- -D warnings` with no warnings.

## Quick Start (Plug‑and‑Play)
1. **Place the project** somewhere permanent, e.g. `C:\\Users\\YourUsername\\Projects\\oled-wallpaper-automator`.
2. **Schedule the daily refresh** by running `setup-task.ps1` with PowerShell. This registers a task named `DailyOLEDWallpaper` to run at 9:00 AM each day. The script will download the pre‑compiled `oled-wallpaper-automator.exe` on the first run.
3. **Manual trigger** – Double‑click `Refresh Wallpaper.bat` to fetch and apply a new wallpaper instantly.
4. **Uninstall** – Run `uninstall.ps1` to delete the scheduled task, then delete the project folder.

## Repository Layout
- `src/main.rs` – Core Rust code handling scraping, file management, and COM interaction.
- `Cargo.toml` – Rust package manifest.
- `Refresh Wallpaper.bat` – Batch file for manual wallpaper refresh.
- `setup-task.ps1` – PowerShell script to schedule the daily task.
- `uninstall.ps1` – PowerShell script to remove the scheduled task.
- `wallpapers/` *(generated)* – Stores the current wallpaper.
- `.wallpaper_history` *(generated)* – Tracks the last 50 wallpaper URLs.

## Building From Source
1. Install [Rust & Cargo](https://rustup.rs/).
2. From a terminal in the project directory, run:
   ```
   cargo build --release
   ```
3. Copy the resulting executable to the project root:
   ```powershell
   copy target\\release\\oled-wallpaper-automator.exe .\\
   ```
The source passes `cargo clippy -- -D warnings` with no issues.

