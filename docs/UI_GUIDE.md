# User Interface Guide

## Main Screen

```
┌─────────────────────────────────────────────────────────────┐
│ Blender File Manager & Exporter               [⚙️ Settings] │
├─────────────────────────────────────────────────────────────┤
│ Blender: ✓ Configured (blender.exe)                        │
├─────────────────────────────────────────────────────────────┤
│ [📁 Select Folder]  Selected: D:\Projects\Assets           │
│                                                              │
│ [🚀 Export All to GLB]  [📤 Export Selected]               │
│                                                              │
│ ┌────────────────────────────────────────────────────────┐ │
│ │ ✅ Successfully exported to: model.glb                  │ │
│ └────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ Found 5 Blender file(s)                                     │
│                                                              │
│ ┌─────────────────────┬──────────┬───────────────────────┐ │
│ │ File Path           │ Size     │ Action                │ │
│ ├─────────────────────┼──────────┼───────────────────────┤ │
│ │ █ models\car.blend  │ 2.34 MB  │ [Export →]           │ │
│ │   models\tree.blend │ 1.12 MB  │ [Export →]           │ │
│ │   assets\rock.blend │ 856 KB   │ [Export →]           │ │
│ └─────────────────────┴──────────┴───────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Key Elements

**Status Bar:**
- ✓ Configured = Blender ready, export enabled
- ⚠ Not configured = Need to configure Blender (click "Configure →")

**Folder Selection:**
- Click "📁 Select Folder" to browse
- Shows selected path after scanning

**Export Controls:**
- "🚀 Export All to GLB" - Batch export all files
- "📤 Export Selected" - Export the highlighted file (appears after selecting)

**Progress Box:**
- Shows export status and results
- ✅ Success, ⚠️ Warnings, ❌ Errors

**File List:**
- Click a row to select it (highlights in blue)
- Shows full path, file size, and individual export button
- Striped rows for readability

---

## Settings Screen

```
┌─────────────────────────────────────────────────────────────┐
│ ⚙️ Settings                                                  │
├─────────────────────────────────────────────────────────────┤
│ ┌────────────────────────────────────────────────────────┐ │
│ │ Blender Executable Path:                                │ │
│ │                                                          │ │
│ │ [C:\Program Files\Blender\blender.exe] [📁 Browse...]  │ │
│ │                                                          │ │
│ │ ✓ File exists                         [💾 Save Path]   │ │
│ └────────────────────────────────────────────────────────┘ │
│                                                              │
│ ┌────────────────────────────────────────────────────────┐ │
│ │ Current Status:                                         │ │
│ │ ✓ Blender configured                                    │ │
│ │ Path: C:\Program Files\Blender Foundation\...           │ │
│ └────────────────────────────────────────────────────────┘ │
│                                                              │
│ ┌────────────────────────────────────────────────────────┐ │
│ │ Configuration File:                                     │ │
│ │ blender-manager-config.json                             │ │
│ │ Your Blender path is saved here for future sessions    │ │
│ └────────────────────────────────────────────────────────┘ │
│                                                              │
│ [← Back]                                                    │
└─────────────────────────────────────────────────────────────┘
```

### Settings Elements

**Path Input:**
- Text field showing current path
- "📁 Browse..." button opens file picker
- Auto-validates if file exists

**Status Indicators:**
- ✓ File exists (green) = Valid path, can save
- ✗ File not found (red) = Invalid path, fix before saving

**Save Button:**
- "💾 Save Path" becomes clickable when valid path is entered
- Saves to `blender-manager-config.json`

**Current Status:**
- Shows whether Blender is configured
- Displays the active path

**Configuration Info:**
- Shows where settings are stored
- Helps locate the config file for manual editing if needed

**Navigation:**
- "← Back" returns to main screen

---

## Workflow Examples

### First Time Setup

1. Launch app
2. See "⚠ Not configured" warning
3. Click "⚙️ Settings"
4. Click "📁 Browse..."
5. Navigate to: `C:\Program Files\Blender Foundation\Blender 4.0\`
6. Select `blender.exe`
7. See "✓ File exists"
8. Click "💾 Save Path"
9. Click "← Back"
10. Now see "✓ Configured"

### Export Single File

1. Click "📁 Select Folder"
2. Choose folder with .blend files
3. See list of files appear
4. Click on `models\car.blend` (row highlights)
5. Click "📤 Export Selected"
6. See progress: "Exporting models\car.blend to GLB..."
7. See result: "✅ Successfully exported to: models\car.glb"

### Batch Export

1. Scan folder (has 10 .blend files)
2. Click "🚀 Export All to GLB"
3. See progress update: "Exporting 3/10: tree.blend"
4. Wait for completion
5. See result: "✅ Batch export completed! Success: 10, Failed: 0"

### Troubleshooting Export

If export fails:
- Check progress box for error message
- "Export script not found" → Check monorepo-3d is available
- "Failed to run Blender" → Check Settings, verify Blender path
- Re-select Blender path in Settings if it was moved

---

## Keyboard & Mouse

**Mouse:**
- Click row to select file
- Click button to trigger action
- Scroll in file list area

**Keyboard:**
- Use Tab to navigate between controls
- Enter/Space to activate buttons
- Arrow keys in text fields

---

## File List Columns

| Column    | Description                              |
|-----------|------------------------------------------|
| File Path | Full path to .blend file                 |
| Size      | File size (B, KB, MB, or GB)             |
| Action    | "Export →" button (per-file export)      |

Selected row is highlighted for "Export Selected" action.

---

## Status Messages

**Success:**
- ✅ Successfully exported to: path\file.glb
- ✅ Batch export completed! Success: N, Failed: M

**Progress:**
- Exporting file.blend to GLB...
- Exporting 3/10: file.blend

**Warnings:**
- ⚠ Export completed with warnings: [details]
- ⚠ Blender not configured

**Errors:**
- ❌ Export failed: [error details]
- ❌ Failed to run Blender: [error]
- Error: Export script not found at [path]
- Error: Blender not configured. Click ⚙️ Settings...

---

## Tips

1. **Select before Export**: Click a file row first, then "📤 Export Selected" appears
2. **Batch Export**: No need to select - "🚀 Export All" processes everything
3. **Output Location**: .glb files are created next to .blend files (same directory)
4. **Recursive Scan**: Folder selection scans all subdirectories
5. **Config Persistence**: Blender path is saved - only configure once
6. **Visual Feedback**: Export progress shows in the gray box below buttons
