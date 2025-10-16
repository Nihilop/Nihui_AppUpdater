# Guide de Release - Nihui Addon Updater

## 📦 Tester localement

### 1. Installer le package process
```bash
pnpm add @tauri-apps/plugin-process
```

### 2. Build et installer la version actuelle (v0.1.0)
```bash
pnpm tauri build
```

L'installeur sera dans : `src-tauri/target/release/bundle/nsis/` ou `msi/`

### 3. Installer l'app sur ton PC
- Double-clique sur le fichier `.exe` ou `.msi`
- L'app va s'installer

### 4. Tester l'auto-updater

Pour tester, tu dois créer une version plus récente (ex: v0.2.0) :

1. **Modifier la version dans `src-tauri/tauri.conf.json`** :
```json
{
  "version": "0.2.0",
  ...
}
```

2. **Modifier la version dans `src-tauri/Cargo.toml`** :
```toml
[package]
version = "0.2.0"
```

3. **Build la nouvelle version avec signature** :
```powershell
# Charger la clé privée
$env:TAURI_SIGNING_PRIVATE_KEY = (Get-Content src-tauri\tauri.keypai -Raw)
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = "TON_MOT_DE_PASSE"

# Build
pnpm tauri build
```

4. **Créer une release GitHub** :
```bash
git add .
git commit -m "Release v0.2.0"
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

5. **Upload sur GitHub** :
- Va sur https://github.com/Nihilop/Nihui_AppUpdater/releases
- GitHub Actions va automatiquement créer la release et uploader les fichiers
- Vérifie que `latest.json` est bien présent dans les assets

6. **Tester** :
- Lance la version v0.1.0 installée
- Elle devrait détecter la v0.2.0 et proposer de l'installer !

---

## 🚀 Workflow de Release Automatique

### Configuration des secrets GitHub

1. Va sur ton repo GitHub : https://github.com/Nihilop/Nihui_AppUpdater/settings/secrets/actions

2. Ajoute 2 secrets :

**`TAURI_SIGNING_PRIVATE_KEY`** :
```bash
# Sur Windows PowerShell :
Get-Content src-tauri\tauri.keypai -Raw | Set-Clipboard
```
Colle le contenu dans le secret GitHub.

**`TAURI_SIGNING_PRIVATE_KEY_PASSWORD`** :
Le mot de passe que tu as entré lors de la génération de la clé.

### Processus de release

1. **Prépare ta version** :
```bash
# Modifie la version dans src-tauri/tauri.conf.json et Cargo.toml
# Commit tes changements
git add .
git commit -m "Bump version to v0.2.0"
git push origin main
```

2. **Créer un tag** :
```bash
git tag v0.2.0
git push origin v0.2.0
```

3. **GitHub Actions fait le reste** :
- Build l'app avec signature
- Crée la release GitHub
- Upload les fichiers (`.msi`, `.exe`, `latest.json`)

4. **L'auto-updater fonctionne** :
- Les utilisateurs avec la version précédente recevront une notification
- Ils peuvent installer la mise à jour en un clic

---

## 🔐 Sécurité

### Clé privée

**IMPORTANT** : La clé privée `src-tauri/tauri.keypai` est déjà dans `.gitignore`.

**Sauvegarde-la** dans un endroit sûr :
- Cloud privé (Google Drive, OneDrive)
- Clé USB
- Gestionnaire de mots de passe

**Si tu la perds** : Tu ne pourras plus signer tes futures versions !

### Rotation des clés (si compromises)

Si ta clé est compromise :

1. Génère une nouvelle paire de clés :
```bash
pnpm tauri signer generate -w src-tauri/tauri.keypair.new
```

2. Remplace la clé publique dans `tauri.conf.json`

3. Update les secrets GitHub

4. **Problème** : Les anciennes versions ne pourront plus se mettre à jour automatiquement (elles utilisent l'ancienne clé publique).

---

## 📊 Vérifier les releases

Tu peux voir toutes tes releases ici :
https://github.com/Nihilop/Nihui_AppUpdater/releases

Le fichier `latest.json` devrait ressembler à :
```json
{
  "version": "0.2.0",
  "pub_date": "2025-01-15T10:30:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "...",
      "url": "https://github.com/.../nihui_app_0.2.0_x64_en-US.msi"
    }
  }
}
```

---

## 🐛 Dépannage

### L'updater ne trouve pas de mise à jour

1. Vérifie que `latest.json` est présent dans les assets de la release
2. Vérifie l'URL dans `tauri.conf.json` (doit pointer vers GitHub)
3. Vérifie la console du navigateur pour les erreurs

### Erreur de signature

1. Vérifie que la clé publique dans `tauri.conf.json` est correcte
2. Vérifie que tu as bien défini les secrets GitHub
3. Vérifie que le mot de passe est correct

### GitHub Actions échoue

1. Vérifie les logs : https://github.com/Nihilop/Nihui_AppUpdater/actions
2. Vérifie que les secrets sont bien configurés
3. Vérifie que le tag commence bien par `v` (ex: `v1.0.0`)
