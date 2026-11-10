# Changelog

## Version 2.0 - In-App Blender Configuration

### 🎉 Major Features

#### In-App Settings Panel
- ✅ Configure Blender path directly in the app
- ✅ Browse for `blender.exe` with file picker
- ✅ Real-time path validation
- ✅ Save/load configuration automatically
- ✅ Visual status indicators (✓ / ⚠ / ✗)

#### Enhanced User Experience
- ✅ No need to edit config files manually
- ✅ No environment variables required
- ✅ Configuration persists across sessions
- ✅ Auto-fallback to monorepo-3d configs
- ✅ Clear setup instructions in-app

#### Export Features
- ✅ Export single .blend file to GLB
- ✅ Batch export all files
- ✅ Progress indicator during export
- ✅ Success/error messages with details
- ✅ Integrates with monorepo-3d pipeline

### 🔧 Technical Changes

#### Configuration System
- Added `blender-manager-config.json` (per-user config)
- Config file created in app directory
- Auto-detection cascade:
  1. Local config (app-generated)
  2. Monorepo configs (blender.local.json, blender.json)
  3. Environment variable (BLENDER_PATH)

#### UI Components
- Settings panel with browse/save workflow
- Status display in main screen
- "Configure →" quick link when not configured
- Path validation with visual feedback
- Settings accessible via ⚙️ button

#### Code Structure
```rust
BlenderApp::new()              // Load saved config on startup
BlenderApp::browse_blender_exe() // File picker for blender.exe
BlenderApp::save_blender_path()  // Persist config to JSON
BlenderApp::render_settings()    // Settings UI panel
```

### 📁 New Files

- `blender-manager-config.json` - Auto-generated user config
- `QUICKSTART.md` - Quick reference guide
- `UI_GUIDE.md` - Visual interface documentation
- `setup.ps1` - PowerShell auto-setup script (optional)

### 📝 Updated Files

- `src/main.rs` - Added settings panel and config management
- `Cargo.toml` - Added serde/serde_json for JSON config
- `README.md` - Updated with in-app configuration workflow
- `BLENDER_SETUP.md` - Rewritten for in-app setup flow

### 🗑️ Removed

- `test-blender-config.bat` - No longer needed (in-app now)

---

## Version 1.0 - Initial Release

### Features
- Browse folders for .blend files
- Display file list with paths and sizes
- Export to GLB via Blender CLI
- Batch export support
- Monorepo-3d integration

### Configuration
- Required manual JSON config or environment variable
- No in-app configuration

---

## Migration Guide: v1.0 → v2.0

### For End Users

**Old workflow:**
1. Edit `blender.local.json` manually
2. Or set `BLENDER_PATH` environment variable
3. Restart system/terminal

**New workflow:**
1. Launch app
2. Click ⚙️ Settings
3. Browse for blender.exe
4. Save
5. Done!

**Your old configs still work!** The app will auto-detect them if you had them set up.

### For Developers

**Breaking Changes:**
- None! Old config methods still work as fallback.

**New Dependencies:**
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**New API:**
```rust
BlenderApp::new()                   // Initializes with saved config
BlenderApp::save_blender_path(path) // Saves config
BlenderApp::load_blender_path(file) // Loads from file
```

---

## Roadmap / Future Ideas

### Possible v3.0 Features
- [ ] Drag & drop .blend files onto window
- [ ] Custom export settings (FPS, scale, flags)
- [ ] Preview .blend thumbnails
- [ ] Export queue with pause/resume
- [ ] Watch folder for auto-export
- [ ] Export profiles (game/web/AR different settings)
- [ ] Integration with Blender add-ons
- [ ] Multi-language UI
- [ ] Dark/light theme toggle
- [ ] Recent folders list
- [ ] Export history log

### Community Requests
Open an issue if you'd like to see a feature!

---

## Known Issues / Limitations

### Current Limitations
- Export is synchronous (UI freezes during batch export)
- No progress bar for individual file export
- No .blend file preview/thumbnail
- Export script path is hardcoded to monorepo-3d location
- Windows-only (currently)

### Workarounds
- **UI freeze during export**: This is expected; wait for completion message
- **Script not found**: Clone monorepo-3d or modify script path in code
- **Large batch exports**: Export in smaller batches if needed

### Planned Fixes
- Async export with real progress tracking
- Configurable export script path
- Cross-platform support (Linux, macOS)

---

## Credits

**Built with:**
- Rust programming language
- egui/eframe (GUI framework)
- rfd (file dialogs)
- walkdir (directory traversal)
- serde/serde_json (config serialization)

**Integrates with:**
- Blender CLI (export engine)
- monorepo-3d Blender pipeline

**Inspired by:**
- monorepo-3d Node.js pipeline
- Need for standalone Blender file management tool

---

## Version History Summary

| Version | Date | Key Feature |
|---------|------|-------------|
| 2.0 | 2024-11 | In-app Blender configuration |
| 1.0 | 2024-11 | Initial release: Browse & export |

---

## Feedback & Contributions

**Found a bug?** Open an issue!
**Have a feature request?** Let us know!
**Want to contribute?** Pull requests welcome!

This is an active project - feedback shapes the roadmap.
