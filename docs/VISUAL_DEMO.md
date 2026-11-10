# Visual Demo Guide

ASCII mockups of the app interface for documentation.

## 1. First Launch (Not Configured)

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ⚠ Not configured                                     │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│  ⚠️ Configure Blender first to enable export                   │
│     Configure →                                                 │
│                                                                 │
│                                                                 │
│  No folder selected yet.                                       │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**User action:** Click ⚙️ (top-right) or "Configure →" link

---

## 2. Settings Panel

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚙️ Settings                                                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender Executable Path                                       │
│                                                                 │
│  ┌───────────────────────────────────────────────────┐         │
│  │ C:\Program Files\Blender Foundation\Blender 4...  │ 📁 Browse│
│  └───────────────────────────────────────────────────┘         │
│                                                                 │
│  Status: ✓ Path is valid                                       │
│                                                                 │
│  💾 Save Path                                                  │
│                                                                 │
│  ────────────────────────────────────────────────────────      │
│                                                                 │
│  Current Configuration:                                        │
│   Blender: ✓ Configured                                       │
│   Path: C:\Program Files\Blender Foundation\Blender 4.0\...   │
│                                                                 │
│  ← Back                                                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**User actions:**
1. Click "📁 Browse" → Select blender.exe
2. Click "💾 Save Path"
3. Click "← Back"

---

## 3. Main Screen (Configured, No Folder)

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ✓ Configured                                         │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│                                                                 │
│  No folder selected yet.                                       │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**User action:** Click "📁 Select Folder" → Pick a directory

---

## 4. Scanning Folder

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ✓ Configured                                         │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│  Selected: D:\Projects\3D\Models                               │
│                                                                 │
│  ──────────────────────────────────────────────────            │
│                                                                 │
│  🔄 Scanning...                                                │
│                                                                 │
│                                                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Wait for scan to complete...**

---

## 5. Files Found (Ready to Export)

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ✓ Configured                                         │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│  Selected: D:\Projects\3D\Models                               │
│                                                                 │
│  🚀 Export All to GLB    📤 Export Selected                    │
│                                                                 │
│  ──────────────────────────────────────────────────            │
│                                                                 │
│  Found 3 Blender file(s)                                       │
│                                                                 │
│  ┌─────────────────────────────────────────────────────┐       │
│  │ File Path                         │ Size    │ Action│       │
│  ├───────────────────────────────────┼─────────┼───────┤       │
│  │ character.blend                   │ 2.4 MB  │Export→│       │
│  │ environment.blend                 │ 15.7 MB │Export→│       │
│  │ props\sword.blend                 │ 845 KB  │Export→│       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**User actions:**
- Click "Export →" on a row to export single file
- Select a row (click name), then "📤 Export Selected"
- Click "🚀 Export All to GLB" for batch export

---

## 6. Exporting (In Progress)

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ✓ Configured                                         │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│  Selected: D:\Projects\3D\Models                               │
│                                                                 │
│  🚀 Export All to GLB    📤 Export Selected                    │
│                                                                 │
│  ┌─────────────────────────────────────────────────────┐       │
│  │ ⏳ Exporting: character.blend → character.glb...    │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
│  ──────────────────────────────────────────────────            │
│                                                                 │
│  Found 3 Blender file(s)                                       │
│  [File list still visible below...]                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Wait for Blender to process...**

---

## 7. Export Complete

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ✓ Configured                                         │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│  Selected: D:\Projects\3D\Models                               │
│                                                                 │
│  🚀 Export All to GLB    📤 Export Selected                    │
│                                                                 │
│  ┌─────────────────────────────────────────────────────┐       │
│  │ ✅ Successfully exported:                            │       │
│  │    character.blend → character.glb                   │       │
│  │                                                      │       │
│  │ ✅ Successfully exported:                            │       │
│  │    environment.blend → environment.glb               │       │
│  │                                                      │       │
│  │ ✅ Successfully exported:                            │       │
│  │    props\sword.blend → props\sword.glb               │       │
│  │                                                      │       │
│  │ 🎉 All exports completed!                            │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Done!** Check output .glb files in the same folders as .blend files.

---

## 8. Export Error Example

```
┌─────────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter                              ⚙️ │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Blender: ✓ Configured                                         │
│                                                                 │
│  📁 Select Folder                                              │
│                                                                 │
│  ┌─────────────────────────────────────────────────────┐       │
│  │ ❌ Export failed for environment.blend:              │       │
│  │    Export script not found:                          │       │
│  │    D:\GitHub\monorepo-3d\tools\model-3d-editor\      │       │
│  │    blender\blender-export.py                         │       │
│  │                                                      │       │
│  │ Please check:                                        │       │
│  │ 1. Export script exists at the path above           │       │
│  │ 2. Blender path is correct                          │       │
│  │ 3. .blend file is not corrupted                     │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Troubleshooting:** Check error message for details.

---

## UI Elements Legend

| Symbol | Meaning |
|--------|---------|
| ⚙️ | Settings button |
| 📁 | Browse/select folder |
| 🚀 | Export all files |
| 📤 | Export selected file |
| 💾 | Save configuration |
| ← | Back to main screen |
| ✓ | Valid/configured |
| ⚠ | Warning/not configured |
| ✗ | Invalid/error |
| 🔄 | Loading/scanning |
| ⏳ | Processing/exporting |
| ✅ | Success |
| ❌ | Error/failed |
| 🎉 | All tasks complete |

---

## Keyboard/Mouse Interactions

### Main Screen
- **Click folder path**: Select row
- **Click "Export →"**: Export that file
- **Click ⚙️**: Open settings

### Settings Screen
- **Type in text field**: Edit path (or use Browse)
- **Click "Browse"**: Native file picker
- **Click "Save"**: Persist config
- **Click "Back"**: Return to main

### File List
- **Click anywhere on row**: Select that file
- **Selected row**: Highlighted background
- **Scroll**: Mouse wheel or drag scrollbar

---

## Window Behavior

### Resize
- Minimum: 800×600
- Default: 1000×700
- Scrollable file list
- Fixed top controls

### Close
- X button or Alt+F4
- Config saved automatically
- No confirmation needed

---

## Color Scheme (Approximate)

- Background: Dark gray (#2B2B2B)
- Text: Light gray (#CCCCCC)
- Headers: White (#FFFFFF)
- Buttons: Blue accent (#0078D7)
- Success: Green (#4CAF50)
- Warning: Orange (#FF9800)
- Error: Red (#F44336)
- Selected row: Lighter gray (#3C3C3C)
- Borders: Medium gray (#555555)

---

## Typical Workflows

### One-Time Setup
```
Launch → ⚙️ Settings → Browse → Select blender.exe → Save → Back → Done
```
(~30 seconds)

### Daily Usage
```
Launch → Select Folder → Export All → Done
```
(~5 seconds + export time)

### Single File Export
```
Launch → Select Folder → Click row → Export Selected → Done
```
(~5 seconds + export time)

### Review Without Export
```
Launch → Select Folder → Review list → Close
```
(Just browsing)

---

These mockups show the key screens and interactions. The actual app renders with native OS styling via egui/eframe.
