# 🌌 OLED Wallpaper Automator

A lightweight, automated Windows utility written in Rust to keep your desktop fresh with high-resolution, OLED-optimized wallpapers scraped daily from [4kwallpapers.com](https://4kwallpapers.com/oled-wallpapers/).

---

## ✨ Features

- **AMOLED/OLED Optimized**: Exclusively fetches from the OLED category, providing high-contrast backgrounds that save power and look stunning on self-emissive screens.
- **Random Variety**: Automatically detects the total number of pages and crawls a random page to ensure wide variety.
- **Intelligent History**: Remembers your last 50 wallpapers using a local history file (`.wallpaper_history`) to prevent repeated backgrounds.
- **Clean Storage**: Automatically deletes previously downloaded wallpapers from the local `wallpapers/` directory to save disk space.
- **Multi-Monitor Span**: Sets the desktop background via the native Windows COM `IDesktopWallpaper` interface using "Span" scaling, perfectly filling all connected monitors.
- **Network Resilience**: Automatically retries up to 3 times (with a 60-second delay) if your network connection is not immediately active upon startup.
- **Set & Forget**: Easily schedule it to run daily using Windows Task Scheduler—even if your PC was off during the scheduled time, it will run as soon as you boot.

---

## 🚀 Quick Start (Plug-and-Play)

This project is fully automated. You do not need to install Rust or manually download any executables. The helper scripts will automatically bootstrap the application for you.

### Installation & Automation

1. **Place the folder**: Save this project folder in a permanent location (e.g., `C:\Users\YourUsername\Projects\oled-wallpaper-automator`). 
   > [!IMPORTANT]
   > Do not move or rename this folder after scheduling, otherwise Windows Task Scheduler will not be able to find the executable.
2. **Schedule Daily Refresh**:
   - Right-click **`setup-task.ps1`** and select **Run with PowerShell**.
   - This schedules a daily Windows task named `DailyOLEDWallpaper` to run at **9:00 AM**.
   - *Note: On the very first execution, the script will automatically download the precompiled `oled-wallpaper-automator.exe` binary directly from GitHub Releases.*

### Manual Trigger
- Double-click **`Refresh Wallpaper.bat`** to fetch and apply a new wallpaper instantly.
- *Tip: You can right-click this batch file and select **Send to -> Desktop (create shortcut)** for easy access.*

### Uninstallation
1. Right-click **`uninstall.ps1`** and select **Run with PowerShell** to safely delete the scheduled task from Windows Task Scheduler.
2. Delete the project folder. Your current wallpaper will remain active.

---

## 📂 File Structure

- **`src/main.rs`**: The Rust source code that handles scraping, file management, and COM interactions.
- **`Cargo.toml`**: The Rust dependency and package manifest.
- **`Refresh Wallpaper.bat`**: Batch file for instant manual wallpaper refreshes (auto-downloads executable if missing).
- **`setup-task.ps1`**: PowerShell script to schedule the daily task in Windows (auto-downloads executable if missing).
- **`uninstall.ps1`**: PowerShell script to delete the scheduled task.
- **`wallpapers/`** *(Generated)*: Directory where the currently downloaded wallpaper is stored.
- **`.wallpaper_history`** *(Generated)*: Local text database storing the last 50 set wallpaper URLs to avoid repetition.

---

## 🛠️ Building From Source

If you prefer to compile the binary yourself instead of using the precompiled release:

1. Install [Rust and Cargo](https://rustup.rs/).
2. Open a terminal in this directory.
3. Build the release binary:
   ```bash
   cargo build --release
   ```
4. Copy the compiled executable from the target directory to the project root:
   ```powershell
   copy target\release\oled-wallpaper-automator.exe .\
   ```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
