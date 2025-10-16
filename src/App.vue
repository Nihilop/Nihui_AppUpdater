<script setup lang="ts">
import { TrayIcon } from '@tauri-apps/api/tray';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Image } from '@tauri-apps/api/image';
import { onMounted, onUnmounted, ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { TauriAPI } from './services/tauri';
import { NotificationService } from './services/notification';
import type { AppConfig, AddonDefinition, AddonStatus } from './types';
import TitleBar from './components/blocks/TitleBar.vue';
import WowStatus from './components/blocks/WowStatus.vue';
import AddonListItem from './components/blocks/AddonListItem.vue';
import SettingsModal from './components/blocks/SettingsModal.vue';
import AddonConfigModal from './components/blocks/AddonConfigModal.vue';
import UpdateDialog from './components/blocks/UpdateDialog.vue';
import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { enable as enableAutostart, disable as disableAutostart, isEnabled as isAutostartEnabled } from '@tauri-apps/plugin-autostart';
import { check as checkUpdate } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

// ===========================
// I18N
// ===========================

const { locale, t } = useI18n();

// ===========================
// STATE
// ===========================

const config = ref<AppConfig>({
  wow_path: null,
  launch_on_startup: true,
  minimize_on_startup: true,
  language: 'en',
  addon_overrides: {},
});

const addonDefinitions = ref<AddonDefinition[]>([]);
const addons = ref<AddonStatus[]>([]);
const autoScanResults = ref<string[]>([]);
const isScanning = ref(false);
const showSettingsModal = ref(false);
const showAddonConfigModal = ref(false);
const selectedAddon = ref<AddonStatus | null>(null);

// App updater state
const showUpdateDialog = ref(false);
const appCurrentVersion = ref('');
const appNewVersion = ref('');
let pendingUpdate: any = null;

// Tray icon reference
let trayIcon: TrayIcon | null = null;

// Auto-check configuration (configurable)
let autoCheckTimer: number | null = null;
const AUTO_CHECK_INTERVAL = ref(60 * 60 * 1000); // 1 hour

// ===========================
// COMPUTED
// ===========================

const hasWowPath = computed(() => config.value.wow_path !== null);

const installedCount = computed(() => {
  return addons.value.filter(a => a.is_installed).length;
});

const updateAvailableCount = computed(() => {
  return addons.value.filter(a => a.update_available).length;
});

// ===========================
// LIFECYCLE
// ===========================

// ===========================
// TRAY ICON HANDLERS
// ===========================

async function showWindow() {
  const window = getCurrentWindow();
  await window.show();
  await window.setFocus();
}

/**
 * Update tray icon and tooltip based on current status
 */
async function updateTrayStatus() {
  if (trayIcon === null) return;

  let statusText = 'Nihui Addon Updater';
  let iconName = 'tray-normal.png';

  if (!hasWowPath.value) {
    // Red: No WoW path configured
    iconName = 'error.png';
    statusText += ' - WoW path not configured';
  } else if (updateAvailableCount.value > 0) {
    // Orange: Updates available
    iconName = 'warning.png';
    statusText += ` - ${updateAvailableCount.value} update${updateAvailableCount.value > 1 ? 's' : ''} available`;
  } else if (installedCount.value > 0) {
    // Green: All up to date
    iconName = 'success.png';
    statusText += ' - All addons up to date';
  }

  try {
    const iconPath = await TauriAPI.getTrayIconPath(iconName);
    const icon = await Image.fromPath(iconPath);
    await trayIcon.setIcon(icon);
    await trayIcon.setTooltip(statusText);
  } catch (error) {
    console.error('Failed to update tray icon:', error);
    // Just update tooltip if icon fails
    try {
      await trayIcon.setTooltip(statusText);
    } catch (e) {
      console.error('Failed to update tooltip:', e);
    }
  }
}

async function quitApp() {
  try {
    // Clean up tray icon before exit
    if (trayIcon !== null) {
      await trayIcon.close();
    }
    // Exit the application completely
    await TauriAPI.quitApp();
  } catch (error) {
    console.error('Failed to quit app:', error);
    // Force exit even if cleanup fails
    try {
      await TauriAPI.quitApp();
    } catch (e) {
      console.error('Force quit failed:', e);
    }
  }
}

async function setupTrayIcon() {
  const TRAY_ID = 'nihui-updater-tray'; // Fixed ID for tray icon

  // Clean up existing tray icon by ID first (in case of hot-reload)
  try {
    const existingTray = await TrayIcon.getById(TRAY_ID);
    if (existingTray) {
      await existingTray.close();
    }
  } catch (error) {
    // No existing tray found, this is fine
  }

  // Reset local reference
  trayIcon = null;

  // Create menu for right-click with Quit option
  const menu = await Menu.new({
    items: [
      await MenuItem.new({
        id: 'quit',
        text: 'Quit',
        action: quitApp,
      }),
    ],
  });

  // Create tray icon with event handling
  try {
    const iconPath = await TauriAPI.getTrayIconPath('tray-normal.png');
    const initialIcon = await Image.fromPath(iconPath);
    trayIcon = await TrayIcon.new({
      id: TRAY_ID, // Use fixed ID
      icon: initialIcon,
      tooltip: 'Nihui Addon Updater',
      menu,
      menuOnLeftClick: false, // Don't show menu on left click
      action: async (event) => {
        // Handle tray icon click events
        if (event.type === 'Click') {
          // Left click: show window
          if (event.button === 'Left') {
            await showWindow();
          }
          // Right click shows menu automatically
        }
      },
    });
  } catch (error) {
    console.error('Failed to create tray icon with custom icon:', error);
    // Try without custom icon as fallback
    try {
      trayIcon = await TrayIcon.new({
        id: TRAY_ID, // Use fixed ID even in fallback
        tooltip: 'Nihui Addon Updater',
        menu,
        menuOnLeftClick: false,
        action: async (event) => {
          if (event.type === 'Click' && event.button === 'Left') {
            await showWindow();
          }
        },
      });
    } catch (e) {
      console.error('Failed to create tray icon entirely:', e);
      return; // Early return if tray creation fails completely
    }
  }

  // Initial status update
  await updateTrayStatus();
}

onMounted(async () => {
  // Load app version from backend
  try {
    appCurrentVersion.value = await TauriAPI.getAppVersion();
  } catch (error) {
    console.error('Failed to get app version:', error);
    appCurrentVersion.value = '0.1.0'; // Fallback
  }

  // Load config first
  await loadConfig();

  // Load addon definitions
  await loadAddonDefinitions();

  // Setup tray icon with a small delay to ensure backend is ready
  setTimeout(async () => {
    await setupTrayIcon();
  }, 500);

  // Auto-scan if no WoW path configured
  if (!config.value.wow_path) {
    await autoScanWowPath();
  } else {
    // Load addons if path exists
    await loadAddons();
  }

  // Show modal if still no path after scan
  if (!config.value.wow_path) {
    showSettingsModal.value = true;
  }

  // Initialize notification service
  // Note: Click handlers on Windows notifications are not supported
  // Users will click the tray icon (which turns orange) to open the app
  await NotificationService.initialize();

  // Start auto-check timer (checks every hour by default)
  startAutoCheckTimer();

  // Check for app updates (after a small delay)
  setTimeout(() => {
    checkForAppUpdate();
  }, 3000);
});

onUnmounted(async () => {
  // Clean up timer when component is unmounted
  stopAutoCheckTimer();

  // Clean up tray icon
  if (trayIcon !== null) {
    try {
      // Remove tray icon when component unmounts
      // This prevents multiple tray icons during hot-reload in dev
      await trayIcon.close();
      trayIcon = null;
    } catch (error) {
      console.error('Failed to cleanup tray icon:', error);
    }
  }
});

// ===========================
// WOW PATH MANAGEMENT
// ===========================

async function autoScanWowPath() {
  isScanning.value = true;

  try {
    const paths = await TauriAPI.findWowPath();
    autoScanResults.value = paths;

    if (paths.length === 1) {
      // Only one path found, auto-select it
      config.value.wow_path = paths[0];
      await saveConfig();
      await loadAddons();
      showSettingsModal.value = false;
    } else if (paths.length > 1) {
      // Multiple paths found, show modal
      showSettingsModal.value = true;
    } else {
      // No paths found
      showSettingsModal.value = true;
    }
  } catch (error) {
    console.error('Auto-scan failed:', error);
  } finally {
    isScanning.value = false;
  }
}

async function selectWowPath(path: string) {
  config.value.wow_path = path;
  await saveConfig();
  await loadAddons();
  autoScanResults.value = [];
  showSettingsModal.value = false;
  await updateTrayStatus();
}

async function validateManualPath(path: string) {
  try {
    const isValid = await TauriAPI.validateWowPath(path);

    if (isValid) {
      config.value.wow_path = path;
      await saveConfig();
      await loadAddons();
      showSettingsModal.value = false;
      await updateTrayStatus();
    } else {
      console.error('Invalid WoW path');
    }
  } catch (error) {
    console.error('Validation failed:', error);
  }
}

// ===========================
// ADDON MANAGEMENT
// ===========================

async function loadAddonDefinitions() {
  try {
    addonDefinitions.value = await TauriAPI.getAddonList();
  } catch (error) {
    console.error('Failed to load addon definitions:', error);
  }
}

async function loadAddons() {
  if (!config.value.wow_path) return;

  try {
    const localAddons = await TauriAPI.getLocalAddonVersions(config.value.wow_path);

    // Create addon status for each definition
    addons.value = addonDefinitions.value.map(definition => {
      const localInfo = localAddons.find(addon => addon.name === definition.local_name);

      return {
        definition,
        local_info: localInfo,
        is_installed: !!localInfo,
        update_available: false,
        status: localInfo ? 'up-to-date' as const : 'not-installed' as const,
      };
    });

    // Auto-check all installed addons after load
    for (const addon of addons.value) {
      if (addon.is_installed) {
        await checkAddonUpdate(addon);
      }
    }

    // Update tray status after loading
    await updateTrayStatus();
  } catch (error) {
    console.error('Failed to load addons:', error);
  }
}

// ===========================
// UPDATE CHECKING
// ===========================

async function checkAddonUpdate(addon: AddonStatus) {
  const localVersion = addon.local_info?.version || 'unknown';

  if (localVersion === 'unknown') {
    addon.status = 'error';
    addon.error = 'Version not found in .toc file';
    addon.update_available = false;
    return;
  }

  try {
    if (addon.definition.update_mode === 'release') {
      // Release mode: compare with GitHub release tag
      const release = await TauriAPI.fetchGithubRelease(
        addon.definition.github_owner,
        addon.definition.github_repo
      );

      const remoteVersion = release.tag_name.replace(/^v/, '');
      addon.remote_version = remoteVersion;

      if (localVersion !== remoteVersion) {
        addon.update_available = true;
        addon.status = 'update-available';
      } else {
        addon.update_available = false;
        addon.status = 'up-to-date';
      }
    } else {
      // Branch mode: compare TOC version from GitHub branch
      const branch = addon.definition.branch || 'main';
      const remoteVersion = await TauriAPI.fetchGithubToc(
        addon.definition.github_owner,
        addon.definition.github_repo,
        branch,
        addon.definition.local_name
      );

      addon.remote_version = remoteVersion;

      if (localVersion !== remoteVersion) {
        addon.update_available = true;
        addon.status = 'update-available';
      } else {
        addon.update_available = false;
        addon.status = 'up-to-date';
      }
    }
  } catch (error) {
    addon.status = 'error';
    addon.error = `Failed to check updates: ${error}`;
    addon.update_available = false;
  }
}

async function checkSingleAddon(addon: AddonStatus) {
  addon.status = 'checking';
  addon.error = undefined;

  if (addon.is_installed) {
    await checkAddonUpdate(addon);
  } else {
    // For non-installed addons, just fetch remote version
    try {
      if (addon.definition.update_mode === 'release') {
        const release = await TauriAPI.fetchGithubRelease(
          addon.definition.github_owner,
          addon.definition.github_repo
        );
        addon.remote_version = release.tag_name.replace(/^v/, '');
      } else {
        const branch = addon.definition.branch || 'main';
        const remoteVersion = await TauriAPI.fetchGithubToc(
          addon.definition.github_owner,
          addon.definition.github_repo,
          branch,
          addon.definition.local_name
        );
        addon.remote_version = remoteVersion;
      }
      addon.status = 'not-installed';
    } catch (error) {
      addon.status = 'error';
      addon.error = `Failed to check remote version: ${error}`;
    }
  }

  // Update tray status after check
  await updateTrayStatus();
}

// ===========================
// AUTO-CHECK & NOTIFICATIONS
// ===========================

/**
 * Background check for updates (silent, sends notification if updates found)
 */
async function checkForUpdatesBackground() {
  if (!hasWowPath.value || installedCount.value === 0) {
    return;
  }

  try {
    // Check all installed addons silently
    for (const addon of addons.value) {
      if (addon.is_installed) {
        await checkAddonUpdate(addon);
      }
    }

    const updatesCount = updateAvailableCount.value;

    if (updatesCount > 0) {
      // Get list of addon names with updates
      const addonNames = addons.value
        .filter(a => a.update_available)
        .map(a => a.definition.local_name);

      // Send system notification
      await NotificationService.sendUpdateNotification(updatesCount, addonNames);
    }

    // Update tray status after background check
    await updateTrayStatus();
  } catch (error) {
    console.error('[Auto-Check] Failed to check updates:', error);
  }
}

/**
 * Start the hourly auto-check timer
 */
function startAutoCheckTimer() {
  if (autoCheckTimer !== null) {
    return; // Timer already running
  }

  // Run first check after 5 minutes
  setTimeout(() => {
    checkForUpdatesBackground();
  }, 5 * 60 * 1000);

  // Then check every interval
  autoCheckTimer = window.setInterval(() => {
    checkForUpdatesBackground();
  }, AUTO_CHECK_INTERVAL.value);
}

/**
 * Stop the auto-check timer
 */
function stopAutoCheckTimer() {
  if (autoCheckTimer !== null) {
    clearInterval(autoCheckTimer);
    autoCheckTimer = null;
  }
}

// ===========================
// INSTALL/UPDATE ACTIONS
// ===========================

async function installAddon(addon: AddonStatus) {
  if (!config.value.wow_path) {
    console.error('WoW path not configured');
    return;
  }

  addon.status = 'checking';

  try {
    await TauriAPI.installAddon(config.value.wow_path, addon.definition);

    // Mark addon as updated so user can be notified again if new update comes
    NotificationService.markAddonAsUpdated(addon.definition.local_name);

    // Reload addons to update the list
    await loadAddons();
    await updateTrayStatus();
  } catch (error) {
    addon.status = 'error';
    addon.error = `Installation failed: ${error}`;
    console.error('Failed to install addon:', error);
  }
}

async function updateAddon(addon: AddonStatus) {
  if (!config.value.wow_path) {
    console.error('WoW path not configured');
    return;
  }

  addon.status = 'checking';

  try {
    await TauriAPI.installAddon(config.value.wow_path, addon.definition);

    // Mark addon as updated so user can be notified again if new update comes
    NotificationService.markAddonAsUpdated(addon.definition.local_name);

    // Reload addons to update the list
    await loadAddons();
    await updateTrayStatus();
  } catch (error) {
    addon.status = 'error';
    addon.error = `Update failed: ${error}`;
    console.error('Failed to update addon:', error);
  }
}

/**
 * Silent update (used when clicking on notification)
 * Currently not used but kept for future implementation
 */
// async function _updateAddonSilent(addon: AddonStatus) {
//   if (!config.value.wow_path) return;
//
//   try {
//     await TauriAPI.installAddon(config.value.wow_path, addon.definition);
//   } catch (error) {
//     console.error(`Silent update failed for ${addon.definition.local_name}:`, error);
//   }
// }

// ===========================
// CONFIG MANAGEMENT
// ===========================

async function saveConfig() {
  try {
    await TauriAPI.saveConfig(config.value);
  } catch (error) {
    console.error('Failed to save config:', error);
  }
}

async function loadConfig() {
  try {
    const loadedConfig = await TauriAPI.loadConfig();
    config.value = loadedConfig;

    // Detect system language only if not configured (first launch)
    let languageToUse = loadedConfig.language;

    if (!languageToUse) {
      // First launch: detect browser/system language
      const browserLang = navigator.language.toLowerCase();

      // Map browser language to supported locales
      if (browserLang.startsWith('fr')) {
        languageToUse = 'fr';
      } else if (browserLang.startsWith('es')) {
        languageToUse = 'es';
      } else if (browserLang.startsWith('de')) {
        languageToUse = 'de';
      } else if (browserLang.startsWith('it')) {
        languageToUse = 'it';
      } else if (browserLang.startsWith('pt')) {
        languageToUse = 'pt';
      } else {
        languageToUse = 'en'; // Default fallback
      }

      // Save detected language to config
      config.value.language = languageToUse;
      await saveConfig();
    }

    // Apply language
    locale.value = languageToUse;

    // Check the REAL autostart state from Windows Registry
    try {
      const actualAutostartState = await isAutostartEnabled();
      // Override with real state
      config.value.launch_on_startup = actualAutostartState;
    } catch (error) {
      console.error('Failed to check autostart state:', error);
    }
  } catch (error) {
    console.error('Failed to load config:', error);
  }
}

async function updateLaunchOnStartup(enabled: boolean) {
  try {
    if (enabled) {
      await enableAutostart();
    } else {
      await disableAutostart();
    }

    // Update config to reflect the change
    config.value.launch_on_startup = enabled;
    await saveConfig();
  } catch (error) {
    console.error('Failed to update autostart:', error);
    // Revert the UI state if the operation failed
    config.value.launch_on_startup = !enabled;
  }
}

async function updateMinimizeOnStartup(enabled: boolean) {
  try {
    // Update config to reflect the change
    config.value.minimize_on_startup = enabled;
    await saveConfig();
  } catch (error) {
    console.error('Failed to update minimize on startup:', error);
    // Revert the UI state if the operation failed
    config.value.minimize_on_startup = !enabled;
  }
}

async function updateLanguage(newLanguage: string) {
  try {
    // Update locale
    locale.value = newLanguage;
    // Update config
    config.value.language = newLanguage;
    await saveConfig();
  } catch (error) {
    console.error('Failed to update language:', error);
    // Revert if save failed
    const previousLanguage = config.value.language;
    locale.value = previousLanguage;
  }
}

// ===========================
// ADDON CONFIG
// ===========================

function openAddonConfig(addon: AddonStatus) {
  selectedAddon.value = addon;
  showAddonConfigModal.value = true;
}

async function onConfigSaved() {
  // Reload addon definitions to get updated config
  await loadAddonDefinitions();
  // Reload addons to update the list with new config
  await loadAddons();
}

// ===========================
// APP AUTO-UPDATE
// ===========================

async function checkForAppUpdate() {
  try {
    const update = await checkUpdate();

    if (update?.available) {
      console.log(`App update available: ${update.version}`);
      appNewVersion.value = update.version;
      pendingUpdate = update;
      showUpdateDialog.value = true;
    } else {
      console.log('App is up to date');
    }
  } catch (error) {
    console.error('Failed to check for app updates:', error);
  }
}

async function handleAppUpdate() {
  if (!pendingUpdate) return;

  try {
    console.log('Downloading and installing update...');
    await pendingUpdate.downloadAndInstall();

    console.log('Update installed, relaunching...');
    await relaunch();
  } catch (error) {
    console.error('Failed to install app update:', error);
  }
}

function handleUpdateRemindLater() {
  showUpdateDialog.value = false;
  pendingUpdate = null;
}
</script>

<template>
  <div class="w-screen h-screen flex flex-col bg-background text-foreground overflow-hidden">
    <!-- Custom TitleBar -->
    <TitleBar />

    <!-- Main Content -->
    <div class="flex-1 overflow-y-auto px-4 py-4 space-y-4">
      <!-- WoW Status -->
      <WowStatus
        :wow-path="config.wow_path"
        :is-scanning="isScanning"
        @open-settings="showSettingsModal = true"
      />

      <!-- Addons List -->
      <div v-if="hasWowPath" class="space-y-2">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-lg font-semibold text-foreground">{{ t('addons.title') }}</h2>
          <div class="flex items-center gap-2 text-xs text-muted-foreground">
            <span>{{ installedCount }} / {{ addonDefinitions.length }} {{ t('addons.installed') }}</span>
            <span v-if="updateAvailableCount > 0" class="text-[--status-idle]">
              • {{ updateAvailableCount }} {{ t('addons.updates') }}
            </span>
          </div>
        </div>

        <AddonListItem
          v-for="addon in addons"
          :key="addon.definition.local_name"
          :addon="addon"
          @install="installAddon(addon)"
          @update="updateAddon(addon)"
          @check="checkSingleAddon(addon)"
          @configure="openAddonConfig(addon)"
        />
      </div>
    </div>

    <!-- Settings Modal -->
    <SettingsModal
      v-model:open="showSettingsModal"
      :wow-path="config.wow_path"
      :auto-scan-results="autoScanResults"
      :is-scanning="isScanning"
      :launch-on-startup="config.launch_on_startup"
      :minimize-on-startup="config.minimize_on_startup"
      :language="config.language"
      @select-path="selectWowPath"
      @validate-manual-path="validateManualPath"
      @auto-scan="autoScanWowPath"
      @update:launchOnStartup="updateLaunchOnStartup"
      @update:minimizeOnStartup="updateMinimizeOnStartup"
      @update:language="updateLanguage"
    />

    <!-- Addon Config Modal -->
    <AddonConfigModal
      v-model:open="showAddonConfigModal"
      :addon="selectedAddon"
      @config-saved="onConfigSaved"
    />

    <!-- App Update Dialog -->
    <UpdateDialog
      v-model:open="showUpdateDialog"
      :current-version="appCurrentVersion"
      :new-version="appNewVersion"
      @download-and-install="handleAppUpdate"
      @remind-later="handleUpdateRemindLater"
    />
  </div>
</template>
