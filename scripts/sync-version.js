import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, '..');

// Read version from version.json
const versionJsonPath = join(rootDir, 'version.json');
const versionData = JSON.parse(readFileSync(versionJsonPath, 'utf8'));
const version = versionData.version;

console.log(`Syncing version: ${version}`);

// Update Cargo.toml
const cargoTomlPath = join(rootDir, 'src-tauri', 'Cargo.toml');
let cargoToml = readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${version}"`);
writeFileSync(cargoTomlPath, cargoToml);
console.log(`✓ Updated ${cargoTomlPath}`);

// Update tauri.conf.json
const tauriConfigPath = join(rootDir, 'src-tauri', 'tauri.conf.json');
const tauriConfig = JSON.parse(readFileSync(tauriConfigPath, 'utf8'));
tauriConfig.version = version;
writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2) + '\n');
console.log(`✓ Updated ${tauriConfigPath}`);

console.log('\n✅ Version synchronized successfully!');
console.log(`\nNext steps:`);
console.log(`1. Review the changes`);
console.log(`2. Commit: git add . && git commit -m "chore: bump version to ${version}"`);
console.log(`3. Tag: git tag v${version}`);
console.log(`4. Push: git push origin main --tags`);
