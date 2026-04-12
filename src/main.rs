#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};
use std::sync::Arc;

// Import necessary crates and modules
mod logger;
use egui::{Context, Style, ViewportId, Visuals};
use logger::init_logger;

// Import saves and settings from library modules
use dungeon_crawler_world::console::{
    console_ui, read_player_skills_from_path, ConsoleCommandContext, ConsoleRegistry, ConsoleState,
};
use dungeon_crawler_world::logic::settings_logic::{LogVerbosity, PowerPreference, Settings};
use dungeon_crawler_world::logic::skills_logic::read_player_skills_for_save;
use dungeon_crawler_world::ui::main_menu::MainMenu;

mod new_save;
mod player;
mod ui_preview;
use ui_preview::UiPreviewManager;
mod fps;
use fps::FpsGraph;

use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

/// Developer mode flag is controlled via Cargo feature `dev-mode`.
/// Enabled in debug builds by default via `Cargo.toml` [features].
/// For release builds in CI, we pass `--no-default-features` to disable it.
const DEV_MODE_ENABLED: bool = cfg!(feature = "dev-mode");

/// Main app struct with settings state
use std::time::{Duration, Instant};

/// Main application struct holding all UI and game state.
struct DungeonCrawlerworld {
    /// Main menu UI state.
    menu: MainMenu,
    /// Current application settings.
    settings: Settings,
    /// State for the developer console.
    console_state: ConsoleState,
    /// Immutable command registry for the developer console.
    console_registry: ConsoleRegistry,
    /// Manager for UI previews (dev mode only).
    ui_preview: UiPreviewManager,
    /// Last known fullscreen state (for toggling fullscreen mode).
    last_fullscreen: Option<bool>,
    /// FPS graph data and rendering.
    fps: FpsGraph,
    /// Whether the console is currently open in this session.
    console_open: bool,
    /// Last value of the show_console setting (to detect toggles).
    last_show_console: bool,
    /// Receiver for log messages to display in the in-game console.
    log_rx: Option<std::sync::mpsc::Receiver<String>>,
    /// Last time the console was redrawn (for throttling redraws).
    last_console_redraw: Option<Instant>,
    /// Flag indicating the app should quit.
    should_quit: bool,
}

struct AppConsoleContext<'a> {
    console_state: &'a mut ConsoleState,
    settings: &'a mut Settings,
    ui_preview: &'a mut UiPreviewManager,
    saves_root: PathBuf,
}

