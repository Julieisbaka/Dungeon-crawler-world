use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::player::Player;
use egui::{ColorImage, ComboBox, Context, TextureHandle, Ui, Vec2};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use image::{GenericImageView, ImageReader};
use serde_json::Value;

// Public state to be held by the caller. Not integrated into the app here.
#[derive(Default)]
pub struct SkillsState {
    catalog: Vec<SkillMeta>,
    selected: Option<usize>,
    loaded: bool,
    /// When true, show all discovered skills as 'owned' for previewing
    show_all: bool,
    /// When true, show a dev-only Show All toggle
    dev_controls: bool,
    /// When true, hide non-owned skills from the grid
    only_owned: bool,
    /// Markdown render cache for images/assets
    md_cache: CommonMarkCache,
}

impl SkillsState {
    /// Enables preview mode, showing all discovered skills regardless of ownership.
    pub fn enable_preview(&mut self) {
        (*self).show_all = true;
    }

    /// Enables developer controls, exposing the Show All toggle button in the UI.
    pub fn enable_dev_controls(&mut self) {
        (*self).dev_controls = true;
    }
}

#[derive(Clone)]
struct SkillMeta {
    name: String,
    description: String,
    dir: PathBuf,
    icon: Option<TextureHandle>,
}

/// Loads an icon texture from disk and registers it with egui.
///
/// # Arguments
/// * `ctx` - The egui context for loading the texture.
/// * `key` - A unique key for the texture.
/// * `icon_path` - The file path to the icon image.
///
/// # Returns
/// * `Option<TextureHandle>` - The loaded texture handle, or None if loading fails.
fn load_icon_texture(ctx: &Context, key: &str, icon_path: &Path) -> Option<TextureHandle> {
    let reader: ImageReader<std::io::BufReader<fs::File>> = ImageReader::open(icon_path).ok()?;
    let img: image::DynamicImage = reader.decode().ok()?;
    let size: (u32, u32) = (&img).dimensions();
    let rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = (&img).to_rgba8();
    let pixels: image::FlatSamples<&[u8]> = (&rgba).as_flat_samples();
    let color_image: ColorImage = ColorImage::from_rgba_unmultiplied(
        [size.0 as usize, size.1 as usize],
        (&pixels).as_slice(),
    );
    Some(ctx.load_texture(
        key.to_string(),
        color_image,
        egui::TextureOptions::default(),
    ))
}

/// Attempts to find the root directory containing the Skills folder.
///
/// # Returns
/// * `Option<PathBuf>` - The path to the Skills root, or None if not found.
fn find_skills_root() -> Option<PathBuf> {
    // Try current working directory first
    if let Ok(cwd) = std::env::current_dir() {
        let p: PathBuf = (&*cwd).join("Skills");
        if (&*p).is_dir() {
            return Some(p);
        }
    }
    // Try relative to the executable (walk up a few parents)
    if let Ok(exe) = std::env::current_exe() {
        let mut dir_opt: Option<PathBuf> = (&*exe)
            .parent()
            .map(|p: &Path| -> PathBuf { p.to_path_buf() });
        for _ in 0..4 {
            if let Some(dir) = (&dir_opt).clone() {
                let candidate: PathBuf = (&*dir).join("Skills");
                if (&*candidate).is_dir() {
                    return Some(candidate);
                }
                dir_opt = (&*dir)
                    .parent()
                    .map(|p: &Path| -> PathBuf { p.to_path_buf() });
            }
        }
    }
    None
}

