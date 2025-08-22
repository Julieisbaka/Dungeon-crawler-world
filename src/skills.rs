use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use egui::{ColorImage, Context, TextureHandle, Ui, Vec2};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use image::{GenericImageView, ImageReader};
use serde_json::Value;

// Public state to be held by the caller. Not integrated into the app here.
#[derive(Default)]
pub struct SkillsState {
	catalog: Vec<SkillMeta>,
	selected: Option<usize>,
	loaded: bool,
	// When true, show all discovered skills as 'owned' for previewing
	show_all: bool,
	// When true, show a dev-only Show All toggle
	dev_controls: bool,
	// When true, hide non-owned skills from the grid
	only_owned: bool,
	// Markdown render cache for images/assets
	md_cache: CommonMarkCache,
}

impl SkillsState {
	// Enable preview mode to show all discovered skills regardless of ownership
	pub fn enable_preview(&mut self) { (*self).show_all = true; }

	// Enable developer controls (expose Show All toggle button)
	pub fn enable_dev_controls(&mut self) { (*self).dev_controls = true; }
}

#[derive(Clone)]
struct SkillMeta {
	name: String,
	description: String,
	dir: PathBuf,
	icon: Option<TextureHandle>,
}

fn load_icon_texture(ctx: &Context, key: &str, icon_path: &Path) -> Option<TextureHandle> {
	let reader: ImageReader<std::io::BufReader<fs::File>> = ImageReader::open(icon_path).ok()?;
	let img: image::DynamicImage = reader.decode().ok()?;
	let size = img.dimensions();
	let rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
	let pixels: image::FlatSamples<&[u8]> = rgba.as_flat_samples();
	let color_image: ColorImage = ColorImage::from_rgba_unmultiplied(
		[size.0 as usize, size.1 as usize],
		pixels.as_slice(),
	);
	Some(ctx.load_texture(key.to_string(), color_image, egui::TextureOptions::default()))
}

fn find_skills_root() -> Option<PathBuf> {
	// Try current working directory first
	if let Ok(cwd) = std::env::current_dir() {
		let p: PathBuf = cwd.join("Skills");
		if p.is_dir() { return Some(p); }
	}
	// Try relative to the executable (walk up a few parents)
	if let Ok(exe) = std::env::current_exe() {
		let mut dir_opt: Option<PathBuf> = exe.parent().map(|p: &Path| -> PathBuf { p.to_path_buf() });
		for _ in 0..4 {
			if let Some(dir) = dir_opt.clone() {
				let candidate: PathBuf = dir.join("Skills");
				if candidate.is_dir() { return Some(candidate); }
				dir_opt = dir.parent().map(|p: &Path| -> PathBuf { p.to_path_buf() });
			}
		}
	}
	None
}

