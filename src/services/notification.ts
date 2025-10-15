import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';

/**
 * Notification Service
 * Handles system notifications for addon updates
 *
 * Note: Click handlers on Windows notifications are not supported in Tauri v2.
 * Users should click on the tray icon (which turns orange) to open the app.
 */

export class NotificationService {
  private static permissionGranted = false;
  private static lastNotificationTimestamp = 0;
  private static notificationCooldown = 60 * 1000; // 1 minute cooldown between notifications
  private static notifiedAddons: Set<string> = new Set(); // Track addons notified in this session

  /**
   * Initialize notification service
   */
  static async initialize(): Promise<void> {
    // Request permission
    await this.requestPermission();
  }

  /**
   * Clear the notification history (useful for testing or manual refresh)
   */
  static clearNotificationHistory(): void {
    this.notifiedAddons.clear();
  }

  /**
   * Remove an addon from the notification history
   * Called when an addon is updated, so user can be notified again if a new update comes
   */
  static markAddonAsUpdated(addonName: string): void {
    if (this.notifiedAddons.has(addonName)) {
      this.notifiedAddons.delete(addonName);
    }
  }

  /**
   * Request notification permission from the user
   */
  static async requestPermission(): Promise<boolean> {
    let permission = await isPermissionGranted();

    if (!permission) {
      const result = await requestPermission();
      permission = result === 'granted';
    }

    this.permissionGranted = permission;
    return permission;
  }

  /**
   * Send a notification for available updates
   * @param _updateCount Number of addons with available updates
   * @param addonNames List of addon names with updates
   */
  static async sendUpdateNotification(_updateCount: number, addonNames: string[]): Promise<void> {
    if (!this.permissionGranted) {
      const granted = await this.requestPermission();
      if (!granted) {
        console.warn('Notification permission not granted');
        return;
      }
    }

    // Filter out addons that have already been notified in this session
    const newAddons = addonNames.filter(name => !this.notifiedAddons.has(name));

    if (newAddons.length === 0) {
      return;
    }

    // Check cooldown to avoid spamming notifications
    const now = Date.now();
    if (now - this.lastNotificationTimestamp < this.notificationCooldown) {
      return;
    }
    this.lastNotificationTimestamp = now;

    const addonList = newAddons.join(', ');
    const body = newAddons.length === 1
      ? `${addonList} has an update available. Click the tray icon to view.`
      : `${newAddons.length} addons have updates available: ${addonList}. Click the tray icon to view.`;

    await sendNotification({
      title: '🔔 Nihui Addons Update Available',
      body,
    });

    // Mark these addons as notified for this session
    newAddons.forEach(name => this.notifiedAddons.add(name));
  }

  /**
   * Send a notification after successful silent update
   * @param updateCount Number of addons that were updated
   */
  static async sendUpdateSuccessNotification(updateCount: number): Promise<void> {
    if (!this.permissionGranted) {
      const granted = await this.requestPermission();
      if (!granted) {
        console.warn('Notification permission not granted');
        return;
      }
    }

    const body = updateCount === 1
      ? '1 addon has been updated successfully!'
      : `${updateCount} addons have been updated successfully!`;

    await sendNotification({
      title: '✅ Update Complete',
      body,
    });
  }

  /**
   * Test notification to verify system is working
   */
  static async sendTestNotification(): Promise<void> {
    if (!this.permissionGranted) {
      const granted = await this.requestPermission();
      if (!granted) {
        console.warn('Notification permission not granted');
        return;
      }
    }

    await sendNotification({
      title: '✅ Notifications Working',
      body: 'You will receive notifications when addon updates are available',
    });
  }
}