/// Discovers all available skills by scanning the Skills directory.
///
/// # Arguments
/// * `ctx` - The egui context for loading textures.
///
/// # Returns
/// * `Vec<SkillMeta>` - A vector of discovered skill metadata.
fn discover_skills(ctx: &Context) -> Vec<SkillMeta> {
    let mut skills: Vec<SkillMeta> = Vec::new();
    let Some(skills_root) = find_skills_root() else {
        return skills;
    };
    if let Ok(entries) = fs::read_dir(skills_root) {
        for entry in entries.flatten() {
            let dir_path: PathBuf = (&entry).path();
            if !(&*dir_path).is_dir() {
                continue;
            }
            // Try to read optional metadata JSON; fallback to directory name and description.md
            let mut name: String = (&*dir_path)
                .file_name()
                .and_then(|s: &std::ffi::OsStr| -> Option<&str> { s.to_str() })
                .unwrap_or("")
                .to_string();
            let mut description: String = String::new();
            // Try to find a metadata json in the directory
            if let Ok(files) = fs::read_dir(&dir_path) {
                for f in files.flatten() {
                    let p: PathBuf = (&f).path();
                    if (&*p).is_file()
                        && (&*p)
                            .extension()
                            .and_then(|e: &std::ffi::OsStr| -> Option<&str> { e.to_str() })
                            .map(|e: &str| -> bool { e.eq_ignore_ascii_case("json") })
                            .unwrap_or(false)
                    {
                        if let Ok(content) = fs::read_to_string(&p) {
                            if let Ok(val) = serde_json::from_str::<Value>(&**&content) {
                                if let Some(n) = (&val)
                                    .get("name")
                                    .and_then(|v: &Value| -> Option<&str> { v.as_str() })
                                {
                                    name = n.to_string();
                                }
                                if let Some(desc) = (&val)
                                    .get("description")
                                    .and_then(|v: &Value| -> Option<&str> { v.as_str() })
                                {
                                    description = desc.to_string();
                                }
                            }
                        }
                        break;
                    }
                }
            }
            // If description points to a markdown file, load it; otherwise try description.md/Description.md as a default
            let mut loaded_md: bool = false;
            if (&*(&*description).trim().to_lowercase()).ends_with(".md") {
                // Only use the file name, not a path, to avoid double Skills/Skills/
                let desc_file: &std::ffi::OsStr =
                    Path::new(&description).file_name().unwrap_or_default();
                let md_path: PathBuf = (&*dir_path).join(desc_file);
                match fs::read_to_string(&md_path) {
                    Ok(md) => {
                        description = md;
                        loaded_md = true;
                    }
                    Err(e) => {
                        eprintln!(
                            "[Skills] Failed to load markdown file for skill '{}': {} (path: {:?})",
                            name, e, md_path
                        );
                    }
                }
            }
            if !loaded_md && (&*description).trim().is_empty() {
                // Try both 'description.md' and 'Description.md' (case-insensitive)
                let candidates: [&str; 2] = ["description.md", "Description.md"];
                for cand in &candidates {
                    let md_path: PathBuf = (&*dir_path).join(cand);
                    match fs::read_to_string(&md_path) {
                        Ok(md) => {
                            description = md;
                            loaded_md = true;
                            break;
                        }
                        Err(e) => {
                            eprintln!("[Skills] Failed to load default {} for skill '{}': {} (path: {:?})", cand, name, e, md_path);
                        }
                    }
                }
            }
            if !loaded_md
                && ((&*description).trim().ends_with(".md") || (&*description).trim().is_empty())
            {
                description = "No description available.".to_string();
            }
            // Attempt icon load (icon.png/jpg/jpeg)
            let mut icon_handle: Option<TextureHandle> = None;
            for candidate in ["icon.png", "icon.jpg", "icon.jpeg"] {
                let ip: PathBuf = (&*dir_path).join(candidate);
                if (&*ip).exists() {
                    icon_handle =
                        load_icon_texture(ctx, &**&format!("skill_icon_{}", name), &**&ip);
                    if (&icon_handle).is_some() {
                        break;
                    }
                }
            }
            (&mut skills).push(SkillMeta {
                name,
                description,
                dir: (&dir_path).clone(),
                icon: icon_handle,
            });
        }
    }
    (&mut *skills).sort_by(|a: &SkillMeta, b: &SkillMeta| -> std::cmp::Ordering {
        (&(&*(*a).name).to_lowercase()).cmp(&(&*(*b).name).to_lowercase())
    });
    skills
}

/// Reads the player's owned skills from disk.
///
/// # Returns
/// * `HashMap<String, i8>` - A map of skill names to their levels.
fn read_player_skills() -> HashMap<String, i8> {
    // Attempt to read current save context (if available)
    let current: Option<String> = (&*crate::CURRENT_SAVE).lock().ok().and_then(
        |g: std::sync::MutexGuard<'_, Option<String>>| -> Option<String> { (&*g).clone() },
    );
    let Some(save) = current else {
        return HashMap::new();
    };
    let path: PathBuf = (&*Path::new("saves").join(save)).join("player.json");
    let Ok(content) = fs::read_to_string(path) else {
        return HashMap::new();
    };
    let Ok(player) = serde_json::from_str::<Player>(&**&content) else {
        return HashMap::new();
    };
    player.skills
}

