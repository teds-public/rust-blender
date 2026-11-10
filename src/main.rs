use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Blender File Manager & Exporter"),
        ..Default::default()
    };

    eframe::run_native(
        "Blender File Manager & Exporter",
        options,
        Box::new(|_cc| Ok(Box::new(BlenderApp::new()))),
    )
}

#[derive(Serialize, Deserialize)]
struct BlenderConfig {
    path: String,
}

#[derive(Clone)]
struct ExportOptions {
    export_animations: bool,
    export_materials: bool,
    fps: i32,
    scale: f32,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            export_animations: true,
            export_materials: true,
            fps: 30,
            scale: 1.0,
        }
    }
}

#[derive(Default)]
struct BlenderApp {
    selected_folder: Option<PathBuf>,
    blender_files: Vec<BlenderFileInfo>,
    is_scanning: bool,
    blender_path: Option<PathBuf>,
    export_progress: String,
    selected_file: Option<usize>,
    show_settings: bool,
    show_export_options: bool,
    blender_path_input: String,
    config_file: PathBuf,
    export_options: ExportOptions,
}

#[derive(Clone)]
struct BlenderFileInfo {
    path: PathBuf,
    size: u64,
}

impl BlenderApp {
    fn new() -> Self {
        let config_file = PathBuf::from("blender-manager-config.json");
        let blender_path = Self::load_blender_path(&config_file);
        let blender_path_input = blender_path.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();

        Self {
            blender_path,
            blender_path_input,
            config_file,
            ..Default::default()
        }
    }

    fn load_blender_path(config_file: &Path) -> Option<PathBuf> {
        if let Ok(content) = fs::read_to_string(config_file) {
            if let Ok(config) = serde_json::from_str::<BlenderConfig>(&content) {
                let path = PathBuf::from(config.path);
                if path.exists() {
                    return Some(path);
                }
            }
        }

        // Fallback: check monorepo configs
        let config_paths = [
            "D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/blender.local.json",
            "D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/blender.json",
        ];

        for config_path in &config_paths {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(config) = serde_json::from_str::<BlenderConfig>(&content) {
                    let path = PathBuf::from(config.path);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }

        // Check environment variable
        if let Ok(env_path) = std::env::var("BLENDER_PATH") {
            let path = PathBuf::from(env_path);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    fn save_blender_path(&mut self, path: PathBuf) {
        self.blender_path = Some(path.clone());
        let config = BlenderConfig {
            path: path.display().to_string(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            let _ = fs::write(&self.config_file, json);
        }
    }

    fn browse_blender_exe(&mut self) {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("Blender Executable", &["exe"])
            .set_file_name("blender.exe")
            .pick_file()
        {
            self.blender_path_input = file.display().to_string();
            self.save_blender_path(file);
        }
    }

    fn patch_material_colors(glb_path: &Path, colors_json_path: &Path) -> Result<(), String> {
        // Read colors JSON
        let colors_content = match fs::read_to_string(colors_json_path) {
            Ok(content) => content,
            Err(_) => return Ok(()), // No colors file, skip patching
        };

        let colors: Value = match serde_json::from_str(&colors_content) {
            Ok(val) => val,
            Err(e) => return Err(format!("Failed to parse colors JSON: {}", e)),
        };

        // Read GLB binary
        let buf = match fs::read(glb_path) {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed to read GLB: {}", e)),
        };

        if buf.len() < 20 {
            return Err("GLB file too small".to_string());
        }

        // Read JSON chunk length (offset 12, little-endian u32)
        let json_len = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]) as usize;
        let json_start = 20;

        if buf.len() < json_start + json_len {
            return Err("Invalid GLB structure".to_string());
        }

        // Parse GLB JSON chunk
        let json_bytes = &buf[json_start..json_start + json_len];
        let json_str = match std::str::from_utf8(json_bytes) {
            Ok(s) => s.trim_end(),
            Err(e) => return Err(format!("Invalid UTF-8 in GLB JSON: {}", e)),
        };

        let mut gltf_json: Value = match serde_json::from_str(json_str) {
            Ok(val) => val,
            Err(e) => return Err(format!("Failed to parse GLB JSON: {}", e)),
        };

        // Patch materials
        let mut changed = false;
        if let Some(materials) = gltf_json.get_mut("materials").and_then(|m| m.as_array_mut()) {
            for mat in materials {
                let mat_name = match mat.get("name").and_then(|n| n.as_str()) {
                    Some(name) => name,
                    None => continue,
                };

                let src = match colors.get(mat_name) {
                    Some(val) => val,
                    None => continue,
                };

                let color = src.get("baseColor")
                    .or_else(|| src.get("diffuse"));

                if let Some(c) = color.and_then(|v| v.as_array()) {
                    if c.len() >= 3 {
                        // Ensure pbrMetallicRoughness exists
                        if !mat.get("pbrMetallicRoughness").is_some() {
                            mat["pbrMetallicRoughness"] = serde_json::json!({});
                        }

                        let alpha = src.get("alpha")
                            .and_then(|v| v.as_f64())
                            .or_else(|| c.get(3).and_then(|v| v.as_f64()))
                            .unwrap_or(1.0);

                        mat["pbrMetallicRoughness"]["baseColorFactor"] = serde_json::json!([
                            c[0].as_f64().unwrap_or(1.0),
                            c[1].as_f64().unwrap_or(1.0),
                            c[2].as_f64().unwrap_or(1.0),
                            alpha
                        ]);

                        if alpha < 1.0 {
                            mat["alphaMode"] = serde_json::json!("BLEND");
                        }

                        if let Some(metallic) = src.get("principled_metallic").and_then(|v| v.as_f64()) {
                            mat["pbrMetallicRoughness"]["metallicFactor"] = serde_json::json!(metallic);
                        }

                        if let Some(roughness) = src.get("principled_roughness").and_then(|v| v.as_f64()) {
                            mat["pbrMetallicRoughness"]["roughnessFactor"] = serde_json::json!(roughness);
                        }

                        changed = true;
                    }
                }
            }
        }

        if !changed {
            return Ok(());
        }

        // Rebuild GLB with patched JSON
        let bin_chunk = &buf[json_start + json_len..];

        // Serialize JSON and pad to 4-byte alignment
        let mut json_text = serde_json::to_string(&gltf_json)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        while json_text.len() % 4 != 0 {
            json_text.push(' ');
        }

        let json_bytes = json_text.as_bytes();

        // Build new GLB
        let mut new_glb = Vec::new();

        // Copy original header (12 bytes)
        new_glb.extend_from_slice(&buf[0..12]);

        // JSON chunk header (8 bytes)
        new_glb.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
        new_glb.extend_from_slice(b"JSON");

        // JSON chunk data
        new_glb.extend_from_slice(json_bytes);

        // BIN chunk (copy as-is)
        new_glb.extend_from_slice(bin_chunk);

        // Update total file length in header (offset 8)
        let total_len = (new_glb.len() as u32).to_le_bytes();
        new_glb[8..12].copy_from_slice(&total_len);

        // Write patched GLB
        fs::write(glb_path, new_glb)
            .map_err(|e| format!("Failed to write patched GLB: {}", e))?;

        Ok(())
    }

    fn scan_folder(&mut self, folder: PathBuf) {
        self.selected_folder = Some(folder.clone());
        self.blender_files.clear();
        self.is_scanning = true;
        self.selected_file = None;

        let mut files = Vec::new();

        for entry in WalkDir::new(folder)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("blend") {
                        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                        files.push(BlenderFileInfo {
                            path: path.to_path_buf(),
                            size,
                        });
                    }
                }
            }
        }

