# Export Options Guide

## Overview

The Blender File Manager now supports customizable export options to control how .blend files are converted to GLB format.

## Accessing Export Options

Click the **"📤 Export Options"** button in the top-right corner of the main screen (next to ⚙️ Settings).

---

## Available Options

### 1. Animation Settings

**Export Animations** (checkbox)
- **Default:** Enabled ✓
- **When Enabled:** Exports all animations with IK constraint solving
- **When Disabled:** Exports static model only (`--no-anim` flag)
- **Use Case:** Disable for props, environment assets, or static objects

**Example:**
- Character models → Keep enabled
- Building models → Disable for smaller file size

---

### 2. Material Settings

**Export Material Colors** (checkbox)
- **Default:** Enabled ✓
- **When Enabled:** Exports diffuse colors, metallic, and roughness values (`--colors` flag)
- **When Disabled:** Materials may appear white/colorless in GLB
- **Use Case:** Always keep enabled unless you're applying materials in the game engine

**Why This Matters:**
Blender's EXPORT mode keeps `doubleSided` but loses material colors. The `--colors` flag:
1. Extracts colors from Blender materials
2. Saves them to a `.colors.json` sidecar file
3. Patches them back into the GLB after export

**Result:** Your models have proper colors in the final GLB! 🎨

---

### 3. Advanced Settings

#### FPS (Frames Per Second)
- **Default:** 30
- **Range:** 1 - 120
- **Purpose:** Sampling rate for animated rigs
- **Use Case:**
  - 30 FPS: Standard for games
  - 60 FPS: Smoother animations (larger files)
  - Lower FPS: Smaller files, choppier animation

**Example:**
```
FPS = 30  → Standard game animation
FPS = 60  → High-quality cinematic
FPS = 15  → Low-quality/background NPCs
```

#### Scale
- **Default:** 1.0
- **Range:** 0.01 - 100.0
- **Purpose:** Unit normalization multiplier
- **Use Case:** Convert between different unit systems

**Example:**
```
Scale = 1.0   → No change (Blender units)
Scale = 0.01  → Blender cm → Game meters
Scale = 100.0 → Blender m → Game cm
```

**When to Use:**
- Model authored at wrong scale in Blender
- Need to match existing game assets
- Converting between unit systems

---

## Current Settings Summary

The panel shows a summary of your current settings:

```
• Animations: Enabled / Disabled (--no-anim)
• Material Colors: Enabled (--colors) / Disabled
• FPS: 30
• Scale: 1.00
```

This is what will be used for the next export.

---

## Buttons

### ← Back
Returns to the main screen. Settings are saved in memory.

### 🔄 Reset to Defaults
Resets all options to default values:
- Animations: Enabled
- Materials: Enabled
- FPS: 30
- Scale: 1.0

---

## Export Behavior

### With Default Settings
```bash
blender --background --factory-startup --python script.py -- \
  input.blend output.glb \
  --fps 30 \
  --up-axis Y \
  --colors output.glb.colors.json
```

**Result:**
- ✅ Animations included
- ✅ Materials with colors
- ✅ 30 FPS sampling
- ✅ Y-up axis (game-ready)

### With Animations Disabled
```bash
blender --background --factory-startup --python script.py -- \
  input.blend output.glb \
  --fps 30 \
  --up-axis Y \
  --no-anim \
  --colors output.glb.colors.json
```

**Result:**
- ❌ No animations
- ✅ Materials with colors
- ⚡ Faster export
- 📦 Smaller file size

### With Materials Disabled
```bash
blender --background --factory-startup --python script.py -- \
  input.blend output.glb \
  --fps 30 \
  --up-axis Y
```

**Result:**
- ✅ Animations included
- ⚠️ White/colorless materials (needs runtime materials)

---

## Common Workflows

### Character Export (Full Quality)
```
✓ Animations: Enabled
✓ Materials: Enabled
  FPS: 30
  Scale: 1.0
```
**Use for:** Player characters, NPCs with animations

### Prop Export (Static)
```
✗ Animations: Disabled
✓ Materials: Enabled
  FPS: 30 (ignored)
  Scale: 1.0
```
**Use for:** Furniture, rocks, buildings, static objects