impl ConsoleCommandContext for AppConsoleContext<'_> {
    fn clear_console(&mut self) {
        self.console_state.clear();
    }

    fn open_preview(&mut self, name: &str) -> Result<(), String> {
        self.ui_preview.open_preview(name)
    }

    fn list_previews(&self) -> Vec<String> {
        self.ui_preview.list_preview_names()
    }

    fn current_save(&self) -> Option<String> {
        dungeon_crawler_world::CURRENT_SAVE
            .lock()
            .ok()
            .and_then(|guard| guard.clone())
    }

    fn saves_root(&self) -> &Path {
        &self.saves_root
    }

    fn read_owned_skills(&self) -> Result<Vec<(String, i8)>, String> {
        let current_save = self.current_save();
        if let Some(save) = current_save.as_deref() {
            let skills = read_player_skills_for_save(self.saves_root(), save);
            if skills.is_empty() {
                return read_player_skills_from_path(self.saves_root(), current_save);
            }
            let mut skills: Vec<(String, i8)> = skills.into_iter().collect();
            skills.sort_by(|left, right| left.0.cmp(&right.0));
            return Ok(skills);
        }
        Err("No current save selected.".to_string())
    }

    fn get_setting_value(&self, key: &str) -> Result<String, String> {
        match key.to_ascii_lowercase().as_str() {
            "show_console" => Ok(self.settings.show_console.to_string()),
            "log_to_console" => Ok(self.settings.log_to_console.to_string()),
            "log_verbosity" => Ok(log_verbosity_name(self.settings.log_verbosity).to_string()),
            "console_max_lines" => Ok(self.settings.console_max_lines.to_string()),
            "show_fps_graph" => Ok(self.settings.show_fps_graph.to_string()),
            "developer_mode" => Ok(self.settings.developer_mode.to_string()),
            "target_fps" => Ok(self.settings.target_fps.to_string()),
            "vsync" => Ok(self.settings.vsync.to_string()),
            "show_fps_counter" => Ok(self.settings.show_fps_counter.to_string()),
            "power_preference" => Ok(power_preference_name(self.settings.power_preference).to_string()),
            other => Err(format!("Unknown setting: {}", other)),
        }
    }

    fn list_setting_values(&self) -> Vec<(String, String)> {
        vec![
            (
                "show_console".to_string(),
                self.settings.show_console.to_string(),
            ),
            (
                "log_to_console".to_string(),
                self.settings.log_to_console.to_string(),
            ),
            (
                "log_verbosity".to_string(),
                log_verbosity_name(self.settings.log_verbosity).to_string(),
            ),
            (
                "console_max_lines".to_string(),
                self.settings.console_max_lines.to_string(),
            ),
            (
                "show_fps_graph".to_string(),
                self.settings.show_fps_graph.to_string(),
            ),
            (
                "developer_mode".to_string(),
                self.settings.developer_mode.to_string(),
            ),
            (
                "target_fps".to_string(),
                self.settings.target_fps.to_string(),
            ),
            (
                "vsync".to_string(),
                self.settings.vsync.to_string(),
            ),
            (
                "show_fps_counter".to_string(),
                self.settings.show_fps_counter.to_string(),
            ),
            (
                "power_preference".to_string(),
                power_preference_name(self.settings.power_preference).to_string(),
            ),
        ]
    }

    fn set_setting_value(&mut self, key: &str, value: &str) -> Result<String, String> {
        let key = key.to_ascii_lowercase();
        match key.as_str() {
            "show_console" => {
                self.settings.show_console = parse_bool_setting(value)?;
            }
            "log_to_console" => {
                self.settings.log_to_console = parse_bool_setting(value)?;
            }
            "log_verbosity" => {
                self.settings.log_verbosity = parse_log_verbosity(value)?;
            }
            "console_max_lines" => {
                self.settings.console_max_lines = value
                    .parse::<usize>()
                    .map_err(|_| format!("Invalid usize value: {}", value))?;
            }
            "show_fps_graph" => {
                self.settings.show_fps_graph = parse_bool_setting(value)?;
            }
            "developer_mode" => {
                self.settings.developer_mode = parse_bool_setting(value)?;
            }
            "target_fps" => {
                self.settings.target_fps = value
                    .parse::<u32>()
                    .map_err(|_| format!("Invalid u32 value: {}", value))?;
            }
            "vsync" => {
                self.settings.vsync = parse_bool_setting(value)?;
            }
            "show_fps_counter" => {
                self.settings.show_fps_counter = parse_bool_setting(value)?;
            }
            "power_preference" => {
                self.settings.power_preference = parse_power_preference(value)?;
            }
            _ => return Err(format!("Unknown setting: {}", key)),
        }
        self.settings.save();
        Ok(format!("Set {} = {}", key, self.get_setting_value(&key)?))
    }

    fn regenerate_grid_preview(&mut self) -> Result<String, String> {
        self.ui_preview.regenerate_grid_preview()?;
        Ok("Regenerated grid preview.".to_string())
    }

    fn reset_grid_preview(&mut self) -> Result<String, String> {
        self.ui_preview.reset_grid_preview_view()?;
        Ok("Reset grid preview view.".to_string())
    }
}

fn parse_bool_setting(value: &str) -> Result<bool, String> {
    match value.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(format!("Invalid boolean value: {}", value)),
    }
}

fn parse_log_verbosity(value: &str) -> Result<LogVerbosity, String> {
    match value.to_ascii_lowercase().as_str() {
        "error" => Ok(LogVerbosity::Error),
        "warn" | "warning" => Ok(LogVerbosity::Warn),
        "info" => Ok(LogVerbosity::Info),
        "debug" => Ok(LogVerbosity::Debug),
        "trace" => Ok(LogVerbosity::Trace),
        _ => Err(format!("Invalid log verbosity: {}", value)),
    }
}

