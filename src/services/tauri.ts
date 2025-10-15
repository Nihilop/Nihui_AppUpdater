import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, AddonInfo, AddonDefinition, GitHubRelease, UpdateMode } from '../types';

/**
 * Tauri API Service
 * Wraps all Tauri commands for type safety
 */

export const TauriAPI = {
  // ===========================
  // WoW Path Management
  // ===========================

  /**
   * Auto-scan for WoW installation paths
   */
  async findWowPath(): Promise<string[]> {
    return await invoke<string[]>('find_wow_path');
  },

  /**
   * Validate if a path is a valid WoW installation
   */
  async validateWowPath(path: string): Promise<boolean> {
    return await invoke<boolean>('validate_wow_path', { path });
  },

  // ===========================
  // Addon Management
  // ===========================

  /**
   * Get all Nihui_* addon versions from local installation
   */
  async getLocalAddonVersions(wowPath: string): Promise<AddonInfo[]> {
    return await invoke<AddonInfo[]>('get_local_addon_versions', { wowPath });
  },

  /**
   * Get the hardcoded list of addon definitions
   */
  async getAddonList(): Promise<AddonDefinition[]> {
    return await invoke<AddonDefinition[]>('get_addon_list');
  },

  // ===========================
  // Config Management
  // ===========================

  /**
   * Save app configuration
   */
  async saveConfig(config: AppConfig): Promise<void> {
    return await invoke<void>('save_config', { config });
  },

  /**
   * Load app configuration
   */
  async loadConfig(): Promise<AppConfig> {
    return await invoke<AppConfig>('load_config');
  },

  /**
   * Save addon override configuration
   */
  async saveAddonOverride(addonName: string, updateMode: UpdateMode, branch?: string): Promise<void> {
    return await invoke<void>('save_addon_override', { addonName, updateMode, branch: branch || null });
  },

  // ===========================
  // GitHub API
  // ===========================

  /**
   * Fetch latest release from GitHub
   */
  async fetchGithubRelease(owner: string, repo: string): Promise<GitHubRelease> {
    return await invoke<GitHubRelease>('fetch_github_release', { owner, repo });
  },

  /**
   * Fetch latest commit SHA from a branch
   */
  async fetchGithubBranch(owner: string, repo: string, branch: string): Promise<string> {
    return await invoke<string>('fetch_github_branch', { owner, repo, branch });
  },

  /**
   * Fetch TOC version from GitHub repository
   */
  async fetchGithubToc(owner: string, repo: string, branch: string, addonName: string): Promise<string> {
    return await invoke<string>('fetch_github_toc', { owner, repo, branch, addonName });
  },

  /**
   * Fetch list of branches from GitHub repository
   */
  async fetchGithubBranches(owner: string, repo: string): Promise<string[]> {
    return await invoke<string[]>('fetch_github_branches', { owner, repo });
  },

  /**
   * Fetch README.md from GitHub repository
   */
  async fetchGithubReadme(owner: string, repo: string, branch: string): Promise<string> {
    return await invoke<string>('fetch_github_readme', { owner, repo, branch });
  },

  // ===========================
  // Addon Installation
  // ===========================

  /**
   * Install or update an addon from GitHub
   */
  async installAddon(wowPath: string, addonDef: AddonDefinition): Promise<string> {
    return await invoke<string>('install_addon', { wowPath, addonDef });
  },

  /**
   * Get the path to a tray icon resource
   */
  async getTrayIconPath(iconName: string): Promise<string> {
    return await invoke<string>('get_tray_icon_path', { iconName });
  },

  /**
   * Quit the application completely
   */
  async quitApp(): Promise<void> {
    return await invoke<void>('quit_app');
  },
};