        self.blender_files = files;
        self.is_scanning = false;
    }

    fn export_to_glb(&mut self, file_index: usize) {
        if file_index >= self.blender_files.len() {
            return;
        }

        let blend_file = &self.blender_files[file_index];
        let blend_path = &blend_file.path;

        let mut output_path = blend_path.clone();
        output_path.set_extension("glb");

        self.export_progress = format!("Exporting {} to GLB...", blend_path.display());

        let blender_exe = match &self.blender_path {
            Some(path) => path.clone(),
            None => {
                self.export_progress = "Error: Blender not configured. Click ⚙️ Settings to set Blender path".to_string();
                return;
            }
        };

        let script_path = "D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/blender-export.py";

        if !Path::new(script_path).exists() {
            self.export_progress = format!("Error: Export script not found at {}", script_path);
            return;
        }

        // Build arguments based on export options
        let mut args = vec![
            "--background".to_string(),
            "--factory-startup".to_string(),
            "--python".to_string(),
            script_path.to_string(),
            "--".to_string(),
            blend_path.to_str().unwrap().to_string(),
            output_path.to_str().unwrap().to_string(),
            "--fps".to_string(),
            self.export_options.fps.to_string(),
            "--up-axis".to_string(),
            "Y".to_string(),
        ];

        // Track colors.json file for cleanup
        let colors_json_path = if self.export_options.export_materials {
            let path = format!("{}.colors.json", output_path.to_str().unwrap());
            args.push("--colors".to_string());
            args.push(path.clone());
            Some(PathBuf::from(path))
        } else {
            None
        };

        // Add --no-anim if animations are disabled
        if !self.export_options.export_animations {
            args.push("--no-anim".to_string());
        }

        // Add --scale if not 1.0
        if (self.export_options.scale - 1.0).abs() > 0.001 {
            args.push("--scale".to_string());
            args.push(self.export_options.scale.to_string());
        }

        let result = Command::new(&blender_exe)
            .args(&args)
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("BLENDER_EXPORT_OK") {
                        // Patch material colors if enabled
                        if let Some(ref colors_path) = colors_json_path {
                            if let Err(e) = Self::patch_material_colors(&output_path, colors_path) {
                                self.export_progress = format!("⚠️ Export succeeded but color patching failed:\n{}", e);
                                return;
                            }
                            // Cleanup colors.json file after patching
                            let _ = fs::remove_file(colors_path);
                        }

                        self.export_progress = format!(
                            "✅ Successfully exported to:\n{}",
                            output_path.display()
                        );
                    } else {
                        self.export_progress = format!(
                            "⚠️ Export completed with warnings:\n{}",
                            stdout
                        );
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    self.export_progress = format!("❌ Export failed:\n{}", stderr);
                }
            }
            Err(e) => {
                self.export_progress = format!("❌ Failed to run Blender: {}", e);
            }
        }
    }

    fn export_all(&mut self) {
        if self.blender_files.is_empty() {
            self.export_progress = "No files to export".to_string();
            return;
        }

        let blender_exe = match &self.blender_path {
            Some(path) => path.clone(),
            None => {
                self.export_progress = "Error: Blender not configured".to_string();
                return;
            }
        };

        let script_path = "D:/GitHub/monorepo-3d/tools/model-3d-editor/blender/blender-export.py";

        if !Path::new(script_path).exists() {
            self.export_progress = format!("Error: Export script not found at {}", script_path);
            return;
        }

        let total = self.blender_files.len();
        let mut success = 0;
        let mut failed = 0;

        for (i, file) in self.blender_files.iter().enumerate() {
            let blend_path = &file.path;
            let mut output_path = blend_path.clone();
            output_path.set_extension("glb");

            self.export_progress = format!(
                "Exporting {}/{}: {}",
                i + 1,
                total,
                blend_path.file_name().unwrap().to_str().unwrap()
            );

            // Build arguments with export options
            let mut args = vec![
                "--background".to_string(),
                "--factory-startup".to_string(),
                "--python".to_string(),
                script_path.to_string(),
                "--".to_string(),
                blend_path.to_str().unwrap().to_string(),
                output_path.to_str().unwrap().to_string(),
                "--fps".to_string(),
                self.export_options.fps.to_string(),
                "--up-axis".to_string(),
                "Y".to_string(),
            ];

            // Track colors.json file for cleanup
            let colors_json_path = if self.export_options.export_materials {
                let path = format!("{}.colors.json", output_path.to_str().unwrap());
                args.push("--colors".to_string());
                args.push(path.clone());
                Some(PathBuf::from(path))
            } else {
                None
            };

            if !self.export_options.export_animations {
                args.push("--no-anim".to_string());
            }

            if (self.export_options.scale - 1.0).abs() > 0.001 {
                args.push("--scale".to_string());
                args.push(self.export_options.scale.to_string());
            }

            let result = Command::new(&blender_exe)
                .args(&args)
                .output();

            match result {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("BLENDER_EXPORT_OK") {
                        // Patch material colors if enabled
                        let mut patch_ok = true;
                        if let Some(ref colors_path) = colors_json_path {
                            if let Err(_) = Self::patch_material_colors(&output_path, colors_path) {
                                patch_ok = false;
                            }
                            // Cleanup colors.json file after patching
                            let _ = fs::remove_file(colors_path);
                        }

                        if patch_ok {
                            success += 1;
                        } else {
                            failed += 1;
                        }
                    } else {
                        failed += 1;
                    }
                }
                _ => {
                    failed += 1;
                }
            }
        }

        self.export_progress = format!(
            "✅ Batch export completed!\nSuccess: {}\nFailed: {}",
            success, failed
        );
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Settings");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Blender Executable Path:");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.blender_path_input);
                if ui.button("📁 Browse...").clicked() {
                    self.browse_blender_exe();
                }
            });

            ui.add_space(5.0);

            if !self.blender_path_input.is_empty() {
                let path = PathBuf::from(&self.blender_path_input);
                if path.exists() {
                    ui.colored_label(egui::Color32::GREEN, "✓ File exists");
                    if ui.button("💾 Save Path").clicked() {
                        self.save_blender_path(path);
                        self.export_progress = "Blender path saved successfully!".to_string();
                    }
                } else {
                    ui.colored_label(egui::Color32::RED, "✗ File not found");
                }
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Current Status:");
            if let Some(ref path) = self.blender_path {
                ui.colored_label(egui::Color32::GREEN, "✓ Blender configured");
                ui.label(format!("Path: {}", path.display()));
            } else {
                ui.colored_label(egui::Color32::YELLOW, "⚠ Blender not configured");
                ui.label("Please select blender.exe to enable export functionality");
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Configuration File:");
            ui.label(format!("{}", self.config_file.display()));
            ui.label("Your Blender path is saved here for future sessions");
        });

        ui.add_space(20.0);

        if ui.button("← Back").clicked() {
            self.show_settings = false;
        }
    }

    fn render_export_options(&mut self, ui: &mut egui::Ui) {
        ui.heading("📤 Export Options");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Animation Settings:");
            ui.add_space(5.0);

            ui.checkbox(&mut self.export_options.export_animations, "Export Animations");
            ui.label("If disabled, exports static model only (--no-anim)");
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Material Settings:");
            ui.add_space(5.0);

            ui.checkbox(&mut self.export_options.export_materials, "Export Material Colors");
            ui.label("Exports diffuse colors, metallic, and roughness values");
            ui.label("This fixes white/colorless models in the GLB output");
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Advanced Settings:");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("FPS:");
                ui.add(egui::DragValue::new(&mut self.export_options.fps).speed(1.0).range(1..=120));
                ui.label("(sampling rate for animations)");
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Scale:");
                ui.add(egui::DragValue::new(&mut self.export_options.scale).speed(0.1).range(0.01..=100.0));
                ui.label("(unit normalization multiplier)");
            });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Current Settings Summary:");
            ui.add_space(5.0);

            ui.label(format!("• Animations: {}", if self.export_options.export_animations { "Enabled" } else { "Disabled (--no-anim)" }));
            ui.label(format!("• Material Colors: {}", if self.export_options.export_materials { "Enabled (--colors)" } else { "Disabled" }));
            ui.label(format!("• FPS: {}", self.export_options.fps));
            ui.label(format!("• Scale: {:.2}", self.export_options.scale));
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.show_export_options = false;
            }

            if ui.button("🔄 Reset to Defaults").clicked() {
                self.export_options = ExportOptions::default();
            }
        });
    }
}