fn log_verbosity_name(value: LogVerbosity) -> &'static str {
    match value {
        LogVerbosity::Error => "error",
        LogVerbosity::Warn => "warn",
        LogVerbosity::Info => "info",
        LogVerbosity::Debug => "debug",
        LogVerbosity::Trace => "trace",
    }
}

fn power_preference_name(value: PowerPreference) -> &'static str {
    match value {
        PowerPreference::Default => "default",
        PowerPreference::LowPower => "low_power",
        PowerPreference::HighPerformance => "high_performance",
    }
}

fn parse_power_preference(value: &str) -> Result<PowerPreference, String> {
    match value.to_ascii_lowercase().as_str() {
        "default" | "0" => Ok(PowerPreference::Default),
        "low_power" | "low" | "1" => Ok(PowerPreference::LowPower),
        "high_performance" | "high" | "2" => Ok(PowerPreference::HighPerformance),
        _ => Err(format!("Invalid power preference: {}. Use default, low_power, or high_performance", value)),
    }
}

impl DungeonCrawlerworld {
    /// Creates a new default instance of the main application struct, initializing all state.
    fn new() -> Self {
        let (_log_tx, log_rx) = init_logger();
        Self {
            menu: MainMenu::new(),
            settings: Settings::default(),
            console_state: ConsoleState::default(),
            console_registry: ConsoleRegistry::new(),
            ui_preview: UiPreviewManager::new(),
            last_fullscreen: None,
            fps: FpsGraph::default(),
            console_open: false,
            last_show_console: Settings::default().show_console,
            log_rx: Some(log_rx),
            last_console_redraw: None,
            should_quit: false,
        }
    }