fn discover_skills(ctx: &Context) -> Vec<SkillMeta> {
	let mut skills: Vec<SkillMeta> = Vec::new();
	let Some(skills_root) = find_skills_root() else { return skills; };
	if let Ok(entries) = fs::read_dir(skills_root) {
		for entry in entries.flatten() {
			let dir_path: PathBuf = entry.path();
			if !dir_path.is_dir() {
				continue;
			}
			// Try to read optional metadata JSON; fallback to directory name and description.md
			let mut name: String = dir_path
				.file_name()
				.and_then(|s: &std::ffi::OsStr| -> Option<&str> { s.to_str() })
				.unwrap_or("")
				.to_string();
			let mut description: String = String::new();
			// Try to find a metadata json in the directory
			if let Ok(files) = fs::read_dir(&dir_path) {
				for f in files.flatten() {
					let p: PathBuf = f.path();
					if p.is_file()
						&& p.extension()
							.and_then(|e: &std::ffi::OsStr| -> Option<&str> { e.to_str() })
							.map(|e| e.eq_ignore_ascii_case("json"))
							.unwrap_or(false)
					{
						if let Ok(content) = fs::read_to_string(&p) {
							if let Ok(val) = serde_json::from_str::<Value>(&content) {
								if let Some(n) = val.get("name").and_then(|v: &Value| -> Option<&str> { v.as_str() }) {
									name = n.to_string();
								}
								if let Some(desc) = val.get("description").and_then(|v: &Value| -> Option<&str> { v.as_str() }) {
									description = desc.to_string();
								}
							}
						}
						break;
					}
				}
			}
			// If description points to a markdown file, load it; otherwise try description.md as a default
			if description.ends_with(".md") {
				let md_path: PathBuf = dir_path.join(&description);
				if let Ok(md) = fs::read_to_string(&md_path) { description = md; }
			} else if description.is_empty() {
				let md_path: PathBuf = dir_path.join("description.md");
				if let Ok(md) = fs::read_to_string(&md_path) { description = md; }
			}
			// Attempt icon load (icon.png/jpg/jpeg)
			let mut icon_handle: Option<TextureHandle> = None;
			for candidate in ["icon.png", "icon.jpg", "icon.jpeg"] {
				let ip: PathBuf = dir_path.join(candidate);
				if ip.exists() {
					icon_handle = load_icon_texture(ctx, &format!("skill_icon_{}", name), &ip);
					if icon_handle.is_some() {
						break;
					}
				}
			}
			skills.push(SkillMeta {
				name,
				description,
				dir: dir_path.clone(),
				icon: icon_handle,
			});
		}
	}
	skills.sort_by(|a: &SkillMeta, b: &SkillMeta| -> std::cmp::Ordering { (*a).name.to_lowercase().cmp(&(*b).name.to_lowercase()) });
	skills
}

