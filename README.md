# Blender File Manager & Exporter

Rust desktop application to browse Blender (.blend) files and export them to GLB format using Blender's CLI.

## Features

- 📁 Select folder via file dialog
- 🔍 Recursively scan for .blend files
- 📊 Display file paths and sizes
- 📤 Export individual .blend files to GLB
- 🚀 Batch export all files to GLB
- ⚙️ Auto-detect Blender from config or environment
- 🖥️ Clean, modern GUI

## Prerequisites

- Rust toolchain (with MSVC target for Windows)
- Visual Studio Build Tools (for MSVC linker)
- **Blender** (for GLB export functionality)

## For Developers

### Prerequisites
- Rust toolchain (MSVC target for Windows)
- Visual Studio Build Tools

### Build from Source

```bash
# Set MSVC as default toolchain (first time only)
rustup default stable-x86_64-pc-windows-msvc

# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run --release
```

Output: `target/release/blender_file_finder.exe`

The app provides an in-app settings panel to configure Blender:

1. Click "⚙️ Settings" button in the top-right corner
2. Click "📁 Browse..." to select your `blender.exe` file
3. Click "💾 Save Path" to save the configuration

Your Blender path is saved in `blender-manager-config.json` (next to the executable) and persists across sessions.

### Auto-Detection Fallback

If not configured in-app, the app will auto-detect Blender from:
1. `blender-manager-config.json` (saved by the app)
2. `D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/blender.local.json`
3. `D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/blender.json`
4. `BLENDER_PATH` environment variable

### Finding Blender

Common installation locations:
- `C:\Program Files\Blender Foundation\Blender 4.0\blender.exe`
- `C:\Program Files\Blender Foundation\Blender 3.6\blender.exe`
- Steam: `C:\Program Files (x86)\Steam\steamapps\common\Blender\blender.exe`

## 🚀 Quick Start

1. **Download/Build** the app:
   ```cmd
   # Double-click one of these:
   build.bat           # Build and run (release mode)
   run.bat            # Run development mode
   ```

2. **First Launch - Configure Blender:**
   - Click "⚙️ Settings"
   - Click "📁 Browse..." → Select `blender.exe`
   - Click "💾 Save Path"
   - Done! ✓

3. **Use the app:**
   - Click "📁 Select Folder"
   - Browse and export .blend files to GLB

See `BLENDER_SETUP.md` for detailed setup instructions.

## Usage

### 1. Configure Blender (First Time Only)

On first launch:
1. Click "⚙️ Settings" in the top-right
2. Click "📁 Browse..." and select your `blender.exe`
3. Click "💾 Save Path"
4. Click "← Back"

This configuration persists across sessions. See `BLENDER_SETUP.md` for details.

### 2. Browse Blender Files
1. Click "📁 Select Folder" 
2. Choose a folder containing .blend files
3. The app scans recursively and displays all files

### 3. Export to GLB

**Configure Export Options (Optional):**
1. Click "📤 Export Options"
2. Adjust settings:
   - **Animations:** Enable/disable animation export
   - **Material Colors:** Enable to fix white/colorless models
   - **FPS:** Animation sampling rate (default: 30)
   - **Scale:** Unit normalization (default: 1.0)
3. Click "← Back"

**Export Single File:**
1. Click on a file to select it (row highlights)
2. Click "📤 Export Selected"
3. Output .glb created in the same directory

**Export All Files:**
1. Click "🚀 Export All to GLB"
2. Batch processes all files
3. Shows progress and final statistics

Export uses Blender's built-in GLB exporter with your configured options.
See [EXPORT_OPTIONS.md](EXPORT_OPTIONS.md) for detailed option descriptions.

## Dependencies

- `eframe` / `egui` - Immediate mode GUI framework
- `rfd` - File dialog
- `walkdir` - Recursive directory traversal
- `serde` / `serde_json` - JSON configuration parsing

## Technical Notes

- Uses egui for cross-platform native GUI
- Synchronous file scanning (suitable for moderate folder sizes)
- Supports .blend file extension (case-insensitive)
- Displays file sizes in B/KB/MB/GB format
- Export uses Blender's Python API via `blender-export.py` script
- Compatible with the monorepo-3d pipeline
- Spawns Blender in `--background` mode for headless export

## Integration with monorepo-3d

This tool integrates with the existing Blender pipeline at:
`D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/`

It uses the same export script and configuration as the Node.js pipeline, ensuring consistent GLB output across workflows.