    /// Updates the application state and renders the UI for each frame.
    ///
    /// # Arguments
    /// * `ctx` - The egui context for UI rendering and input.
    /// * `window` - The winit window for viewport commands.
    fn update(&mut self, ctx: &Context, window: &Window) {
        // Apply fullscreen setting when it changes
        if self.last_fullscreen != Some(self.settings.fullscreen) {
            self.last_fullscreen = Some(self.settings.fullscreen);
            window.set_fullscreen(if self.settings.fullscreen {
                Some(winit::window::Fullscreen::Borderless(None))
            } else {
                None
            });
        }

        // Update FPS graph with delta time in ms
        let dt_ms: f32 = ctx.input(|i: &egui::InputState| -> f32 { i.stable_dt }) * 1000.0;
        self.fps.push_frame_time(dt_ms);

        // Render the main menu UI and check if user wants to quit
        if self.menu.show(ctx, &mut self.settings, DEV_MODE_ENABLED) {
            self.should_quit = true;
        }

        // Developer Console window: follow the setting on both rising and falling edges.
        // Only react when the setting value actually changes (edge detection), so we avoid
        // unnecessary updates every frame and preserve the prior bidirectional behavior.
        if self.settings.show_console != self.last_show_console {
            self.console_open = self.settings.show_console;
            self.last_show_console = self.settings.show_console;
        }

        // Poll logger and write to in-game console if enabled, filter by verbosity
        if self.settings.log_to_console {
            if let Some(rx) = &self.log_rx {
                // Convert verbosity to numeric threshold for efficient comparison
                // Higher number = more verbose (show more messages)
                let threshold = match self.settings.log_verbosity {
                    LogVerbosity::Error => 0,
                    LogVerbosity::Warn => 1,
                    LogVerbosity::Info => 2,
                    LogVerbosity::Debug => 3,
                    LogVerbosity::Trace => 4,
                };
                while let Ok(msg) = rx.try_recv() {
                    // Extract log level from message and convert to numeric value
                    // Lower number = higher priority (ERROR=0, TRACE=4)
                    let msg_level = if msg.starts_with("[ERROR]") {
                        0
                    } else if msg.starts_with("[WARN]") {
                        1
                    } else if msg.starts_with("[INFO]") {
                        2
                    } else if msg.starts_with("[DEBUG]") {
                        3
                    } else {
                        4 // TRACE or unrecognized
                    };
                    // Show message if its level is <= threshold
                    if msg_level <= threshold {
                        self.console_state.log_line(msg);
                    }
                }
            }
        }

        if DEV_MODE_ENABLED && self.settings.developer_mode && self.console_open {
            let mut open: bool = true;
            let now: Instant = Instant::now();
            let redraw_interval: Duration = Duration::from_millis(33); // ~30 FPS max
            let should_redraw: bool = self.console_state.is_dirty()
                && self
                    .last_console_redraw
                    .is_none_or(|last: Instant| -> bool {
                        now.duration_since(last) >= redraw_interval
                    });
            egui::Window::new("Console")
                .open(&mut open)
                .resizable(true)
                .vscroll(true)
                .hscroll(false)
                .default_size(egui::vec2(500.0, 250.0))
                .show(ctx, |ui: &mut egui::Ui| {
                    // Always render the console UI so input is processed
                    console_ui(ui, &mut self.console_state, self.settings.console_max_lines);
                    // Only clear dirty and update redraw time if log area changed
                    if should_redraw {
                        self.console_state.clear_dirty();
                        self.last_console_redraw = Some(now);
                    }
                });
            if !open {
                // Closing the window hides the console until re-enabled in settings
                self.settings.show_console = false;
                self.console_open = false;
                self.settings.save();
            }
            // After UI event handling, process any queued commands
            let pending_commands = self.console_state.take_pending();
            let mut all_lines: Vec<String> = Vec::new();
            if !pending_commands.is_empty() {
                let saves_root = PathBuf::from("saves");
                let mut context = AppConsoleContext {
                    console_state: &mut self.console_state,
                    settings: &mut self.settings,
                    ui_preview: &mut self.ui_preview,
                    saves_root,
                };
                for cmd in &pending_commands {
                    all_lines.extend(self.console_registry.execute(cmd, &mut context));
                }
            }
            self.console_state.log_lines(all_lines);
        }

        // Render any active preview windows (gated by dev mode so previews are a dev tool)
        if DEV_MODE_ENABLED && self.settings.developer_mode {
            self.ui_preview.render(ctx, DEV_MODE_ENABLED);

            // FPS graph overlay in the bottom-right corner when enabled
            if self.settings.show_fps_graph {
                egui::TopBottomPanel::bottom("fps_graph_panel")
                    .resizable(false)
                    .min_height(90.0)
                    .show_separator_line(false)
                    .show(ctx, |ui: &mut egui::Ui| {
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Min),
                            |ui: &mut egui::Ui| {
                                self.fps.ui(ui);
                            },
                        );
                    });
            }
        }

        // Simple FPS counter overlay — available to all users (no developer mode required).
        if self.settings.show_fps_counter {
            let current_fps = self.fps.current_fps();
            egui::Area::new(egui::Id::new("fps_counter_area"))
                .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-8.0, 8.0))
                .interactable(false)
                .show(ctx, |ui: &mut egui::Ui| {
                    ui.label(
                        egui::RichText::new(format!("FPS: {:.0}", current_fps))
                            .monospace()
                            .color(egui::Color32::from_rgb(180, 220, 180)),
                    );
                });
        }
    }

    /// Check if the app should quit
    fn should_quit(&self) -> bool {
        self.should_quit
    }
}

struct WinitApp {
    window: Option<Arc<Window>>,
    device: Option<egui_wgpu::wgpu::Device>,
    queue: Option<egui_wgpu::wgpu::Queue>,
    surface: Option<egui_wgpu::wgpu::Surface<'static>>,
    surface_config: Option<egui_wgpu::wgpu::SurfaceConfiguration>,
    egui_ctx: Option<egui::Context>,
    egui_winit_state: Option<egui_winit::State>,
    egui_renderer: Option<egui_wgpu::Renderer>,
    app: Option<DungeonCrawlerworld>,
    /// Time the most recent frame was rendered (for FPS-cap frame pacing).
    last_frame_time: Option<Instant>,
    /// Last applied VSync setting (to detect changes and reconfigure the surface).
    last_vsync: Option<bool>,
}

impl WinitApp {
    fn new() -> Self {
        Self {
            window: None,
            device: None,
            queue: None,
            surface: None,
            surface_config: None,
            egui_ctx: None,
            egui_winit_state: None,
            egui_renderer: None,
            app: None,
            last_frame_time: None,
            last_vsync: None,
        }
    }

