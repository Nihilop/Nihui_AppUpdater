const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const ICONS_DIR = path.join(__dirname, 'icons');
const BACKUP_DIR = path.join(ICONS_DIR, 'backup');
const TRAY_DIR = path.join(ICONS_DIR, 'tray');

// Icon configurations
const APP_ICON_SIZES = {
  'icon.png': 512,
  '128x128.png': 128,
  '128x128@2x.png': 256,
  'Square30x30Logo.png': 30,
  'Square44x44Logo.png': 44,
  'Square71x71Logo.png': 71,
  'Square89x89Logo.png': 89,
  'Square107x107Logo.png': 107,
  'Square142x142Logo.png': 142,
  'Square150x150Logo.png': 150,
  'Square284x284Logo.png': 284,
  'Square310x310Logo.png': 310,
  'StoreLogo.png': 50,
};

const TRAY_ICONS = [
  'icon_tray-normal.png',
  'icon_error.png',
  'icon_warning.png',
  'icon_success.png',
];

async function main() {
  console.log('🎨 Generating icons from source files...\n');

  // Create backup directory
  if (!fs.existsSync(BACKUP_DIR)) {
    fs.mkdirSync(BACKUP_DIR, { recursive: true });
    console.log('✅ Created backup directory');
  }

  // Create tray directory
  if (!fs.existsSync(TRAY_DIR)) {
    fs.mkdirSync(TRAY_DIR, { recursive: true });
    console.log('✅ Created tray directory\n');
  }

  // Backup existing icons (only if not already backed up)
  console.log('📦 Backing up existing icons...');
  const filesToBackup = ['icon.png', '128x128.png', '128x128@2x.png', '32x32.png', 'icon.ico', 'icon.icns'];
  for (const file of filesToBackup) {
    const source = path.join(ICONS_DIR, file);
    const dest = path.join(BACKUP_DIR, file);
    if (fs.existsSync(source) && !fs.existsSync(dest)) {
      fs.copyFileSync(source, dest);
      console.log(`  ✓ Backed up ${file}`);
    }
  }

  console.log('\n📐 Generating app icons from base_icon.png...');

  // Generate all app icon sizes from base_icon.png
  const baseIconPath = path.join(ICONS_DIR, 'base_icon.png');
  if (!fs.existsSync(baseIconPath)) {
    console.error('❌ Error: base_icon.png not found!');
    process.exit(1);
  }

  for (const [filename, size] of Object.entries(APP_ICON_SIZES)) {
    const outputPath = path.join(ICONS_DIR, filename);
    await sharp(baseIconPath)
      .resize(size, size, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
      .png()
      .toFile(outputPath);
    console.log(`  ✓ Generated ${filename} (${size}x${size})`);
  }

  // Generate 32x32.png from icon_tray-normal.png (default tray icon)
  console.log('\n🖼️  Generating default tray icon (32x32.png)...');
  const trayNormalPath = path.join(ICONS_DIR, 'icon_tray-normal.png');
  if (fs.existsSync(trayNormalPath)) {
    await sharp(trayNormalPath)
      .resize(32, 32, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
      .png()
      .toFile(path.join(ICONS_DIR, '32x32.png'));
    console.log('  ✓ Generated 32x32.png from icon_tray-normal.png');
  }

  // Generate tray icons (32x32 for Windows system tray)
  console.log('\n🎯 Generating tray icons (32x32)...');
  for (const trayIcon of TRAY_ICONS) {
    const sourcePath = path.join(ICONS_DIR, trayIcon);
    if (!fs.existsSync(sourcePath)) {
      console.warn(`  ⚠️  Warning: ${trayIcon} not found, skipping...`);
      continue;
    }

    // Remove icon_ prefix and keep the rest
    const outputName = trayIcon.replace('icon_', '');
    const outputPath = path.join(TRAY_DIR, outputName);

    await sharp(sourcePath)
      .resize(32, 32, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
      .png()
      .toFile(outputPath);
    console.log(`  ✓ Generated tray/${outputName} (32x32)`);
  }

  // Generate multi-size .ico file for Windows
  console.log('\n🪟 Generating icon.ico (multi-size)...');
  try {
    // Generate multiple sizes for .ico
    const icoSizes = [16, 24, 32, 48, 64, 128, 256];
    const icoBuffers = [];

    for (const size of icoSizes) {
      const buffer = await sharp(baseIconPath)
        .resize(size, size, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
        .png()
        .toBuffer();
      icoBuffers.push(buffer);
    }

    // Note: Sharp doesn't support .ico creation directly
    // We'll need to use a different approach or tool
    console.log('  ⚠️  Note: .ico file generation requires additional tools');
    console.log('  ℹ️  You can use an online converter or ImageMagick:');
    console.log('     https://convertio.co/png-ico/');
    console.log('     Or: magick convert base_icon.png -define icon:auto-resize=256,128,64,48,32,16 icon.ico');
  } catch (error) {
    console.error('  ❌ Error generating .ico:', error.message);
  }

  // Generate .icns for macOS
  console.log('\n🍎 Generating icon.icns...');
  console.log('  ⚠️  Note: .icns file generation requires macOS tools or png2icons');
  console.log('  ℹ️  You can use: npm install -g png2icons');
  console.log('     Then: png2icons base_icon.png icon.icns');

  console.log('\n✅ Icon generation complete!');
  console.log('\n📝 Summary:');
  console.log('  • App icons generated from base_icon.png');
  console.log('  • Tray icons (32x32) generated in tray/ folder');
  console.log('  • Backup saved in icons/backup/');
  console.log('\n⚠️  Manual steps needed:');
  console.log('  1. Generate icon.ico using an online converter or ImageMagick');
  console.log('  2. Generate icon.icns using png2icons (if targeting macOS)');
}

main().catch(console.error);
