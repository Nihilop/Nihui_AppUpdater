# Nihui Addon Updater - Dev Notes

## 🏗️ Architecture

### Backend (Rust/Tauri)
- **Auto-scan WoW path** : Scanne automatiquement les chemins Battle.net courants
- **Validation path** : Vérifie que le chemin contient `_retail_/Interface/AddOns`
- **Lecture .toc** : Parse les fichiers `.toc` pour extraire `## Version:`
- **Config management** : Sauvegarde/charge config dans `%APPDATA%/nihui_app/config.json`
- **GitHub API** : Fetch releases ou branches avec comparaison de versions

### Frontend (Vue.js + Tailwind)
- **Settings** : Config chemin WoW (auto-scan + manuel)
- **Update Checker** : Scan GitHub et comparaison versions
- **Addon List** : Affichage status avec icônes
- **Update Mode Toggle** : Switch entre Release/Branch

## 📝 TODO Avant Premier Test

### 1. Configuration GitHub (URGENT)
Dans `src/App.vue`, lignes 15-16, remplace :
```typescript
github_owner: 'YOUR_GITHUB_USERNAME', // TODO: Replace
github_repo: 'Nihui-Addons', // TODO: Replace
```

Par tes vraies infos GitHub :
```typescript
github_owner: 'ton-username-github',
github_repo: 'nom-de-ton-repo',
```

### 2. Ajouter ## Version: aux .toc
Le backend lit la version depuis les fichiers `.toc`. Assure-toi que **TOUS** tes addons ont cette ligne :

```lua
## Interface: 110002
## Title: Nihui_iv
## Version: 0.2.0  <-- IMPORTANT!
## SavedVariables: NihuiIVDB
```

### 3. Build Rust
Cargo va télécharger les dépendances au premier build :
```bash
cd Nihui_App
pnpm install  # Si pas déjà fait
pnpm tauri dev
```

## 🔧 Fonctionnalités Implémentées

✅ **Auto-scan WoW path**
- Scan `C:\`, `D:\`, `E:\`, `F:\` pour Battle.net installs
- Essaie de lire `%PROGRAMDATA%\Battle.net\Battle.net.config`
- Fallback sur input manuel si rien trouvé

✅ **Validation path**
- Vérifie `_retail_/Interface/AddOns` existe
- Sauvegarde jusqu'à `World of Warcraft\` (pas juste AddOns)

✅ **Lecture versions locales**
- Scan tous les addons `Nihui_*` dans AddOns/
- Parse `## Version:` depuis les .toc
- Gère les addons sans version (marque "unknown")

✅ **GitHub API - Release Mode**
- Fetch latest release via `/repos/{owner}/{repo}/releases/latest`
- Compare `tag_name` (en enlevant le "v") avec version locale
- Affiche "update available" si différent

✅ **GitHub API - Branch Mode**
- Fetch latest commit SHA via `/repos/{owner}/{repo}/commits/{branch}`
- Compare avec version locale (si format `commit:abc1234`)
- Utile pour dev (track les commits)

✅ **UI complète**
- Dark theme avec Tailwind
- Status indicators avec emojis
- Toggle Release/Branch
- Summary cards (Total / Updates / Status)
- Tray icon avec tooltip

## 🚧 Features à Implémenter Plus Tard

❌ **Download & Install**
Le bouton "Update" est disabled pour l'instant. Pour l'implémenter :

1. **Backend Rust** - Ajoute command :
```rust
#[tauri::command]
async fn download_and_install_addon(
    addon_name: String,
    zipball_url: String,
    wow_path: String
) -> Result<(), String> {
    // 1. Download zip depuis GitHub
    // 2. Extract dans temp folder
    // 3. Backup ancien addon
    // 4. Copy nouveau addon dans AddOns/
    // 5. Update .toc version
    Ok(())
}
```

2. **Frontend** - Wire le bouton Update

❌ **Multi-repo support**
Pour l'instant un seul repo GitHub. Si tu veux séparer les addons :
- Modifier `AppConfig` pour accepter un `Vec<RepoConfig>`
- Chaque addon peut pointer vers son propre repo

❌ **Rollback system**
Garder les backups des versions précédentes et permettre rollback

❌ **Notifications**
Notifier l'user via le tray icon quand une update est dispo

## 📦 Structure du Projet

```
Nihui_App/
├── src/                    # Frontend Vue.js
│   ├── App.vue            # UI complète (Settings + Update Checker + List)
│   ├── types.ts           # Types TypeScript
│   └── services/
│       └── tauri.ts       # Wrapper pour Tauri commands
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── lib.rs         # Toutes les Tauri commands
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Dépendances Rust
│   └── tauri.conf.json    # Config Tauri
└── package.json           # Dépendances npm
```

## 🧪 Testing

### Test 1 : Auto-scan WoW
1. Lance l'app : `pnpm tauri dev`
2. Au démarrage, l'app devrait scanner automatiquement
3. Si WoW trouvé → affiche le chemin
4. Si pas trouvé → propose input manuel

### Test 2 : Lecture addons
1. Configure le chemin WoW
2. L'app devrait lister tous les `Nihui_*` avec versions

### Test 3 : Check updates (Release mode)
1. Clique "Check for Updates"
2. L'app fetch GitHub releases
3. Compare avec versions locales
4. Affiche combien d'updates disponibles

### Test 4 : Check updates (Branch mode)
1. Toggle vers Branch mode
2. Clique "Check for Updates"
3. L'app fetch le dernier commit SHA
4. Compare avec versions locales

## ⚠️ Notes Importantes

1. **CORS** : GitHub API public, pas besoin de token pour les requêtes simples, mais rate limit à 60/h. Si besoin, ajoute un token dans les headers.

2. **Config Storage** : La config est dans :
    - Windows : `%APPDATA%\nihui_app\config.json`
    - Editable manuellement si besoin

3. **Battle.net Config** : Le scan essaie de lire `%PROGRAMDATA%\Battle.net\Battle.net.config`, mais la regex peut ne pas tout capturer. Si auto-scan ne trouve pas, c'est normal, l'user peut entrer manuellement.

4. **TOC Version Format** : Le regex cherche `## Version: X.Y.Z`, case-insensitive. Fonctionne avec ou sans espaces.

## 🎯 Prochaines Étapes

1. **Remplace les TODO dans App.vue** (github_owner, github_repo)
2. **Ajoute ## Version: dans tous les .toc**
3. **Teste l'auto-scan**
4. **Teste la lecture des versions locales**
5. **Push tes addons sur GitHub**
6. **Crée une release GitHub (tag v0.2.0)**
7. **Teste le check updates**

## 🐛 Debug

Si quelque chose ne marche pas :

1. **Console Rust** : Les erreurs backend s'affichent dans le terminal où tu as lancé `pnpm tauri dev`
2. **Console Browser** : Ouvre DevTools (F12) pour voir les logs frontend
3. **Config file** : Vérifie `%APPDATA%\nihui_app\config.json`

Bon test ! 🚀
