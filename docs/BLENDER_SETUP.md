# Blender Setup Guide

## Quick Start - In-App Configuration (Recommended)

The easiest way to configure Blender is directly in the app:

### Step-by-Step

1. **Launch the app** (`build.bat` or `run.bat`)

2. **Open Settings**
   - Click the "⚙️ Settings" button in the top-right corner

3. **Browse for Blender**
   - Click "📁 Browse..." button
   - Navigate to your Blender installation
   - Select `blender.exe`
   - Common location: `C:\Program Files\Blender Foundation\Blender 4.0\blender.exe`

4. **Save the Path**
   - After selecting, you'll see "✓ File exists" in green
   - Click "💾 Save Path" button
   - Done! The path is saved for future sessions

5. **Go Back**
   - Click "← Back" to return to the main screen
   - You should now see "Blender: ✓ Configured" at the top

## Configuration File

Your Blender path is saved in: `blender-manager-config.json`

This file is created in the same directory as the application executable.

**Example content:**
```json
{
  "path": "C:\\Program Files\\Blender Foundation\\Blender 4.0\\blender.exe"
}
```

## Finding Your Blender Installation

### Common Installation Locations

**Standard Windows Installation:**
- `C:\Program Files\Blender Foundation\Blender 4.0\blender.exe`
- `C:\Program Files\Blender Foundation\Blender 3.6\blender.exe`
- `C:\Program Files (x86)\Blender Foundation\blender.exe`

**Steam:**
- `C:\Program Files (x86)\Steam\steamapps\common\Blender\blender.exe`

**Microsoft Store:**
- `C:\Program Files\WindowsApps\BlenderFoundation.Blender*\blender.exe`

**Portable Installation:**
- Wherever you extracted Blender (look for `blender.exe`)

### How to Find It

**Method 1: Using Windows Search**
1. Press `Win + S`
2. Type "blender.exe"
3. Right-click the result → "Open file location"
4. Copy the full path

**Method 2: From Blender Itself**
1. Open Blender
2. Go to **Help** → **System** → **System Info**
3. Look for "Executable" line
4. Copy that path

**Method 3: Command Prompt**
```cmd
where blender
```

## Alternative Configuration Methods

### Option 1: Manual Config File

Create `blender-manager-config.json` next to the app executable:

```json
{
  "path": "C:\\Program Files\\Blender Foundation\\Blender 4.0\\blender.exe"
}
```

⚠️ **Important**: Use double backslashes `\\` in JSON!

### Option 2: Environment Variable (Optional)

```cmd
setx BLENDER_PATH "C:\Program Files\Blender Foundation\Blender 4.0\blender.exe"
```

Restart the app after setting environment variables.

### Option 3: Monorepo Integration

If working with the monorepo, place config at:
```
D:\GitHub\monorepo-3d\tools\model-3d-editor\blender\blender.local.json
```

## Verifying Setup

After configuration:

1. **Look at the top of the main screen:**
   - ✅ **"Blender: ✓ Configured"** (green) = Ready to export!
   - ⚠️ **"Blender: ⚠ Not configured"** (yellow) = Click "Configure →"

2. **Check Settings panel:**
   - Open Settings (⚙️ button)
   - Should show "✓ Blender configured" with the path

3. **Test export:**
   - Scan a folder with .blend files
   - Try exporting one file
   - Should see export progress and success message

## Troubleshooting

### "Blender not configured"
- **Solution**: Click "⚙️ Settings" and browse for `blender.exe`
- Make sure you selected the actual `blender.exe` file, not a folder
- Path must exist and be accessible

### "File not found" error in Settings
- The selected path doesn't exist or was moved
- Browse again to locate the correct `blender.exe`

### "Export script not found"
- Make sure this file exists:
  `D:\GitHub\monorepo-3d\tools\model-3d-editor\blender\blender-export.py`
- This is required for export functionality
- You may need to clone or download the monorepo-3d repository

### Export fails silently
- Check that Blender can actually run:
  ```cmd
  "C:\Path\To\blender.exe" --version
  ```
- Make sure the .blend file isn't corrupted
- Try exporting from Blender UI first to verify the file

### Settings not saving
- Check write permissions for the app directory
- Make sure `blender-manager-config.json` can be created
- Try running the app as Administrator (right-click → Run as Administrator)

## Without Blender

If you don't configure Blender:
- ✅ **Browsing** .blend files still works
- ✅ File scanning and listing works
- ❌ **Export** buttons will be disabled
- ⚠️ Warning message at top: "Blender: ⚠ Not configured"

The app is fully functional for browsing; export features require Blender.

## Changing Blender Path

To change your Blender installation:

1. Click "⚙️ Settings"
2. Click "📁 Browse..." and select the new `blender.exe`
3. Click "💾 Save Path"
4. The new path immediately replaces the old one

Your old configuration is overwritten - no need to manually delete anything.
