# Gestion de Version - Nihui Addon Updater

## ✨ Version Centralisée avec version.json

La version de l'application est maintenant **ultra-centralisée** ! Un seul fichier à éditer, un script qui fait tout le reste.

### 📍 Où changer la version ?

**Fichier unique : `version.json` à la racine**

```json
{
  "version": "1.0.0"
}
```

### 🔄 Comment ça fonctionne ?

1. **version.json** : Source de vérité unique pour le numéro de version
2. **Script de sync** : Met à jour automatiquement tous les fichiers nécessaires
   - `src-tauri/Cargo.toml` - Version du package Rust
   - `src-tauri/tauri.conf.json` - Version de configuration Tauri
3. **Backend** : Lit la version depuis `Cargo.toml` via `env!("CARGO_PKG_VERSION")`
4. **Frontend** : Appelle `TauriAPI.getAppVersion()` pour afficher la version
5. **Auto-updater** : Compare avec les releases GitHub

### 🚀 Workflow de montée de version

#### Étape 1 : Change la version
```json
// Édite version.json
{
  "version": "1.1.0"
}
```

#### Étape 2 : Synchronise automatiquement
```bash
pnpm version:sync
```

Le script va :
- ✓ Mettre à jour `src-tauri/Cargo.toml`
- ✓ Mettre à jour `src-tauri/tauri.conf.json`
- ✓ T'afficher les commandes git à exécuter

#### Étape 3 : Commit et tag (commandes affichées par le script)
```bash
git add .
git commit -m "chore: bump version to 1.1.0"
git tag v1.1.0
git push origin main --tags
```

#### Étape 4 : GitHub Actions fait le reste !
- Build automatique
- Signature
- Création de la release
- Upload des fichiers (.msi, .nsis, latest.json)

### 🎯 Avec GitHub Desktop

1. **Change la version** dans `version.json`
2. **Lance le script** : `pnpm version:sync`
3. **Commit les changements** dans GitHub Desktop
4. **Push vers GitHub**
5. **Créer le tag** :
   - Dans GitHub Desktop : Menu `Repository` → `Create Tag...`
   - Nom du tag : `v1.1.0` (avec le `v` !)
   - Clique sur "Create Tag"
   - Push le tag : Menu `Repository` → `Push origin` (ou `Ctrl+P`)

### 📦 Build local

Si tu veux build localement sans passer par GitHub Actions :

```bash
# Build simple (sans signature)
pnpm tauri build

# Build avec signature (pour que l'auto-updater fonctionne)
$env:TAURI_SIGNING_PRIVATE_KEY = (Get-Content src-tauri\tauri.keypai -Raw)
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = "ton_mot_de_passe"
pnpm tauri build
```

### ⚠️ Note importante sur la synchronisation

Le script `pnpm version:sync` met automatiquement à jour :
- `src-tauri/Cargo.toml` → Source de vérité pour le backend
- `src-tauri/tauri.conf.json` → Utilisé pour le bundle et les releases

**Plus besoin de toucher ces fichiers manuellement !** Le script s'occupe de tout.

### 🐛 Dépannage

#### La version affichée est incorrecte
- Vérifie que tu as bien changé `src-tauri/Cargo.toml`
- Rebuild l'app : `pnpm tauri build`
- La version est chargée au build, pas à l'exécution

#### L'updater ne trouve pas la nouvelle version
- Vérifie que tu as bien push le **tag** (pas juste le commit)
- Vérifie que GitHub Actions a bien créé la release
- Vérifie que `latest.json` est présent dans les assets de la release

### 📊 Vérifier la version courante

Tu peux vérifier la version compilée dans l'app :
- Ouvre la console développeur (`F12`)
- La version est chargée au démarrage et affichée dans les logs
- Ou regarde dans la dialog d'update quand une mise à jour est disponible
