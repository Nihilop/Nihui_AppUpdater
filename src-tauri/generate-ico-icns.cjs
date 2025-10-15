const png2icons = require('png2icons');
const fs = require('fs');
const path = require('path');

const ICONS_DIR = path.join(__dirname, 'icons');

async function main() {
  console.log('🪟 Generating icon.ico and icon.icns...\n');

  const baseIconPath = path.join(ICONS_DIR, 'base_icon.png');
  const icoOutputPath = path.join(ICONS_DIR, 'icon.ico');
  const icnsOutputPath = path.join(ICONS_DIR, 'icon.icns');

  if (!fs.existsSync(baseIconPath)) {
    console.error('❌ Error: base_icon.png not found!');
    process.exit(1);
  }

  // Read base icon
  const input = fs.readFileSync(baseIconPath);

  // Generate .ico (Windows)
  console.log('🪟 Generating icon.ico...');
  try {
    const icoBuffer = await png2icons.createICO(input, png2icons.BILINEAR, 0, false);
    fs.writeFileSync(icoOutputPath, icoBuffer);
    console.log('  ✅ Generated icon.ico');
  } catch (error) {
    console.error('  ❌ Error generating .ico:', error.message);
  }

  // Generate .icns (macOS)
  console.log('\n🍎 Generating icon.icns...');
  try {
    const icnsBuffer = await png2icons.createICNS(input, png2icons.BILINEAR, 0);
    fs.writeFileSync(icnsOutputPath, icnsBuffer);
    console.log('  ✅ Generated icon.icns');
  } catch (error) {
    console.error('  ❌ Error generating .icns:', error.message);
  }

  console.log('\n✅ All icon files generated successfully!');
}

main().catch(console.error);
