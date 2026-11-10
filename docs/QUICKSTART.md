# Blender File Manager & Exporter - Quick Reference

## 🎯 What This App Does

- Browse folders to find all Blender (.blend) files
- Display file paths and sizes in a clean table
- Export .blend files to GLB format (game-ready 3D models)
- Batch export multiple files at once
- Integrates with monorepo-3d Blender pipeline

## 🚀 Getting Started (3 Steps)

### Step 1: Build & Launch
```cmd
# Double-click this file:
build.bat
```

### Step 2: Configure Blender (First Time Only)
1. Click "⚙️ Settings" (top-right)
2. Click "📁 Browse..."
3. Select your `blender.exe` (usually in `C:\Program Files\Blender Foundation\`)
4. Click "💾 Save Path"
5. Click "← Back"

### Step 3: Use It
1. Click "📁 Select Folder"
2. Choose a folder with .blend files
3. Click a file to select it, then "📤 Export Selected"
4. Or click "🚀 Export All to GLB" to export everything

**Done!** Your .glb files are created next to the .blend files.

## 📁 Files You Need

| File | What It Does |
|------|--------------|
| `build.bat` | Build and run the app (recommended) |
| `run.bat` | Run development mode |
| `run-release.bat` | Run optimized version |
| `README.md` | Full documentation |
| `BLENDER_SETUP.md` | Detailed Blender configuration guide |
| `UI_GUIDE.md` | Visual UI reference |

## ⚙️ Configuration

Your Blender path is saved in:
```
blender-manager-config.json
```
(Created automatically when you save settings)

## 🎨 Main Features

### Browse Mode
- ✅ Scan folders recursively
- ✅ Display all .blend files with sizes
- ✅ Click to select files
- ✅ No Blender required for browsing

### Export Mode (Requires Blender)
- ✅ Export single file to GLB
- ✅ Batch export all files
- ✅ Show progress and results
- ✅ Handles animations and IK rigs
- ✅ Game-ready output (Y-up, 30 FPS)

## 🔧 Requirements

**To Run:**
- Windows 10/11
- Rust executables (built by `build.bat`)

**To Export:**
- Blender 3.6 or 4.0+ installed
- Export script at: `D:\GitHub\monorepo-3d\tools\model-3d-editor\blender\blender-export.py`

## ❓ Common Questions

### "Where is my Blender?"
Common paths:
- `C:\Program Files\Blender Foundation\Blender 4.0\blender.exe`
- `C:\Program Files\Blender Foundation\Blender 3.6\blender.exe`
- Steam: `C:\Program Files (x86)\Steam\steamapps\common\Blender\blender.exe`

### "Export script not found"
You need the monorepo-3d repository at:
```
D:\GitHub\monorepo-3d\
```
The script is at:
```
D:\GitHub\monorepo-3d\tools\model-3d-editor\blender\blender-export.py
```

### "Export buttons are disabled"
Configure Blender path in Settings first (⚙️ button).

### "Where are my .glb files?"
Same directory as the source .blend files.
Example:
- Input: `D:\Models\car.blend`
- Output: `D:\Models\car.glb`

## 📖 Documentation

- **README.md** - Full project documentation
- **BLENDER_SETUP.md** - Detailed Blender configuration
- **UI_GUIDE.md** - Visual interface reference
- **Cargo.toml** - Rust dependencies

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| "Blender not configured" | Click ⚙️ Settings, browse for blender.exe, save |
| "Export script not found" | Clone/download monorepo-3d repository |
| "Failed to run Blender" | Check Blender path in Settings is correct |
| Export buttons missing | Configure Blender first |
| Can't save settings | Check write permissions, run as Administrator |

## 💡 Pro Tips

1. **First time?** Just run `build.bat` - it handles everything
2. **Configure once** - Blender path persists forever
3. **Output location** - GLB files appear next to BLEND files
4. **Batch export** - Process entire folders in one click
5. **No environment variables** - Everything is in-app
6. **Recursive scan** - Finds .blend files in all subdirectories

## 🎓 Example Workflow

```
1. Double-click: build.bat
2. App opens
3. Click: ⚙️ Settings
4. Click: 📁 Browse... → select blender.exe
5. Click: 💾 Save Path
6. Click: ← Back
7. Click: 📁 Select Folder → choose your assets folder
8. See list of .blend files
9. Click: 🚀 Export All to GLB
10. Wait for completion
11. ✅ Done! Check the source folders for .glb files
```

## 📦 Export Settings

All exports use these Blender settings:
- Format: GLB (binary)
- Up axis: Y (game engines)
- FPS: 30
- Animation: Yes (with IK solving)
- Compression: None (for compatibility)

Compatible with Three.js, Unity, Unreal, Godot, etc.

## 🔗 Integration

Works with:
- **monorepo-3d** - Uses the same export pipeline
- **Node.js pipeline** - Same `blender-export.py` script
- **Game asset workflows** - Drop-in replacement for manual export

## 📝 License & Credits

Built with:
- Rust + egui (GUI)
- Blender CLI (export engine)
- Integration with monorepo-3d pipeline

---

**Need Help?**
- Check `BLENDER_SETUP.md` for configuration issues
- Check `UI_GUIDE.md` for interface questions
- Check `README.md` for technical details
