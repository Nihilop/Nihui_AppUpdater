# Gestion de Version - Nihui Addon Updater

## ✨ Version Centralisée

La version de l'application est maintenant **centralisée** ! Tu n'as qu'à la changer à **UN SEUL ENDROIT**.

### 📍 Où changer la version ?

**Fichier unique : `src-tauri/Cargo.toml`**

```toml
[package]
name = "nihui_app"
version = "1.0.0"  <-- Change ici uniquement !
```

### 🔄 Comment ça fonctionne ?

1. **Backend** : Le backend Rust lit automatiquement la version depuis `Cargo.toml` via `env!("CARGO_PKG_VERSION")`
2. **Frontend** : Au démarrage, le frontend appelle `TauriAPI.getAppVersion()` qui demande la version au backend
3. **Auto-updater** : Utilise cette version pour comparer avec les releases GitHub

### 🚀 Workflow de montée de version

#### Étape 1 : Change la version
```bash
# Édite src-tauri/Cargo.toml
version = "1.1.0"  # Nouvelle version
```

#### Étape 2 : Commit et push
```bash
git add .
git commit -m "Bump version to v1.1.0"
git push origin main
```

#### Étape 3 : Créer un tag
```bash
git tag v1.1.0
git push origin v1.1.0
```

#### Étape 4 : GitHub Actions fait le reste !
- Build automatique
- Signature
- Création de la release
- Upload des fichiers

### 🎯 Avec GitHub Desktop

1. **Change la version** dans `src-tauri/Cargo.toml`
2. **Commit les changements** dans GitHub Desktop
3. **Push vers GitHub**
4. **Créer le tag** :
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

### ⚠️ Note importante sur tauri.conf.json

Le fichier `tauri.conf.json` contient aussi une version, mais elle **n'est plus utilisée** pour l'auto-updater. Tu peux la laisser synchronisée manuellement si tu veux, mais ce n'est pas obligatoire. La source de vérité est `Cargo.toml`.

Si tu veux synchroniser les deux, tu peux faire :

```json
// src-tauri/tauri.conf.json
{
  "version": "1.1.0"  <-- Optionnel, peut rester différent
}
```

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
