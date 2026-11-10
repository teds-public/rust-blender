# Project Structure

```
rust-blender/
├── src/
│   └── main.rs                    # Main application code
│
├── target/                        # Build output (generated)
│   ├── debug/
│   │   └── blender_file_finder.exe
│   └── release/
│       └── blender_file_finder.exe
│
├── Cargo.toml                     # Rust dependencies & metadata
├── Cargo.lock                     # Locked dependency versions
│
├── blender-manager-config.json    # Auto-generated user config
│
├── build.bat                      # Build & run (release mode)
├── run.bat                        # Run (development mode)
├── run-release.bat                # Run (release mode)
├── setup.ps1                      # PowerShell auto-setup (optional)
│
├── README.md                      # Main documentation
├── QUICKSTART.md                  # Quick reference guide
├── BLENDER_SETUP.md               # Detailed Blender setup guide
├── UI_GUIDE.md                    # Visual UI reference
├── CHANGELOG.md                   # Version history
└── PROJECT_STRUCTURE.md           # This file
```

## File Descriptions

### Source Code

**`src/main.rs`** (540 lines)
- Main application entry point
- GUI using egui/eframe
- Blender path configuration logic
- File scanning with walkdir
- Export logic via Blender CLI
- Settings panel UI
- File list display

### Configuration

**`Cargo.toml`**
- Package metadata (name, version, edition)
- Dependencies: eframe, egui, rfd, walkdir, serde, serde_json
- Release build profile settings

**`blender-manager-config.json`** (auto-generated)
```json
{
  "path": "C:\\Program Files\\Blender Foundation\\Blender 4.0\\blender.exe"
}
```
- Created when user saves Blender path in Settings
- Located next to the executable
- UTF-8 JSON format

### Batch Scripts (Windows)

**`build.bat`**
- Adds Cargo to PATH
- Builds in release mode
- Runs the application
- Best for end users

**`run.bat`**
- Development mode (faster compile)
- Useful for testing changes
- Less optimized binary

**`run-release.bat`**
- Runs existing release build
- No recompilation
- Fastest startup if already built

**`setup.ps1`**
- PowerShell alternative
- Auto-detects Blender
- Creates config file
- Optional convenience script

### Documentation

**`README.md`** - Main docs
- Features overview
- Quick start guide
- Build instructions
- Usage examples
- Technical details

**`QUICKSTART.md`** - Quick reference
- 3-step getting started
- Common questions
- Troubleshooting
- Pro tips

**`BLENDER_SETUP.md`** - Blender configuration
- In-app setup walkthrough
- Finding Blender path
- Alternative config methods
- Troubleshooting

**`UI_GUIDE.md`** - Interface reference
- ASCII art UI mockups
- Screen layouts
- Workflow examples
- Keyboard/mouse shortcuts

**`CHANGELOG.md`** - Version history
- Feature additions
- Breaking changes
- Migration guides
- Roadmap

**`PROJECT_STRUCTURE.md`** - This file
- Directory layout
- File purposes
- Code organization

## Code Organization

### Main Components

```rust
// Application state
struct BlenderApp {
    selected_folder: Option<PathBuf>,   // Current scanned folder
    blender_files: Vec<BlenderFileInfo>, // Found .blend files
    blender_path: Option<PathBuf>,      // Configured Blender
    export_progress: String,             // Status messages
    show_settings: bool,                 // Settings panel toggle
    config_file: PathBuf,                // Config file location
    // ...
}

// File metadata
struct BlenderFileInfo {
    path: PathBuf,  // Full path to .blend
    size: u64,      // File size in bytes
}

// Saved config format
struct BlenderConfig {
    path: String,   // Blender executable path
}
```

### Key Functions

```rust
// Initialization
BlenderApp::new()                      // Load saved config

// Configuration
BlenderApp::load_blender_path()        // Read config file
BlenderApp::save_blender_path()        // Write config file
BlenderApp::browse_blender_exe()       // File picker dialog

// File operations
BlenderApp::scan_folder()              // Find all .blend files
BlenderApp::export_to_glb()            // Export single file
BlenderApp::export_all()               // Batch export

// UI
BlenderApp::render_settings()          // Settings panel
eframe::App::update()                  // Main UI loop

// Utilities
format_size()                          // Human-readable file size
```

## Dependencies

### Runtime Dependencies

```toml
[dependencies]
eframe = "0.30"      # Application framework
egui = "0.30"        # Immediate mode GUI
rfd = "0.15"         # Native file dialogs
walkdir = "2.5"      # Recursive directory traversal
serde = "1.0"        # Serialization framework
serde_json = "1.0"   # JSON serialization
```

### Why These Dependencies?

- **egui/eframe**: Lightweight, cross-platform GUI
- **rfd**: Native OS file picker dialogs
- **walkdir**: Efficient recursive file scanning
- **serde**: Standard Rust serialization
- **serde_json**: JSON config file handling

## Build Artifacts

### Debug Build
```
target/debug/blender_file_finder.exe    (~50 MB)
```
- Fast compilation
- Includes debug symbols
- Less optimized