impl eframe::App for BlenderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_settings {
                self.render_settings(ui);
                return;
            }

            if self.show_export_options {
                self.render_export_options(ui);
                return;
            }

            ui.horizontal(|ui| {
                ui.heading("Blender File Manager & Exporter");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙️ Settings").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("📤 Export Options").clicked() {
                        self.show_export_options = true;
                    }
                });
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Blender:");
                if let Some(ref path) = self.blender_path {
                    ui.colored_label(egui::Color32::GREEN, "✓ Configured");
                    ui.label(format!("({})", path.file_name().unwrap().to_str().unwrap()));
                } else {
                    ui.colored_label(egui::Color32::YELLOW, "⚠ Not configured");
                    if ui.small_button("Configure →").clicked() {
                        self.show_settings = true;
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("📁 Select Folder").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.scan_folder(folder);
                    }
                }

                if let Some(ref folder) = self.selected_folder {
                    ui.label(format!("Selected: {}", folder.display()));
                }
            });

            ui.add_space(10.0);

            if !self.blender_files.is_empty() && self.blender_path.is_some() {
                ui.horizontal(|ui| {
                    if ui.button("🚀 Export All to GLB").clicked() {
                        self.export_all();
                    }

                    if let Some(selected) = self.selected_file {
                        if ui.button("📤 Export Selected").clicked() {
                            self.export_to_glb(selected);
                        }
                    }
                });
                ui.add_space(5.0);
            }

            if !self.export_progress.is_empty() {
                ui.group(|ui| {
                    ui.label(&self.export_progress);
                });
                ui.add_space(5.0);
            }

            ui.separator();
            ui.add_space(10.0);

            if self.is_scanning {
                ui.label("🔄 Scanning...");
            } else if !self.blender_files.is_empty() {
                ui.label(format!("Found {} Blender file(s)", self.blender_files.len()));
                ui.add_space(10.0);

                let mut export_requested = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("blender_files_grid")
                        .striped(true)
                        .num_columns(3)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.strong("File Path");
                            ui.strong("Size");
                            ui.strong("Action");
                            ui.end_row();

                            for (i, file) in self.blender_files.iter().enumerate() {
                                let is_selected = self.selected_file == Some(i);
                                let response =
                                    ui.selectable_label(is_selected, file.path.display().to_string());
                                if response.clicked() {
                                    self.selected_file = Some(i);
                                }

                                ui.label(format_size(file.size));

                                if self.blender_path.is_some() {
                                    if ui.small_button("Export →").clicked() {
                                        export_requested = Some(i);
                                    }
                                } else {
                                    ui.label("—");
                                }

                                ui.end_row();
                            }
                        });
                });

                // Handle export after iteration completes
                if let Some(index) = export_requested {
                    self.export_to_glb(index);
                }
            } else if self.selected_folder.is_some() {
                ui.label("No Blender files found in the selected folder.");
            }
        });
    }
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