fn read_player_skills() -> HashMap<String, i32> {
	let mut map: HashMap<String, i32> = HashMap::new();
	// Attempt to read current save context (if available)
	let current: Option<String> = crate::CURRENT_SAVE.lock().ok().and_then(|g: std::sync::MutexGuard<'_, Option<String>>| -> Option<String> { g.clone() });
	let Some(save) = current else { return map };
	let path: PathBuf = Path::new("saves").join(save).join("player.json");
	let Ok(content) = fs::read_to_string(path) else { return map };
	let Ok(json) = serde_json::from_str::<Value>(&content) else { return map };
	if let Some(skills) = json.get("skills").and_then(|v: &Value| -> Option<&serde_json::Map<String, Value>> { v.as_object() }) {
		for (name, val) in skills.iter() {
			if let Some(lvl) = val.as_i64() {
				map.insert(name.clone(), lvl as i32);
			}
		}
	}
	map
}

// Public UI entry: renders a gallery of all skills; unknown skills show no name/icon/description.
// Clicking an owned skill opens a details panel with its description and current level.
pub fn skills_ui(ui: &mut Ui, state: &mut SkillsState) {
	// Lazy-load catalog when first shown
	if !(*state).loaded {
		let ctx: Context = ui.ctx().clone();
		(*state).catalog = discover_skills(&ctx);
		(*state).loaded = true;
	}

	let player_skills: HashMap<String, i32> = read_player_skills();

	// Toolbar: Reload, Only Owned filter, and optional dev Show All toggle
	ui.horizontal(|ui: &mut Ui| {
		if ui.button("Reload").clicked() {
			let ctx: Context = ui.ctx().clone();
			(*state).catalog = discover_skills(&ctx);
			(*state).loaded = true;
			(*state).selected = None;
		}
		ui.separator();
		ui.checkbox(&mut (*state).only_owned, "Only owned");
		// Show the Show All toggle only when compiled with dev-mode AND when the caller enabled dev controls
		if cfg!(feature = "dev-mode") && (*state).dev_controls {
			ui.separator();
			let label = if (*state).show_all { "Show All (Dev): On" } else { "Show All (Dev): Off" };
			if ui.toggle_value(&mut (*state).show_all, label).clicked() {
				// Clear any selection when toggling preview mode
				(*state).selected = None;
			}
		}
		let owned_count = (*state)
			.catalog
			.iter()
			.filter(|m: &&SkillMeta| player_skills.contains_key(&(**m).name))
			.count();
		ui.label(format!("Owned: {}  Total: {}", owned_count, (*state).catalog.len()));
	});
	ui.add_space(6.0);

	if (*state).catalog.is_empty() {
		ui.label("No skills found. Ensure the 'Skills' folder is located next to the executable or project root.");
		return;
	}

	egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui: &mut Ui| {
		// Use full available width for the gallery
		ui.set_min_width(ui.available_width());
		ui.horizontal_wrapped(|ui: &mut Ui| {
			let tile_size: Vec2 = Vec2::new(140.0, 140.0);
			let spacing = (*ui.spacing()).item_spacing.x;
			for (idx, meta) in (*state).catalog.iter().enumerate() {
				let owned_real: bool = player_skills.contains_key(&(*meta).name);
				if (*state).only_owned && !owned_real { continue; }
				// Only let Show All affect behavior when dev-mode is compiled in and dev controls are enabled
				let dev_show_all_active: bool = cfg!(feature = "dev-mode") && (*state).dev_controls && (*state).show_all;
				let treated_owned: bool = dev_show_all_active || owned_real;
				let mut frame: egui::Frame = egui::Frame::group(ui.style()).inner_margin(egui::Margin::symmetric(8, 8));
				// Gray-out unknown (not owned and not in show_all)
				if !treated_owned {
					frame = frame.fill(egui::Color32::from_gray(30));
				}
				frame.show(ui, |ui: &mut Ui| {
					ui.set_min_size(tile_size);
					ui.vertical_centered(|ui: &mut Ui| {
						if treated_owned {
							if let Some(tex) = &(*meta).icon {
								ui.add(egui::Image::new(tex).fit_to_exact_size(Vec2::splat(72.0)));
								ui.add_space(6.0);
							}
							ui.label(&(*meta).name);
							if ui.button("View").clicked() { (*state).selected = Some(idx); }
						} else {
							// Unknown: show a filled gray tile without name/icon/description
							ui.add_space(48.0);
						}
					});
				});
				ui.add_space(spacing);
			}
		});
	});

	// Detail view in a floating window for owned skills or all in preview
	if let Some(idx) = (*state).selected {
		if let Some(meta) = (*state).catalog.get(idx) {
			let dev_show_all_active: bool = cfg!(feature = "dev-mode") && (*state).dev_controls && (*state).show_all;
			if dev_show_all_active || player_skills.contains_key(&(*meta).name) {
				let level = player_skills.get(&(*meta).name).copied().unwrap_or(0);
				let mut open = true;
				egui::Window::new(format!("{}", meta.name))
					.open(&mut open)
					.collapsible(false)
					.resizable(true)
					.show(ui.ctx(), |ui: &mut Ui| {
						ui.horizontal(|ui: &mut Ui| {
							if let Some(tex) = &(*meta).icon {
								ui.add(egui::Image::new(tex).fit_to_exact_size(Vec2::splat(64.0)));
							}
							ui.vertical(|ui: &mut Ui| {
								ui.label(format!("Level: {}", level));
							});
						});
						ui.add_space(8.0);
						if !(*meta).description.trim().is_empty() {
							// Configure markdown rendering with local file base for images
							// Set tooltip for links
							(*ui.style_mut()).url_in_tooltip = true;
							// Build a file:// base for relative images using the skill directory
							let base_uri: String = {
								let abs: PathBuf = (*meta).dir.canonicalize().unwrap_or((*meta).dir.clone());
								let s: String = abs.to_string_lossy().replace('\\', "/");
								format!("file:///{}/", s.trim_start_matches('/'))
							};
							let viewer: CommonMarkViewer = CommonMarkViewer::new()
								.default_width(Some(700))
								.max_image_width(Some(600))
								.explicit_image_uri_scheme(false)
								.default_implicit_uri_scheme(base_uri);

							let _resp: egui::InnerResponse<()> = viewer.show(ui, &mut (*state).md_cache, (*meta).description.as_str());
						} else {
							ui.label("No description available.");
						}
					});
				if !open {
					(*state).selected = None;
				}
			} else {
				// If selection points to an unknown skill, clear it
				(*state).selected = None;
			}
		}
	}
}

