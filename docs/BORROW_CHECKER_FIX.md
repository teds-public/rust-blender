# Technical Notes: Borrow Checker Fix

## Problem

**Error:**
```
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> src/main.rs:462:25
   |
450 | for (i, file) in self.blender_files.iter().enumerate() {
    |                  --------------------- immutable borrow occurs here
...
462 |                         self.export_to_glb(i);
    |                         ^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

## Root Cause

In the file list rendering loop:

```rust
for (i, file) in self.blender_files.iter().enumerate() {
    // ... UI code ...
    if ui.small_button("Export →").clicked() {
        self.export_to_glb(i);  // ❌ Mutable borrow while iterating!
    }
}
```

**Why this fails:**
1. `self.blender_files.iter()` creates an **immutable borrow** of `self.blender_files`
2. `self.export_to_glb()` requires a **mutable borrow** of `&mut self`
3. Rust prevents both from happening simultaneously (aliasing XOR mutability)

## Solution: Deferred Execution Pattern

Store the action request, then execute after iteration completes:

```rust
let mut export_requested = None;

egui::ScrollArea::vertical().show(ui, |ui| {
    egui::Grid::new("blender_files_grid")
        .show(ui, |ui| {
            for (i, file) in self.blender_files.iter().enumerate() {
                // ... UI code ...
                if ui.small_button("Export →").clicked() {
                    export_requested = Some(i);  // ✅ Just store the index
                }
            }
        });
});

// Handle export after iteration completes
if let Some(index) = export_requested {
    self.export_to_glb(index);  // ✅ Now we can mutably borrow
}
```

## Why This Works

1. **During iteration**: Only immutable borrow of `self.blender_files` exists
2. **After iteration**: The immutable borrow ends, `export_requested` contains the index
3. **Then export**: Now we can safely take `&mut self` to call `export_to_glb()`

## Alternative Solutions (Not Used)

### Option 1: Clone the data
```rust
let files = self.blender_files.clone();
for (i, file) in files.iter().enumerate() {
    if ui.small_button("Export →").clicked() {
        self.export_to_glb(i);  // OK, but expensive clone
    }
}
```
❌ Performance cost of cloning all file info on every frame

### Option 2: Indices-only iteration
```rust
for i in 0..self.blender_files.len() {
    let file = &self.blender_files[i];
    // ... rest of loop ...
}
```
❌ Doesn't solve the problem - still borrowing `self.blender_files`

### Option 3: Interior mutability (RefCell)
```rust
struct BlenderApp {
    blender_files: RefCell<Vec<BlenderFileInfo>>,
    // ...
}
```
❌ Overkill for this use case, runtime borrow checking overhead

### Option 4: Split struct into smaller pieces
```rust
struct BlenderApp {
    file_state: FileState,
    export_state: ExportState,
    // ...
}
```
✅ Valid but more complex refactoring than needed here

## Pattern: Deferred Execution in GUI Loops

This is a common pattern in immediate mode GUIs (like egui):

```rust
// General pattern:
let mut action = None;

// Render loop (immutable borrows only)
for item in self.items.iter() {
    if ui.button("Do Something").clicked() {
        action = Some(item.id);
    }
}

// Execute after rendering (mutable borrow allowed)
if let Some(id) = action {
    self.perform_action(id);
}
```

**Why it's common in egui:**
- egui draws everything every frame
- UI events (clicks) must be captured during the frame
- Actions that modify state must happen after the frame
- Separating "what was clicked" from "perform action" solves this cleanly

## Borrow Checker Lesson

This demonstrates Rust's core rule:

> You can have either:
> - One mutable reference (`&mut T`)
> - OR any number of immutable references (`&T`)
> - But never both at the same time

**The fix respects this by:**
1. Using only `&T` during iteration (reading file list)
2. Storing the intent (which file to export)
3. Taking `&mut self` after iteration ends

## Performance Impact

**None.** This pattern:
- Doesn't clone data
- Doesn't box/heap-allocate
- `Option<usize>` is a stack-allocated discriminated union
- Compiler optimizes this to essentially zero overhead

## Similar Fixes in This Codebase

The same pattern is NOT needed for:

```rust
if ui.button("📤 Export Selected").clicked() {
    self.export_to_glb(selected);  // ✅ OK - not inside an iteration
}
```

This is fine because we're not iterating over `self` here - the borrow is straightforward.

---

**Takeaway:** When iterating over `self.something` and needing to mutate `self` based on UI events, defer the mutation until after the iteration completes. Store the event (index, ID, flag) and act on it outside the loop.