### Release Build
```
target/release/blender_file_finder.exe  (~8 MB)
```
- Slower compilation
- Optimized for performance
- Smaller binary size
- Recommended for distribution

## External Dependencies

### Required at Runtime

**Blender** (optional - for export only)
- Any version 3.5+
- Typically: `C:\Program Files\Blender Foundation\Blender *\blender.exe`
- Not bundled - user configures path

**Export Script**
- Path: `D:\GitHub\monorepo-3d\tools\model-3d-editor\blender\blender-export.py`
- Hardcoded in current version
- Must exist for export to work

### Not Required

- ❌ Python runtime
- ❌ Node.js
- ❌ Additional DLLs
- ❌ Internet connection

The app is self-contained except for Blender + export script.

## Configuration Cascade

The app searches for Blender in this order:

```
1. blender-manager-config.json          (app-generated, local)
   ↓ (if not found or invalid)
2. monorepo-3d/.../blender.local.json   (monorepo, machine-specific)
   ↓ (if not found or invalid)
3. monorepo-3d/.../blender.json         (monorepo, committed default)
   ↓ (if not found or invalid)
4. BLENDER_PATH environment variable     (system-wide)
   ↓ (if not set)
5. Not configured                        (export disabled)
```

First valid path wins.

## Data Flow

### Startup
```
main()
  → BlenderApp::new()
    → load_blender_path()
      → Try: app config
      → Try: monorepo configs
      → Try: environment variable
  → eframe::run_native()
```

### Configuration
```
User clicks "📁 Browse..."
  → browse_blender_exe()
    → rfd::FileDialog::pick_file()
  → User selects blender.exe
  → save_blender_path()
    → Create BlenderConfig struct
    → serde_json::to_string_pretty()
    → fs::write(config_file)
  → blender_path updated
```

### Scanning
```
User clicks "📁 Select Folder"
  → rfd::FileDialog::pick_folder()
  → scan_folder()
    → walkdir::WalkDir::new()
    → Filter *.blend files
    → Collect paths + sizes
  → blender_files populated
  → UI updates with file list
```

### Export
```
User clicks "Export →"
  → export_to_glb()
    → Validate blender_path
    → Construct output .glb path
    → std::process::Command::new(blender_exe)
      → --background --factory-startup
      → --python blender-export.py
      → -- input.blend output.glb --flags
    → Wait for completion
    → Parse stdout for success/error
  → export_progress updated
  → UI shows result
```

## Platform-Specific Notes

### Windows
- Uses `\` path separators (automatic via PathBuf)
- .exe extension required for executables
- Batch scripts for easy launching

### Future: Linux/macOS
- Would use `/` path separators (automatic via PathBuf)
- No .exe extension
- Would need shell scripts instead of .bat

Rust's PathBuf handles most platform differences automatically.

## Performance Characteristics

### File Scanning
- Complexity: O(n) where n = total files in directory tree
- Typical: < 1 second for ~1000 files
- Bottleneck: Disk I/O

### Export
- Single file: ~2-10 seconds (depends on .blend complexity)
- Batch: Sequential (n × single-file time)
- Bottleneck: Blender processing
- UI freezes during export (synchronous)

### Memory
- Minimal (~10-20 MB baseline)
- Scales with number of files found (trivial per-file)

## Security Considerations

### User Input
- ✅ File paths validated via fs::exists()
- ✅ Config file is JSON (safe deserialization)
- ✅ No code execution from config

### External Commands
- ⚠️ Runs Blender with user-selected path
- ⚠️ Passes file paths to Blender CLI
- ✅ No shell injection (using Command::args(), not shell)

### File System
- ✅ Only reads .blend files (no modification)
- ✅ Creates .glb files in same directory
- ✅ Config file only in app directory

## Future Architecture Ideas

### Async Export
```rust
// Current: synchronous (UI blocks)
export_to_glb()

// Future: async with progress
async fn export_to_glb_async() {
    tokio::spawn(async move {
        // Export in background
        // Update progress via channel
    })
}
```

### Plugin System
```rust
trait ExportPlugin {
    fn name() -> &'static str;
    fn export(&self, blend: Path, output: Path) -> Result<()>;
}
```

### Database
```rust
// Cache file metadata, export history
struct ExportHistory {
    blend_path: PathBuf,
    glb_path: PathBuf,
    timestamp: DateTime,
    success: bool,
}
```

These are ideas, not implemented.

## Development Workflow

### Making Changes
1. Edit `src/main.rs`
2. Run `cargo run` to test (or `run.bat`)
3. Fix errors, iterate
4. Test export functionality
5. Run `cargo build --release` for final binary

### Adding Features
1. Update `src/main.rs`
2. Update relevant docs (README, CHANGELOG)
3. Test on actual .blend files
4. Build release binary
5. Update version in Cargo.toml

### Distribution
- Share `target/release/blender_file_finder.exe`
- Include `README.md` and `QUICKSTART.md`
- User configures Blender in-app
- No installer needed (standalone .exe)

---

## Summary

This is a single-binary Rust application with:
- One source file (~540 lines)
- Six dependencies
- Three batch script launchers
- Five documentation files
- In-app configuration
- Integration with Blender CLI

Simple, focused, effective.