// Public UI entry: renders a gallery of all skills; unknown skills show no name/icon/description.
// Clicking an owned skill opens a details panel with its description and current level.
/// Renders the skills UI, showing the skill grid and details.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `state` - The mutable state for the skills UI.
pub fn skills_ui(ui: &mut Ui, state: &mut SkillsState) {
    // Lazy-load catalog when first shown
    if !(*state).loaded {
        let ctx: Context = ui.ctx().clone();
        (*state).catalog = discover_skills(&ctx);
        (*state).loaded = true;
    }

    let player_skills: HashMap<String, i8> = read_player_skills();

    // --- Gallery Controls ---
    static mut SEARCH: Option<String> = None;
    static mut SORT_MODE: u8 = 0; // 0: Name A-Z, 1: Name Z-A, 2: Level High-Low, 3: Level Low-High
    static mut PAGE: usize = 0;
    const PAGE_SIZE: usize = 12;

    // Controls: Search, Sort, Pagination
    ui.horizontal(|ui: &mut Ui| {
        let mut search: String = unsafe { (&SEARCH).clone().unwrap_or_default() };
        ui.label("Search:");
        if (&ui.text_edit_singleline(&mut search)).changed() {
            unsafe {
                SEARCH = Some((&search).clone());
                PAGE = 0;
            }
        }
        ui.separator();
        let mut sort_mode: u8 = unsafe { SORT_MODE };
        ComboBox::from_id_salt("skills_sort_mode")
            .selected_text(match sort_mode {
                0 => "Name (A-Z)",
                1 => "Name (Z-A)",
                2 => "Level (High-Low)",
                3 => "Level (Low-High)",
                _ => "Name (A-Z)",
            })
            .show_ui(ui, |ui: &mut Ui| {
                ui.selectable_value(&mut sort_mode, 0, "Name (A-Z)");
                ui.selectable_value(&mut sort_mode, 1, "Name (Z-A)");
                ui.selectable_value(&mut sort_mode, 2, "Level (High-Low)");
                ui.selectable_value(&mut sort_mode, 3, "Level (Low-High)");
            });
        if sort_mode != unsafe { SORT_MODE } {
            unsafe {
                SORT_MODE = sort_mode;
                PAGE = 0;
            }
        }
        ui.separator();
        let mut page: usize = unsafe { PAGE };
        if page > 0 && (&ui.button("< Prev")).clicked() {
            page -= 1;
            unsafe {
                PAGE = page;
            }
        }
        ui.label(format!("Page {}", page + 1));
        let filtered_count: usize = (&*(*state).catalog)
            .iter()
            .filter(|meta: &&SkillMeta| {
                let owned_real: bool = (&player_skills).contains_key(&(**meta).name);
                let dev_show_all_active: bool =
                    cfg!(feature = "dev-mode") && (*state).dev_controls && (*state).show_all;
                let treated_owned: bool = dev_show_all_active || owned_real;
                (!(*state).only_owned || treated_owned)
                    && ((&search).is_empty()
                        || (&*(&*(**meta).name).to_lowercase())
                            .contains(&(&*search).to_lowercase()))
            })
            .count();
        if (&((page + 1) * PAGE_SIZE)) < &filtered_count && (&ui.button("Next >")).clicked() {
            page += 1;
            unsafe {
                PAGE = page;
            }
        }
    });
    ui.add_space(6.0);

    // --- Gallery Grid ---
    let search: String = unsafe { (&SEARCH).clone().unwrap_or_default() };
    let sort_mode: u8 = unsafe { SORT_MODE };
    let page: usize = unsafe { PAGE };
    let mut filtered: Vec<_> = (&*(*state).catalog)
        .iter()
        .enumerate()
        .filter(|(_, meta)| {
            let owned_real: bool = (&player_skills).contains_key(&(**meta).name);
            let dev_show_all_active: bool =
                cfg!(feature = "dev-mode") && (*state).dev_controls && (*state).show_all;
            let treated_owned: bool = dev_show_all_active || owned_real;
            (!(*state).only_owned || treated_owned)
                && ((&search).is_empty()
                    || (&*(&*(**meta).name).to_lowercase()).contains(&(&*search).to_lowercase()))
        })
        .collect();
    match sort_mode {
        0 => (&mut *filtered).sort_by(
            |a: &(usize, &SkillMeta), b: &(usize, &SkillMeta)| -> std::cmp::Ordering {
                (&(&*(*(*a).1).name).to_lowercase()).cmp(&(&*(*(*b).1).name).to_lowercase())
            },
        ),
        1 => (&mut *filtered).sort_by(
            |a: &(usize, &SkillMeta), b: &(usize, &SkillMeta)| -> std::cmp::Ordering {
                (&(&*(*(*b).1).name).to_lowercase()).cmp(&(&*(*(*a).1).name).to_lowercase())
            },
        ),
        2 => (&mut *filtered).sort_by(
            |a: &(usize, &SkillMeta), b: &(usize, &SkillMeta)| -> std::cmp::Ordering {
                let la: i8 = (&player_skills).get(&(*(*a).1).name).copied().unwrap_or(0);
                let lb: i8 = (&player_skills).get(&(*(*b).1).name).copied().unwrap_or(0);
                (&lb).cmp(&la).then_with(|| -> std::cmp::Ordering {
                    (&(&*(*(*a).1).name).to_lowercase()).cmp(&(&*(*(*b).1).name).to_lowercase())
                })
            },
        ),
        3 => (&mut *filtered).sort_by(
            |a: &(usize, &SkillMeta), b: &(usize, &SkillMeta)| -> std::cmp::Ordering {
                let la: i8 = (&player_skills).get(&(*(*a).1).name).copied().unwrap_or(0);
                let lb: i8 = (&player_skills).get(&(*(*b).1).name).copied().unwrap_or(0);
                (&la).cmp(&lb).then_with(|| -> std::cmp::Ordering {
                    (&(&*(*(*a).1).name).to_lowercase()).cmp(&(&*(*(*b).1).name).to_lowercase())
                })
            },
        ),
        _ => {}
    }
    let total_pages: usize = ((&filtered).len() + PAGE_SIZE - 1) / PAGE_SIZE;
    let start: usize = page * PAGE_SIZE;
    let end: usize = ((page + 1) * PAGE_SIZE).min((&filtered).len());
    let page_items: &[(usize, &SkillMeta)] =
        &(&filtered)[start.min((&filtered).len())..end.min((&filtered).len())];

    if page_items.is_empty() {
        ui.label("No skills found. Try adjusting your search or filters.");
        return;
    }

    let columns: usize = 4;
    egui::Grid::new("skills_gallery_grid")
        .spacing(Vec2::splat(12.0))
        .show(ui, |ui: &mut Ui| {
            for (i, (idx, meta)) in page_items.iter().enumerate() {
                let owned_real: bool = (&player_skills).contains_key(&(**meta).name);
                let dev_show_all_active: bool =
                    cfg!(feature = "dev-mode") && (*state).dev_controls && (*state).show_all;
                let treated_owned: bool = dev_show_all_active || owned_real;
                let mut frame: egui::Frame =
                    egui::Frame::group(&**ui.style()).inner_margin(egui::Margin::symmetric(8, 8));
                if !treated_owned {
                    frame = frame.fill(egui::Color32::from_gray(30));
                }
                frame.show(ui, |ui: &mut Ui| {
                    ui.set_min_size(Vec2::new(140.0, 140.0));
                    ui.vertical_centered(|ui: &mut Ui| {
                        if treated_owned {
                            if let Some(tex) = &(**meta).icon {
                                ui.add(egui::Image::new(tex).fit_to_exact_size(Vec2::splat(72.0)));
                                ui.add_space(6.0);
                            }
                            ui.label(&(**meta).name);
                            if (&ui.button("View")).clicked() {
                                (*state).selected = Some(*idx);
                            }
                        } else {
                            ui.add_space(48.0);
                        }
                    });
                });
                if &((i + 1) % columns) == &0 {
                    ui.end_row();
                }
            }
        });

    // Detail view in a floating window for owned skills or all in preview
    if let Some(idx) = (*state).selected {
        if let Some(meta) = (&*(*state).catalog).get(idx) {
            let dev_show_all_active: bool =
                cfg!(feature = "dev-mode") && (*state).dev_controls && (*state).show_all;
            if dev_show_all_active || (&player_skills).contains_key(&(*meta).name) {
                let level: i8 = (&player_skills).get(&(*meta).name).copied().unwrap_or(0);
                let mut open: bool = true;
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
                        if !(&*(*meta).description).trim().is_empty() {
                            // Configure markdown rendering with local file base for images
                            // Set tooltip for links
                            (*ui.style_mut()).url_in_tooltip = true;
                            // Build a file:// base for relative images using the skill directory
                            let base_uri: String = {
                                let abs: PathBuf = (&*(*meta).dir)
                                    .canonicalize()
                                    .unwrap_or((&(*meta).dir).clone());
                                let s: String = (&*(&*abs).to_string_lossy()).replace('\\', "/");
                                format!("file:///{}/", s.trim_start_matches('/'))
                            };
                            let viewer: CommonMarkViewer = CommonMarkViewer::new()
                                .default_width(Some(700))
                                .max_image_width(Some(600))
                                .explicit_image_uri_scheme(false)
                                .default_implicit_uri_scheme(base_uri);

                            let _resp: egui::InnerResponse<()> = viewer.show(
                                ui,
                                &mut (*state).md_cache,
                                (&(*meta).description).as_str(),
                            );
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
