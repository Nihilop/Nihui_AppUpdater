// Types matching Rust backend structs

export type UpdateMode = 'release' | 'branch';

export interface AddonOverride {
  update_mode?: UpdateMode;
  branch?: string;
}

export interface AppConfig {
  wow_path: string | null;
  launch_on_startup: boolean;
  minimize_on_startup: boolean;
  language: string;
  addon_overrides: Record<string, AddonOverride>;
}

export interface AddonDefinition {
  local_name: string;
  nice_name: string;
  github_owner: string;
  github_repo: string;
  description: string;
  update_mode: UpdateMode;
  branch?: string;
}

export interface AddonInfo {
  name: string;
  version: string;
  path: string;
}

export interface GitHubRelease {
  tag_name: string;
  name: string;
  published_at: string;
  zipball_url: string;
}

export interface AddonStatus {
  definition: AddonDefinition;
  local_info?: AddonInfo; // undefined if not installed
  remote_version?: string;
  is_installed: boolean;
  update_available: boolean;
  status: 'checking' | 'not-installed' | 'up-to-date' | 'update-available' | 'error';
  error?: string;
}
