//! OLED Wallpaper Automator
#![deny(missing_docs)]
#![allow(clippy::duration_suboptimal_units)]
// std
use std::fs;

use std::path::Path;
use std::thread;
use std::time::Duration;

// external crates
use anyhow::{anyhow, Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;
use scraper::{Html, Selector};
use windows::core::HSTRING;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper, DWPOS_FILL};

const BASE_URL: &str = "https://4kwallpapers.com/oled-wallpapers/";
const MAX_RETRIES: u32 = 3;

const RETRY_DELAY: Duration = Duration::from_secs(60);
const MAX_HISTORY: usize = 50;

/// Entry point for the OLED Wallpaper Automator
fn main() {
    // SAFETY: The COM library must be initialized on the current thread before executing COM operations.
    // We ignore the error result in case the COM library is already initialized on this thread.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }

    for attempt in 1..=MAX_RETRIES {
        println!("--- OLED Wallpaper Automator Start (Attempt {attempt}/{MAX_RETRIES}) ---");
        match run() {
            Ok(()) => {
                println!("--- OLED Wallpaper Automator Finished ---");
                return;
            }
            Err(e) => {
                eprintln!("Attempt {attempt} failed: {e}");
                if attempt < MAX_RETRIES {
                    println!("Retrying in {} seconds...", RETRY_DELAY.as_secs());
                    thread::sleep(RETRY_DELAY);
                } else {
                    eprintln!("ALL ATTEMPTS FAILED. Exiting.");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn run() -> Result<()> {
    // 1. Resolve paths relative to executable directory
    let exe_dir = std::env::current_exe()
        .context("Failed to get current executable path")?
        .parent()
        .ok_or_else(|| anyhow!("Failed to get executable directory"))?
        .to_path_buf();
    
    let wallpaper_dir = exe_dir.join("wallpapers");
    let history_path = exe_dir.join(".wallpaper_history");

    // 2. Load history
    let mut history = load_history(&history_path)?;

    // 3. Get list of wallpaper links
    let links = get_wallpaper_links()?;
    
    // 4. Filter links using history
    let selected_url = {
        let mut rng = rand::thread_rng();
        let filtered_links: Vec<String> = links
            .iter()
            .filter(|link| !history.contains(*link))
            .cloned()
            .collect();

        if let Some(link) = filtered_links.choose(&mut rng) {
            link.clone()
        } else {
            println!("All scraped wallpapers are in history. Selecting a random one anyway.");
            links.choose(&mut rng).context("No wallpaper links found at all")?.clone()
        }
    };

    println!("Successfully selected wallpaper: {selected_url}");

    // 5. Download and set wallpaper
    download_and_set_wallpaper(&selected_url, &wallpaper_dir)?;

    // 6. Update history
    add_to_history(&mut history, selected_url, MAX_HISTORY);
    save_history(&history_path, &history)?;

    Ok(())
}

fn load_history(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path).context("Failed to read history file")?;
    let lines = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(lines)
}

fn save_history(path: &Path, history: &[String]) -> Result<()> {
    let content = history.join("\n");
    fs::write(path, content).context("Failed to write history file")?;
    Ok(())
}

// Helper function to update the history list and enforce the maximum size limit.
fn add_to_history(history: &mut Vec<String>, url: String, max_history: usize) {
    history.push(url);
    if history.len() > max_history {
        let excess = history.len() - max_history;
        history.drain(0..excess);
    }
}

fn get_wallpaper_links() -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("Failed to build HTTP client")?;

    println!("Accessing {BASE_URL}...");
    let resp = client.get(BASE_URL).send().context("Failed to send initial request")?;
    let body = resp.text().context("Failed to read initial response body")?;
    
    let document = Html::parse_document(&body);
    let pagination_selector = Selector::parse(".pagination a").map_err(|_| anyhow!("Invalid pagination selector"))?;
    
    let max_pages = if let Some(second_to_last) = document.select(&pagination_selector).rev().nth(1) {
        let text: String = second_to_last.text().collect();
        text.trim().parse::<u32>().unwrap_or(40).max(1)
    } else {
        40
    };
    println!("Detected max pages: {max_pages}");

    let mut rng = rand::thread_rng();
    let random_page = rng.gen_range(1..=max_pages);
    
    let page_url = if random_page == 1 {
        BASE_URL.to_string()
    } else {
        format!("{BASE_URL}?page={random_page}")
    };
    println!("Fetching random page: {page_url}");

    let page_resp = client.get(&page_url).send().context("Failed to fetch page")?;
    let page_body = page_resp.text().context("Failed to read page body")?;
    
    let page_document = Html::parse_document(&page_body);
    let img_selector = Selector::parse("a[href*=\"/images/wallpapers/\"]").map_err(|_| anyhow!("Invalid image selector"))?;
    
    let links: Vec<String> = page_document
        .select(&img_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| {
            if href.starts_with('/') {
                format!("https://4kwallpapers.com{href}")
            } else {
                href.to_string()
            }
        })
        .collect();

    if links.is_empty() {
        return Err(anyhow!("No wallpaper links found on page {random_page}"));
    }

    println!("Found {} wallpapers on page {random_page}", links.len());
    Ok(links)
}

fn download_and_set_wallpaper(url: &str, wallpaper_dir: &Path) -> Result<()> {
    fs::create_dir_all(wallpaper_dir).context("Failed to create wallpaper directory")?;
    
    // Clean old wallpaper files to save space
    if wallpaper_dir.exists() {
        for entry in fs::read_dir(wallpaper_dir).context("Failed to read wallpaper directory")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path).context("Failed to remove old wallpaper file")?;
            }
        }
    }

    let file_name = url.split('/').next_back().unwrap_or("wallpaper.png");
    let file_path = wallpaper_dir.join(file_name);

    println!("Downloading wallpaper to {}...", file_path.display());

    #[allow(clippy::duration_suboptimal_units)]
let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(60))
    .build()
    .context("Failed to build download HTTP client")?;

    let mut resp = client.get(url).send().context("Failed to download image")?;
    let mut file = fs::File::create(&file_path).context("Failed to create image file")?;
    resp.copy_to(&mut file).context("Failed to save image bytes")?;

    println!("Setting wallpaper via COM IDesktopWallpaper interface...");
    
    let file_path_str = file_path.to_str().ok_or_else(|| anyhow!("Invalid path encoding"))?;
    let path_hstring = HSTRING::from(file_path_str);

    // SAFETY: COM must be initialized on this thread (guaranteed by main).
    // CoCreateInstance and interface methods are safe if given valid parameters.
    unsafe {
        // Create the IDesktopWallpaper instance
        let wallpaper_mgr: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
            .map_err(|e| anyhow!("Failed to create DesktopWallpaper COM instance: {e}"))?;

        // Set the wallpaper style to Fill
        wallpaper_mgr.SetPosition(DWPOS_FILL)
            .map_err(|e| anyhow!("Failed to set wallpaper style to Fill: {e}"))?;

        // Set the wallpaper path (None sets it for all monitors)
        wallpaper_mgr.SetWallpaper(None, &path_hstring)
            .map_err(|e| anyhow!("Failed to set wallpaper: {e}"))?;
    }

    println!("Wallpaper set successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn load_history_should_return_empty_when_file_not_found() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().join("non_existent_file_xyz_123.txt");
        let history = load_history(&path)?;
        assert!(history.is_empty(), "Expected empty history list");
        Ok(())
    }

    #[test]
    fn save_and_load_history_should_persist_and_retrieve_correctly() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path();
        
        let original_history = vec![
            "https://example.com/1.png".to_string(),
            "https://example.com/2.png".to_string(),
        ];
        
        save_history(path, &original_history)?;
        let loaded = load_history(path)?;
        
        assert_eq!(loaded, original_history, "Loaded history does not match saved history");
        Ok(())
    }

    #[test]
    fn add_to_history_should_truncate_when_exceeding_max_limit() {
        let mut history = vec![
            "1.png".to_string(),
            "2.png".to_string(),
            "3.png".to_string(),
        ];
        add_to_history(&mut history, "4.png".to_string(), 3);
        
        assert_eq!(history.len(), 3);
        assert_eq!(history, vec!["2.png".to_string(), "3.png".to_string(), "4.png".to_string()]);
    }

    #[test]
    fn load_history_should_trim_and_filter_empty_lines() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path();
        
        // Write raw history with extra spacing and empty lines
        fs::write(path, "  url1.png  \n\n  url2.png\n")?;
        let loaded = load_history(path)?;
        
        assert_eq!(loaded, vec!["url1.png".to_string(), "url2.png".to_string()]);
        Ok(())
    }
}
