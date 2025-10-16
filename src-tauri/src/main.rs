// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use regex::Regex;
use zip::ZipArchive;
use tauri::Manager;

// ===========================
// TYPES & STRUCTS
// ===========================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonOverride {
    pub update_mode: Option<UpdateMode>,
    pub branch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub wow_path: Option<String>,
    #[serde(default = "default_launch_on_startup")]
    pub launch_on_startup: bool,
    #[serde(default = "default_minimize_on_startup")]
    pub minimize_on_startup: bool,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub addon_overrides: HashMap<String, AddonOverride>,
}

fn default_launch_on_startup() -> bool {
    true
}

fn default_minimize_on_startup() -> bool {
    true
}

fn default_language() -> String {
    "".to_string() // Empty string = auto-detect system language in frontend
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateMode {
    Release,
    Branch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonDefinition {
    pub local_name: String,
    pub nice_name: String,
    pub github_owner: String,
    pub github_repo: String,
    pub description: String,
    pub update_mode: UpdateMode,
    pub branch: Option<String>, // Branch name for branch mode
}

// Hardcoded list of Nihui addons
fn get_addon_definitions() -> Vec<AddonDefinition> {
    vec![
        AddonDefinition {
            local_name: "Nihui_uf".to_string(),
            nice_name: "Unit Frames".to_string(),
            github_owner: "Nihilop".to_string(),
            github_repo: "Nihui_unitframe".to_string(),
            description: "Unit frames addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        AddonDefinition {
            local_name: "Nihui_ab".to_string(),
            nice_name: "Action Bars".to_string(),
            github_owner: "Nihilop".to_string(),
            github_repo: "Nihui_actionbars".to_string(),
            description: "Action bars addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        AddonDefinition {
            local_name: "Nihui_iv".to_string(),
            nice_name: "Inventory".to_string(),
            github_owner: "Nihilop".to_string(),
            github_repo: "Nihui_inventory".to_string(),
            description: "Inventory addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        AddonDefinition {
            local_name: "Nihui_cb".to_string(),
            nice_name: "Cast Bars".to_string(),
            github_owner: "Nihilop".to_string(),
            github_repo: "Nihui_castbars".to_string(),
            description: "Castbars addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        AddonDefinition {
            local_name: "Nihui_np".to_string(),
            nice_name: "Nameplates".to_string(),
            github_owner: "Nihilop".to_string(),
            github_repo: "Nihui_nameplate".to_string(),
            description: "Nameplate addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        AddonDefinition {
            local_name: "Nihui_chat".to_string(),
            nice_name: "Nihui Chatbox".to_string(),
            github_owner: "Nihilop".to_string(),
            github_repo: "Nihui_chat".to_string(),
            description: "Chatbox addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        AddonDefinition {
            local_name: "WaypointUI".to_string(),
            nice_name: "Waypoint UI".to_string(),
            github_owner: "Adaptvx".to_string(),
            github_repo: "Waypoint-UI".to_string(),
            description: "waypoint addon".to_string(),
            update_mode: UpdateMode::Branch,
            branch: Some("main".to_string()),
        },
        // Add more addons here as needed
        // Example with branch mode:
        // AddonDefinition {
        //     local_name: "Nihui_nameplate".to_string(),
        //     github_owner: "Nihilop".to_string(),
        //     github_repo: "Nihui_nameplate".to_string(),
        //     description: "Nameplate addon".to_string(),
        //     update_mode: UpdateMode::Branch,
        //     branch: Some("dev".to_string()),
        // },
    ]
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonInfo {
    pub name: String,
    pub version: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub published_at: String,
    pub zipball_url: String,
}

// ===========================
// WOW PATH SCANNER
// ===========================

/// Auto-scan for WoW installation paths on Windows
#[tauri::command]
fn find_wow_path() -> Result<Vec<String>, String> {
    let mut paths = Vec::new();

    // Try to read Battle.net product database
    #[cfg(target_os = "windows")]
    {
        // Battle.net product.db is in C:\ProgramData\Battle.net\Agent\product.db
        let product_db = PathBuf::from("C:\\ProgramData\\Battle.net\\Agent\\product.db");

        if product_db.exists() {
            // Read as bytes since it's a binary file
            if let Ok(bytes) = fs::read(&product_db) {
                // Convert bytes to string, replacing invalid UTF-8 with replacement char
                let content = String::from_utf8_lossy(&bytes);

                // Extract Windows paths that contain "World of Warcraft"
                // Looking for patterns like "E:/Battle.net/World of Warcraft"
                let re = Regex::new(r"([A-Z]:[/\\][^\x00-\x1F]*World of Warcraft)").unwrap();
                for cap in re.captures_iter(&content) {
                    if let Some(install_path) = cap.get(1) {
                        let mut path_str = install_path.as_str().to_string();

                        // Clean up the path - remove any trailing garbage characters
                        if let Some(pos) = path_str.find(|c: char| c < ' ') {
                            path_str = path_str[..pos].to_string();
                        }

                        // Normalize path separators to backslash for Windows
                        path_str = path_str.replace("/", "\\");

                        let path = PathBuf::from(&path_str);
                        if validate_wow_path_internal(&path) {
                            if !paths.contains(&path_str) {
                                paths.push(path_str);
                            }
                        }
                    }
                }
            }
        }
    }

    if paths.is_empty() {
        Err("No WoW installation found. Please select manually.".to_string())
    } else {
        Ok(paths)
    }
}

/// Validate if a path is a valid WoW _retail_ directory
#[tauri::command]
fn validate_wow_path(path: String) -> Result<bool, String> {
    let path_buf = PathBuf::from(&path);
    Ok(validate_wow_path_internal(&path_buf))
}

fn validate_wow_path_internal(path: &Path) -> bool {
    // Check if _retail_/Interface/AddOns exists
    let retail_path = path.join("_retail_");
    let interface_path = retail_path.join("Interface");
    let addons_path = interface_path.join("AddOns");

    retail_path.exists() && interface_path.exists() && addons_path.exists()
}

// ===========================
// ADDON VERSION READER
// ===========================

/// Read addon versions from local installation (based on addon definitions)
#[tauri::command]
fn get_local_addon_versions(wow_path: String) -> Result<Vec<AddonInfo>, String> {
    let addons_path = PathBuf::from(&wow_path)
        .join("_retail_")
        .join("Interface")
        .join("AddOns");

    if !addons_path.exists() {
        return Err("AddOns directory not found".to_string());
    }

    let mut addons = Vec::new();

    // Get list of addons to scan from definitions
    let addon_definitions = get_addon_definitions();
    let addon_names: Vec<String> = addon_definitions.iter()
        .map(|def| def.local_name.clone())
        .collect();

    // Read all directories
    if let Ok(entries) = fs::read_dir(&addons_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();

                    // Check if this directory matches any addon in our definitions
                    if addon_names.contains(&dir_name) {
                        let toc_path = entry.path().join(format!("{}.toc", dir_name));

                        if let Ok(version) = read_version_from_toc(&toc_path) {
                            addons.push(AddonInfo {
                                name: dir_name.clone(),
                                version,
                                path: entry.path().to_string_lossy().to_string(),
                            });
                        } else {
                            // No version found, add as "unknown"
                            addons.push(AddonInfo {
                                name: dir_name.clone(),
                                version: "unknown".to_string(),
                                path: entry.path().to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(addons)
}

/// Read version from a .toc file
fn read_version_from_toc(toc_path: &Path) -> Result<String, String> {
    if !toc_path.exists() {
        return Err("TOC file not found".to_string());
    }

    let content = fs::read_to_string(toc_path)
        .map_err(|e| format!("Failed to read TOC file: {}", e))?;

    // Look for ## Version: line
    let re = Regex::new(r"(?m)^##\s*Version:\s*(.+)$").unwrap();

    if let Some(cap) = re.captures(&content) {
        if let Some(version) = cap.get(1) {
            return Ok(version.as_str().trim().to_string());
        }
    }

    Err("Version not found in TOC file".to_string())
}

// ===========================
// CONFIG MANAGEMENT
// ===========================

fn get_config_path() -> PathBuf {
    let app_data = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    app_data.join("nihui_app").join("config.json")
}

/// Save app configuration
#[tauri::command]
fn save_config(config: AppConfig, app: tauri::AppHandle) -> Result<(), String> {
    let config_path = get_config_path();

    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // Apply autostart setting
    apply_autostart_setting(config.launch_on_startup, &app)?;

    Ok(())
}

/// Apply autostart setting
fn apply_autostart_setting(enabled: bool, app: &tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;

    let autostart_manager = app.autolaunch();

    if enabled {
        autostart_manager.enable()
            .map_err(|e| format!("Failed to enable autostart: {}", e))?;
    } else {
        autostart_manager.disable()
            .map_err(|e| format!("Failed to disable autostart: {}", e))?;
    }

    Ok(())
}

/// Load app configuration
#[tauri::command]
fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path();

    if !config_path.exists() {
        // Return default config
        return Ok(AppConfig {
            wow_path: None,
            launch_on_startup: true,
            minimize_on_startup: true,
            language: "en".to_string(),
            addon_overrides: HashMap::new(),
        });
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    Ok(config)
}

/// Get the hardcoded list of addon definitions with overrides applied
#[tauri::command]
fn get_addon_list() -> Result<Vec<AddonDefinition>, String> {
    let mut addons = get_addon_definitions();

    // Load config to get overrides
    if let Ok(config) = load_config() {
        // Apply overrides to each addon
        for addon in &mut addons {
            if let Some(override_config) = config.addon_overrides.get(&addon.local_name) {
                // Apply update_mode override if present
                if let Some(update_mode) = &override_config.update_mode {
                    addon.update_mode = update_mode.clone();
                }
                // Apply branch override if present
                if let Some(branch) = &override_config.branch {
                    addon.branch = Some(branch.clone());
                }
            }
        }
    }

    Ok(addons)
}

/// Save addon override configuration
#[tauri::command]
fn save_addon_override(
    addon_name: String,
    update_mode: UpdateMode,
    branch: Option<String>,
) -> Result<(), String> {
    let config_path = get_config_path();

    // Load existing config
    let mut config = load_config().unwrap_or(AppConfig {
        wow_path: None,
        launch_on_startup: true,
        minimize_on_startup: true,
        language: "en".to_string(),
        addon_overrides: HashMap::new(),
    });

    // Update override for this addon
    config.addon_overrides.insert(
        addon_name,
        AddonOverride {
            update_mode: Some(update_mode),
            branch,
        },
    );

    // Save config
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

// ===========================
// GITHUB API
// ===========================

/// Fetch latest release from GitHub
#[tauri::command]
async fn fetch_github_release(owner: String, repo: String) -> Result<GitHubRelease, String> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);

    let client = reqwest::Client::builder()
        .user_agent("Nihui-App/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch release: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }

    let release: GitHubRelease = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse release JSON: {}", e))?;

    Ok(release)
}

/// Fetch latest commit from a branch
#[tauri::command]
async fn fetch_github_branch(owner: String, repo: String, branch: String) -> Result<String, String> {
    let url = format!("https://api.github.com/repos/{}/{}/commits/{}", owner, repo, branch);

    let client = reqwest::Client::builder()
        .user_agent("Nihui-App/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch branch: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse commit JSON: {}", e))?;

    // Get the short SHA
    let sha = json["sha"]
        .as_str()
        .ok_or("Failed to get commit SHA")?
        .chars()
        .take(7)
        .collect::<String>();

    Ok(sha)
}

/// Fetch TOC file content from GitHub for version comparison
#[tauri::command]
async fn fetch_github_toc(owner: String, repo: String, branch: String, addon_name: String) -> Result<String, String> {
    // Raw GitHub URL: https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{addon_name}.toc
    // The .toc file is at the root of the repo
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}.toc",
        owner, repo, branch, addon_name
    );

    let client = reqwest::Client::builder()
        .user_agent("Nihui-App/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch TOC: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub error: {}", response.status()));
    }

    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read TOC content: {}", e))?;

    // Extract version from TOC content
    let re = Regex::new(r"(?m)^##\s*Version:\s*(.+)$").unwrap();

    if let Some(cap) = re.captures(&content) {
        if let Some(version) = cap.get(1) {
            return Ok(version.as_str().trim().to_string());
        }
    }

    Err("Version not found in TOC file".to_string())
}

/// Fetch list of branches from GitHub repository
#[tauri::command]
async fn fetch_github_branches(owner: String, repo: String) -> Result<Vec<String>, String> {
    let url = format!("https://api.github.com/repos/{}/{}/branches", owner, repo);

    let client = reqwest::Client::builder()
        .user_agent("Nihui-App/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch branches: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }

    let branches: Vec<serde_json::Value> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse branches JSON: {}", e))?;

    let branch_names: Vec<String> = branches
        .iter()
        .filter_map(|b| b["name"].as_str().map(|s| s.to_string()))
        .collect();

    Ok(branch_names)
}

/// Fetch README.md content from GitHub
#[tauri::command]
async fn fetch_github_readme(owner: String, repo: String, branch: String) -> Result<String, String> {
    // Try README.md first, then README.MD, then readme.md
    let possible_names = vec!["README.md", "README.MD", "readme.md"];

    let client = reqwest::Client::builder()
        .user_agent("Nihui-App/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    for readme_name in possible_names {
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            owner, repo, branch, readme_name
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch README: {}", e))?;

        if response.status().is_success() {
            let content = response
                .text()
                .await
                .map_err(|e| format!("Failed to read README content: {}", e))?;

            return Ok(content);
        }
    }

    Err("README.md not found in repository".to_string())
}

/// Quit the application completely
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

/// Get the current app version
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ===========================
// TRAY ICON RESOURCES
// ===========================

/// Get the path to a tray icon resource
#[tauri::command]
fn get_tray_icon_path(app: tauri::AppHandle, icon_name: String) -> Result<String, String> {
    // Try multiple strategies to find icons
    let mut search_paths = Vec::new();

    // Strategy 1: Check resource directory (for installed apps)
    if let Ok(resource_dir) = app.path().resource_dir() {
        search_paths.push(resource_dir.join("icons").join(&icon_name));
    }

    // Strategy 2: Check relative to exe directory (for standalone exe)
    if let Ok(exe_dir) = std::env::current_exe().and_then(|exe| {
        exe.parent()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No parent"))
            .map(|p| p.to_path_buf())
    }) {
        search_paths.push(exe_dir.join("icons").join(&icon_name));
        search_paths.push(exe_dir.join("resources").join("icons").join(&icon_name));
    }

    // Strategy 3: In development, check icons/tray directory
    #[cfg(debug_assertions)]
    {
        if let Ok(current_dir) = std::env::current_dir() {
            search_paths.push(current_dir.join("icons").join("tray").join(&icon_name));
        }
    }

    // Try each path until we find the icon
    for path in &search_paths {
        if path.exists() {
            return path
                .to_str()
                .map(|s| s.to_string())
                .ok_or_else(|| "Failed to convert path to string".to_string());
        }
    }

    // If we get here, icon was not found in any location
    Err(format!(
        "Icon '{}' not found. Searched in:\n{}",
        icon_name,
        search_paths
            .iter()
            .map(|p| format!("  - {:?}", p))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

// ===========================
// ADDON INSTALLATION
// ===========================

/// Install or update an addon from GitHub
#[tauri::command]
async fn install_addon(
    wow_path: String,
    addon_def: AddonDefinition,
) -> Result<String, String> {
    // Build download URL based on update mode
    let download_url = match addon_def.update_mode {
        UpdateMode::Release => {
            // Get latest release to get zipball URL
            let release = fetch_github_release(
                addon_def.github_owner.clone(),
                addon_def.github_repo.clone(),
            )
            .await?;
            release.zipball_url
        }
        UpdateMode::Branch => {
            let branch = addon_def.branch.as_ref().ok_or("Branch name is required for branch mode")?;
            format!(
                "https://github.com/{}/{}/archive/refs/heads/{}.zip",
                addon_def.github_owner, addon_def.github_repo, branch
            )
        }
    };

    // Download ZIP
    let client = reqwest::Client::builder()
        .user_agent("Nihui-App/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download addon: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()));
    }

    let zip_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read download data: {}", e))?;

    // Create temp directory for extraction
    let temp_dir = tempfile::tempdir()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    // Extract ZIP
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(cursor)
        .map_err(|e| format!("Failed to open ZIP archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read ZIP entry: {}", e))?;

        let outpath = match file.enclosed_name() {
            Some(path) => temp_dir.path().join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to extract file: {}", e))?;
        }
    }

    // Find the addon folder in the extracted content
    // GitHub archives have structure: repo-name-branch/ (for branch mode) or repo-name-sha/ (for release mode)
    // The addon files are directly in this root folder
    let addon_folder_name = &addon_def.local_name;
    let toc_file_name = format!("{}.toc", addon_folder_name);

    // Find the root folder that contains the .toc file
    let mut addon_source_path: Option<PathBuf> = None;

    if let Ok(entries) = fs::read_dir(temp_dir.path()) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    // Check if this directory contains the .toc file
                    let toc_path = entry.path().join(&toc_file_name);
                    if toc_path.exists() {
                        addon_source_path = Some(entry.path());
                        break;
                    }
                }
            }
        }
    }

    let source_path = addon_source_path.ok_or_else(|| {
        format!("Addon files with '{}' not found in the downloaded archive", toc_file_name)
    })?;

    // Destination path in WoW AddOns directory
    let addons_path = PathBuf::from(&wow_path)
        .join("_retail_")
        .join("Interface")
        .join("AddOns");

    let dest_path = addons_path.join(addon_folder_name);

    // Remove existing addon if present
    if dest_path.exists() {
        fs::remove_dir_all(&dest_path)
            .map_err(|e| format!("Failed to remove existing addon: {}", e))?;
    }

    // Copy addon to WoW AddOns directory
    copy_dir_all(&source_path, &dest_path)?;

    Ok(format!("Successfully installed {} to {}", addon_folder_name, dest_path.display()))
}

/// Recursively copy a directory
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst)
        .map_err(|e| format!("Failed to create destination directory: {}", e))?;

    for entry in fs::read_dir(src)
        .map_err(|e| format!("Failed to read source directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let ty = entry.file_type()
            .map_err(|e| format!("Failed to get file type: {}", e))?;

        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }
    Ok(())
}

// ===========================
// ADDON UNINSTALLATION
// ===========================

/// Uninstall an addon by removing its directory
#[tauri::command]
fn uninstall_addon(
    wow_path: String,
    addon_name: String,
) -> Result<String, String> {
    // Build path to addon directory
    let addon_path = PathBuf::from(&wow_path)
        .join("_retail_")
        .join("Interface")
        .join("AddOns")
        .join(&addon_name);

    // Check if addon exists
    if !addon_path.exists() {
        return Err(format!("Addon '{}' is not installed", addon_name));
    }

    // Remove addon directory
    fs::remove_dir_all(&addon_path)
        .map_err(|e| format!("Failed to uninstall addon: {}", e))?;

    Ok(format!("Successfully uninstalled {}", addon_name))
}

// ===========================
// MAIN
// ===========================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))
        .invoke_handler(tauri::generate_handler![
            find_wow_path,
            validate_wow_path,
            get_local_addon_versions,
            get_addon_list,
            save_config,
            load_config,
            save_addon_override,
            fetch_github_release,
            fetch_github_branch,
            fetch_github_branches,
            fetch_github_toc,
            fetch_github_readme,
            get_tray_icon_path,
            install_addon,
            uninstall_addon,
            quit_app,
            get_app_version,
        ])
        .setup(|app| {
            // Check if app was launched with --minimized flag
            let args: Vec<String> = std::env::args().collect();
            let launched_minimized = args.iter().any(|arg| arg == "--minimized");

            if launched_minimized {
                // Load config to check minimize_on_startup setting
                if let Ok(config) = load_config() {
                    if config.minimize_on_startup {
                        // Hide the main window on startup
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Prevent window from closing and hide it instead
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