### High-Quality Cinematic
```
✓ Animations: Enabled
✓ Materials: Enabled
  FPS: 60
  Scale: 1.0
```
**Use for:** Cutscenes, hero characters, close-up shots

### Background Assets
```
✗ Animations: Disabled
✓ Materials: Enabled
  FPS: 30
  Scale: 1.0
```
**Use for:** Distant objects, environment clutter

### Unit Conversion Example
```
✓ Animations: Enabled
✓ Materials: Enabled
  FPS: 30
  Scale: 0.01
```
**Use for:** Model is 100x too large (authored in cm, need meters)

---

## Tips

### 🎨 Always Export Materials
Unless you're applying materials at runtime, keep "Export Material Colors" enabled. This fixes the white/colorless model issue.

### ⚡ Disable Animations for Static Objects
Saves export time and file size. Buildings, props, and environment assets don't need animations.

### 📊 FPS vs File Size
Higher FPS = smoother animation but larger files. Use 30 FPS for most cases.

### 📏 Scale for Unit Mismatch
If your exported model is the wrong size in-game:
- Too large: Scale < 1.0 (e.g., 0.1 or 0.01)
- Too small: Scale > 1.0 (e.g., 10.0 or 100.0)

### 💾 Settings Persist During Session
Your export options remain active until you close the app. Each export uses the current settings.

### 🔄 Reset When Unsure
Click "Reset to Defaults" to get back to recommended settings.

---

## Troubleshooting

### Models appear white/colorless in GLB
**Solution:** Enable "Export Material Colors"

### File size too large
**Solution:**
- Disable animations if not needed
- Lower FPS (e.g., 15 or 20)

### Model is wrong size in game
**Solution:** Adjust Scale value
- Model 100x too big → Scale = 0.01
- Model 10x too small → Scale = 10.0

### Animations are choppy
**Solution:** Increase FPS (e.g., 60)

### Export takes too long
**Solution:**
- Disable animations for static objects
- Lower FPS if animations aren't critical

---

## Technical Details

### Material Color Export Process

1. **Extract Phase** (`--colors` flag)
   ```python
   # Reads from Blender materials:
   - diffuse_color
   - metallic
   - roughness
   - Base Color (Principled BSDF)
   - Alpha (transparency)
   ```

2. **Sidecar File** (`output.glb.colors.json`)
   ```json
   {
     "Material.001": {
       "diffuse": [0.8, 0.2, 0.1, 1.0],
       "baseColor": [0.8, 0.2, 0.1, 1.0],
       "alpha": 1.0,
       "principled_metallic": 0.0,
       "principled_roughness": 0.5
     }
   }
   ```

3. **Patch Phase** (automatic)
   - Reads the GLB binary
   - Extracts JSON chunk
   - Patches material colors from sidecar
   - Rewrites GLB with correct colors
   - Deletes sidecar file

**Result:** GLB has both `doubleSided` AND correct colors! 🎉

### Animation Export with IK

When animations are enabled (`export_animations: true`):

```python
export_force_sampling=True  # Critical flag!
```

This:
- Solves IK constraints every frame
- Samples the result as keyframes
- Writes baked animation to GLB

**Without this:** Legs with 2 static keys don't move (IK goals swing but legs stay still)

**With this:** Smooth, natural animation with IK solved ✅

---

## Command-Line Equivalent

If you were running the export script manually, these options map to:

| UI Option | CLI Flag |
|-----------|----------|
| Animations: Disabled | `--no-anim` |
| Materials: Enabled | `--colors output.glb.colors.json` |
| FPS: 60 | `--fps 60` |
| Scale: 0.1 | `--scale 0.1` |

**Full command example:**
```bash
blender --background --factory-startup \
  --python blender-export.py -- \
  input.blend output.glb \
  --fps 60 \
  --scale 0.1 \
  --no-anim \
  --colors output.glb.colors.json
```

---

## See Also

- **README.md** - Main documentation
- **BLENDER_SETUP.md** - Configure Blender path
- **UI_GUIDE.md** - Interface reference
- **monorepo-3d/blender-export.py** - The export script source

---

**Need help?** Check the in-app "Current Settings Summary" to verify your configuration before exporting.