    /// Returns the configured FPS cap (0 = unlimited).
    fn target_fps(&self) -> u32 {
        self.app
            .as_ref()
            .map(|a| a.settings.target_fps)
            .unwrap_or(0)
    }
}

impl winit::application::ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        use egui_wgpu::wgpu;

        if self.window.is_some() {
            return;
        }

        // Create window
        let window_attrs: winit::window::WindowAttributes =
            winit::window::WindowAttributes::default()
                .with_title("Dungeon crawler world")
                .with_inner_size(winit::dpi::PhysicalSize::new(400, 300))
                .with_min_inner_size(winit::dpi::PhysicalSize::new(300, 200));

        let window: Arc<Window> = Arc::new(event_loop.create_window(window_attrs).unwrap());

        // Initialize wgpu
        let instance: wgpu::Instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface: wgpu::Surface<'_> = instance.create_surface(window.clone()).unwrap();

        // Load settings early to apply the GPU power preference when selecting the adapter.
        let startup_settings = Settings::load();

        let adapter: wgpu::Adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: match startup_settings.power_preference {
                    PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
                    PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
                    PowerPreference::Default => wgpu::PowerPreference::default(),
                },
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: Default::default(),
        }))
        .unwrap();

        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
        let mut surface_config: wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>> = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        // Apply VSync setting from the loaded settings.
        surface_config.present_mode = if startup_settings.vsync {
            wgpu::PresentMode::Fifo
        } else {
            wgpu::PresentMode::AutoNoVsync
        };
        surface.configure(&device, &surface_config);
        self.last_vsync = Some(startup_settings.vsync);

        // Initialize egui
        let egui_ctx: Context = egui::Context::default();
        egui_ctx.set_style(Style {
            visuals: Visuals::dark(),
            ..Default::default()
        });

        let egui_winit_state: egui_winit::State = egui_winit::State::new(
            egui_ctx.clone(),
            ViewportId::ROOT,
            event_loop,
            Some(window.scale_factor() as f32),
            None,
            Some(wgpu::Limits::default().max_texture_dimension_2d as usize),
        );

        let egui_renderer: egui_wgpu::Renderer =
            egui_wgpu::Renderer::new(&device, surface_config.format, None, 1, true);

        // Create app
        let app: DungeonCrawlerworld = DungeonCrawlerworld::new();

        self.window = Some(window);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.egui_ctx = Some(egui_ctx);
        self.egui_winit_state = Some(egui_winit_state);
        self.egui_renderer = Some(egui_renderer);
        self.app = Some(app);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let window: &Arc<Window> = self.window.as_ref().unwrap();
        let egui_winit_state: &mut egui_winit::State = self.egui_winit_state.as_mut().unwrap();

        let response: egui_winit::EventResponse = egui_winit_state.on_window_event(window, &event);
        if response.consumed {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let (Some(surface), Some(device), Some(surface_config)) = (
                    self.surface.as_ref(),
                    self.device.as_ref(),
                    self.surface_config.as_mut(),
                ) {
                    surface_config.width = physical_size.width;
                    surface_config.height = physical_size.height;
                    surface.configure(device, surface_config);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.render();

                // Record the time this frame was completed for FPS-cap frame pacing.
                self.last_frame_time = Some(Instant::now());

                // Check if app wants to quit
                if self.app.as_ref().unwrap().should_quit() {
                    event_loop.exit();
                    return;
                }

                // When no FPS cap is active, keep requesting redraws immediately for
                // maximum throughput.  When a cap is set, about_to_wait() handles the
                // timing and requests the next redraw once the deadline has passed.
                if self.target_fps() == 0 {
                    if let Some(win) = &self.window {
                        win.request_redraw();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let target_fps = self.target_fps();

        if target_fps > 0 {
            let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
            let next_frame = self
                .last_frame_time
                .unwrap_or_else(Instant::now)
                .checked_add(frame_duration)
                .unwrap_or_else(Instant::now);
            if next_frame > Instant::now() {
                event_loop.set_control_flow(
                    winit::event_loop::ControlFlow::WaitUntil(next_frame),
                );
                return;
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl WinitApp {
    fn render(&mut self) {
        use egui_wgpu::wgpu;

        // Reconfigure the surface if the VSync setting has changed.
        {
            let new_vsync = self.app.as_ref().map(|a| a.settings.vsync);
            if new_vsync != self.last_vsync {
                if let (Some(vsync), Some(surface), Some(device), Some(config)) = (
                    new_vsync,
                    self.surface.as_ref(),
                    self.device.as_ref(),
                    self.surface_config.as_mut(),
                ) {
                    config.present_mode = if vsync {
                        wgpu::PresentMode::Fifo
                    } else {
                        wgpu::PresentMode::AutoNoVsync
                    };
                    surface.configure(device, config);
                    self.last_vsync = Some(vsync);
                }
            }
        }

        let window: &Arc<Window> = self.window.as_ref().unwrap();
        let device: &wgpu::Device = self.device.as_ref().unwrap();
        let queue: &wgpu::Queue = self.queue.as_ref().unwrap();
        let surface: &wgpu::Surface<'_> = self.surface.as_ref().unwrap();
        let surface_config: &wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>> =
            self.surface_config.as_ref().unwrap();
        let egui_ctx: &Context = self.egui_ctx.as_ref().unwrap();
        let egui_winit_state: &mut egui_winit::State = self.egui_winit_state.as_mut().unwrap();
        let egui_renderer: &mut egui_wgpu::Renderer = self.egui_renderer.as_mut().unwrap();
        let app: &mut DungeonCrawlerworld = self.app.as_mut().unwrap();

        let output_frame: egui_wgpu::wgpu::SurfaceTexture = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                log::error!("Failed to acquire next swap chain texture: {}", e);
                return;
            }
        };

        let output_view: egui_wgpu::wgpu::TextureView = output_frame
            .texture
            .create_view(&egui_wgpu::wgpu::TextureViewDescriptor::default());

        // Begin egui frame
        let raw_input: egui::RawInput = egui_winit_state.take_egui_input(window);
        let full_output: egui::FullOutput = egui_ctx.run(raw_input, |ctx: &Context| {
            app.update(ctx, window);
        });

        // Handle platform output
        egui_winit_state.handle_platform_output(window, full_output.platform_output);

        // Render egui
        let paint_jobs: Vec<egui::ClippedPrimitive> =
            egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_descriptor: egui_wgpu::ScreenDescriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [surface_config.width, surface_config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        let mut encoder: egui_wgpu::wgpu::CommandEncoder =
            device.create_command_encoder(&egui_wgpu::wgpu::CommandEncoderDescriptor {
                label: Some("egui encoder"),
            });

        // Upload egui textures
        for (id, image_delta) in &full_output.textures_delta.set {
            egui_renderer.update_texture(device, queue, *id, image_delta);
        }

        // Update buffers
        egui_renderer.update_buffers(device, queue, &mut encoder, &paint_jobs, &screen_descriptor);

        // Render to texture
        // Use wgpu's safe forget_lifetime() method to convert the RenderPass lifetime to 'static.
        // This is safe because CommandEncoder is in a locked state while the pass is active.
        {
            let rpass: egui_wgpu::wgpu::RenderPass<'_> =
                encoder.begin_render_pass(&egui_wgpu::wgpu::RenderPassDescriptor {
                    label: Some("egui main render pass"),
                    color_attachments: &[Some(egui_wgpu::wgpu::RenderPassColorAttachment {
                        view: &output_view,
                        resolve_target: None,
                        ops: egui_wgpu::wgpu::Operations {
                            load: egui_wgpu::wgpu::LoadOp::Clear(egui_wgpu::wgpu::Color::BLACK),
                            store: egui_wgpu::wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            let mut rpass_static: egui_wgpu::wgpu::RenderPass<'static> = rpass.forget_lifetime();
            egui_renderer.render(&mut rpass_static, &paint_jobs, &screen_descriptor);
        }

        queue.submit(Some(encoder.finish()));
        output_frame.present();

        // Free textures
        for id in &full_output.textures_delta.free {
            egui_renderer.free_texture(id);
        }
    }
}

/// Entry point for the application. Sets up the window and runs the event loop with wgpu/egui.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop: EventLoop<()> = EventLoop::new()?;
    let mut app: WinitApp = WinitApp::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
