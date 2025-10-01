#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Global variable holding the name of the current save file, protected by a mutex for thread safety.
pub static CURRENT_SAVE: Lazy<Mutex<Option<String>>> =
    Lazy::new(|| -> Mutex<Option<String>> { Mutex::new(None) });

/// Sets the current save file name.
///
/// # Arguments
/// * `save_name` - The name of the save file to set as current.
pub fn set_current_save(save_name: &str) {
    let mut current: std::sync::MutexGuard<'_, Option<String>> = (&*CURRENT_SAVE).lock().unwrap();
    *current = Some(save_name.to_string());
    log::info!("Current save set to: {}", save_name);
}

use std::error::Error;

// Import necessary crates and modules for winit/wgpu setup
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId, WindowAttributes},
};
use egui_wgpu::wgpu;

mod logger;
use egui::{CentralPanel, Context, RichText, Style, Visuals};
use logger::init_logger;
mod saves;
use saves::show_save_ui;
mod new_save;
mod player;
mod settings;
use settings::{settings_ui, Settings, SettingsResult};
mod console;
use console::{console_ui, ConsoleState};
mod skills;
mod ui_preview;
use ui_preview::UiPreviewManager;
mod fps;
use fps::FpsGraph;

/// Developer mode flag is controlled via Cargo feature `dev-mode`.
/// Enabled in debug builds by default via `Cargo.toml` [features].
/// For release builds in CI, we pass `--no-default-features` to disable it.
const DEV_MODE_ENABLED: bool = cfg!(feature = "dev-mode");

/// Main app struct with settings state
use std::time::{Duration, Instant};

/// Main application struct holding all UI and game state.
struct DungeonCrawlerworld {
    /// Whether the settings window is currently shown.
    show_settings: bool,
    /// Whether the saves menu is currently shown.
    show_saves: bool,
    /// Current application settings.
    settings: Settings,
    /// State for the saves menu UI.
    save_menu_state: saves::SaveMenuState,
    /// State for the developer console.
    console_state: ConsoleState,
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
}

/// Application state that manages the window and rendering.
///
/// This struct coordinates between winit (window management), wgpu (GPU rendering),
/// and egui (UI framework). It holds all the state needed for the application lifecycle:
/// - Window creation and management via winit
/// - GPU device, queue, and surface for wgpu rendering
/// - egui integration for UI rendering through egui-winit and egui-wgpu
/// - The main game/application logic in DungeonCrawlerworld
struct App {
    /// The application window, wrapped in Arc for shared ownership with wgpu surface
    window: Option<std::sync::Arc<Window>>,
    /// wgpu surface for rendering to the window
    surface: Option<wgpu::Surface<'static>>,
    /// wgpu device for creating GPU resources
    device: Option<wgpu::Device>,
    /// wgpu command queue for submitting rendering commands
    queue: Option<wgpu::Queue>,
    /// Configuration for the wgpu surface (size, format, present mode)
    surface_config: Option<wgpu::SurfaceConfiguration>,
    /// egui-winit state for handling window events and input
    egui_winit: Option<egui_winit::State>,
    /// egui-wgpu renderer for drawing egui UI with wgpu
    egui_wgpu: Option<egui_wgpu::Renderer>,
    /// egui context shared across all UI rendering
    egui_ctx: egui::Context,
    /// The main application/game logic and state
    dungeon_crawler: DungeonCrawlerworld,
    /// Flag to signal when the application should quit
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let egui_ctx = egui::Context::default();
        egui_ctx.set_style(Style {
            visuals: Visuals::dark(),
            ..Default::default()
        });
        
        Self {
            window: None,
            surface: None,
            device: None,
            queue: None,
            surface_config: None,
            egui_winit: None,
            egui_wgpu: None,
            egui_ctx,
            // Initialize the main game logic with default settings, empty save state,
            // and sets up the logging system for the in-game console
            dungeon_crawler: DungeonCrawlerworld::default(),
            should_quit: false,
        }
    }
}

impl Default for DungeonCrawlerworld {
    /// Creates a new default instance of the main application struct, initializing all state.
    fn default() -> Self {
        let (_log_tx, log_rx) = init_logger();
        Self {
            show_settings: false,
            show_saves: false,
            settings: Settings::default(),
            save_menu_state: saves::SaveMenuState::default(),
            console_state: ConsoleState::default(),
            ui_preview: UiPreviewManager::new(),
            last_fullscreen: None,
            fps: FpsGraph::default(),
            console_open: false,
            last_show_console: Settings::default().show_console,
            log_rx: Some(log_rx),
            last_console_redraw: None,
        }
    }
}

impl DungeonCrawlerworld {
    /// Updates the application state and renders the UI for each frame.
    ///
    /// # Arguments
    /// * `ctx` - The egui context for UI rendering and input.
    /// * `should_quit` - Mutable reference to a flag indicating whether to quit
    /// * `window` - Reference to the winit window for fullscreen control
    fn update(&mut self, ctx: &Context, should_quit: &mut bool, window: &Window) {
        // Always repaint so the FPS graph and other time-based UI update in real time
        ctx.request_repaint();
        // Apply fullscreen setting when it changes
        if (*self).last_fullscreen != Some((*self).settings.fullscreen) {
            (*self).last_fullscreen = Some((*self).settings.fullscreen);
            // Set fullscreen mode via winit
            if (*self).settings.fullscreen {
                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            } else {
                window.set_fullscreen(None);
            }
        }

        // Update FPS graph with delta time in ms
        let dt_ms: f32 = ctx.input(|i: &egui::InputState| -> f32 { (*i).stable_dt }) * 1000.0;
        (&mut (*self).fps).push_frame_time(dt_ms);

        // ESCAPE KEY HANDLING
        let escape_pressed: bool =
            ctx.input(|i: &egui::InputState| -> bool { i.key_pressed(egui::Key::Escape) });
        // Quit confirmation dialog state
        static mut QUIT_CONFIRM: bool = false;
        let mut quit_confirm: bool = unsafe { QUIT_CONFIRM };

        CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&**&ctx.style())
                    .inner_margin(egui::Margin::same(0))
                    .outer_margin(egui::Margin::same(0)),
            )
            .show(ctx, |ui: &mut egui::Ui| {
                let avail: egui::Vec2 = ui.available_size();
                ui.allocate_ui_with_layout(
                    avail,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui: &mut egui::Ui| {
                        if (*self).show_settings {
                            ui.heading(RichText::new("Settings").size(28.0));
                            ui.add_space(8.0);
                            let mut back: bool = false;
                            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                                ui,
                                |ui: &mut egui::Ui| {
                                    ui.set_max_width(700.0);
                                    let res: SettingsResult =
                                        settings_ui(ui, &mut (*self).settings, DEV_MODE_ENABLED);
                                    if res.request_save {
                                        (&(*self).settings).save();
                                        (*self).show_settings = false;
                                    }
                                    if res.request_back {
                                        back = true;
                                    }
                                },
                            );
                            if back || escape_pressed {
                                (*self).show_settings = false;
                            }
                        } else if (*self).show_saves {
                            ui.heading(RichText::new("Saves Menu").size(28.0));
                            ui.add_space(8.0);
                            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                                ui,
                                |ui: &mut egui::Ui| {
                                    ui.set_max_width(900.0);
                                    show_save_ui(ui, &mut (*self).save_menu_state);
                                },
                            );
                            // Only close saves menu on explicit back, escape, or sub-menu exit
                            if (*self).save_menu_state.back_requested || escape_pressed {
                                (*self).save_menu_state.back_requested = false;
                                (*self).save_menu_state.in_new_save_menu = false;
                                (*self).save_menu_state.editing_save = None;
                                (*self).show_saves = false;
                            }
                        } else {
                            ui.add_space(8.0);
                            ui.heading(RichText::new("Game Menu").size(30.0));
                            ui.add_space(24.0);
                            if (&ui.add_sized([220.0, 36.0], egui::Button::new("Saves"))).clicked()
                            {
                                (*self).show_saves = true;
                            }
                            ui.add_space(8.0);
                            if (&ui.add_sized([220.0, 36.0], egui::Button::new("Settings")))
                                .clicked()
                            {
                                (*self).show_settings = true;
                            }
                            ui.add_space(8.0);
                            if (&ui.add_sized([220.0, 36.0], egui::Button::new("Quit"))).clicked() {
                                quit_confirm = true;
                            }
                        }
                        // Quit confirmation dialog
                        if quit_confirm {
                            egui::Window::new("Quit Game?")
                                .collapsible(false)
                                .resizable(false)
                                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                                .show(ctx, |ui: &mut egui::Ui| {
                                    ui.label("Are you sure you want to quit?");
                                    ui.horizontal(|ui: &mut egui::Ui| {
                                        if (&ui.button("Yes")).clicked() {
                                            *should_quit = true;
                                            quit_confirm = false;
                                        }
                                        if (&ui.button("No")).clicked() {
                                            quit_confirm = false;
                                        }
                                    });
                                });
                        }
                    },
                );
            });
        unsafe {
            QUIT_CONFIRM = quit_confirm;
        }

        // Developer Console window: only when enabled and explicitly opened this session
        // Detect setting edge to open on user toggle (not on startup load)
        if (*self).settings.show_console != (*self).last_show_console {
            if (*self).settings.show_console {
                (*self).console_open = true;
            }
            (*self).last_show_console = (*self).settings.show_console;
        }

        // Poll logger and write to in-game console if enabled, filter by verbosity
        if (*self).settings.log_to_console {
            if let Some(rx) = &(*self).log_rx {
                let verbosity: &settings::LogVerbosity = &(*self).settings.log_verbosity;
                while let Ok(msg) = rx.try_recv() {
                    // Simple filter: look for log level prefix in message
                    // e.g., "[ERROR] ...", "[WARN] ...", "[INFO] ...", "[DEBUG] ...", "[TRACE] ..."
                    let show: bool = match verbosity {
                        settings::LogVerbosity::Error => (&*msg).contains("[ERROR]"),
                        settings::LogVerbosity::Warn => {
                            (&*msg).contains("[ERROR]") || (&*msg).contains("[WARN]")
                        }
                        settings::LogVerbosity::Info => {
                            (&*msg).contains("[ERROR]")
                                || (&*msg).contains("[WARN]")
                                || (&*msg).contains("[INFO]")
                        }
                        settings::LogVerbosity::Debug => {
                            (&*msg).contains("[ERROR]")
                                || (&*msg).contains("[WARN]")
                                || (&*msg).contains("[INFO]")
                                || (&*msg).contains("[DEBUG]")
                        }
                        settings::LogVerbosity::Trace => true,
                    };
                    if show {
                        (&mut (*self).console_state).log_line(msg);
                    }
                }
            }
        }

        if DEV_MODE_ENABLED && (*self).settings.developer_mode && (*self).console_open {
            let mut open: bool = true;
            let now: Instant = Instant::now();
            let redraw_interval: Duration = Duration::from_millis(33); // ~30 FPS max
            let should_redraw: bool = (&(*self).console_state).is_dirty()
                && (*self)
                    .last_console_redraw
                    .map_or(true, |last: Instant| -> bool {
                        (&now).duration_since(last) >= redraw_interval
                    });
            egui::Window::new("Console")
                .open(&mut open)
                .resizable(true)
                .vscroll(true)
                .hscroll(false)
                .default_size(egui::vec2(500.0, 250.0))
                .show(ctx, |ui| {
                    // Always render the console UI so input is processed
                    console_ui(
                        ui,
                        &mut (*self).console_state,
                        (*self).settings.console_max_lines,
                    );
                    // Only clear dirty and update redraw time if log area changed
                    if should_redraw {
                        (&mut (*self).console_state).clear_dirty();
                        (*self).last_console_redraw = Some(now);
                    }
                });
            if !open {
                // Closing the window hides the console until re-enabled in settings
                (*self).settings.show_console = false;
                (*self).console_open = false;
                (&(*self).settings).save();
            }
            // After UI event handling, process any queued commands
            for cmd in (&mut (*self).console_state).take_pending() {
                let trimmed: &str = (&*cmd).trim();
                if trimmed.is_empty() {
                    continue;
                }
                let mut parts: std::str::SplitWhitespace<'_> = trimmed.split_whitespace();
                let head: &str = (&mut parts).next().unwrap_or("");
                match head {
                    "invoke" => {
                        let name: String = (&*parts.collect::<Vec<_>>()).join(" ");
                        if (&name).is_empty() {
                            (&mut (*self).console_state).log_line("Usage: invoke <ui>");
                        } else {
                            if DEV_MODE_ENABLED && (*self).settings.developer_mode {
                                match (&mut (*self).ui_preview).open_preview(&**&name) {
                                    Ok(()) => (&mut (*self).console_state)
                                        .log_line(format!("Invoked UI preview: {}", name)),
                                    Err(e) => (&mut (*self).console_state).log_line(e),
                                }
                            } else {
                                (&mut (*self).console_state)
                                    .log_line("UI previews are only available in Developer Mode.");
                            }
                        }
                    }
                    // Fallback to built-in commands
                    _ => (&mut (*self).console_state).run_command(trimmed),
                }
            }
        }

        // Render any active preview windows (gated by dev mode so previews are a dev tool)
        if DEV_MODE_ENABLED && (*self).settings.developer_mode {
            (&mut (*self).ui_preview).render(ctx, DEV_MODE_ENABLED);

            // FPS graph overlay in the bottom-right corner when enabled
            if (*self).settings.show_fps_graph {
                egui::TopBottomPanel::bottom("fps_graph_panel")
                    .resizable(false)
                    .min_height(90.0)
                    .show_separator_line(false)
                    .show(ctx, |ui: &mut egui::Ui| {
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Min),
                            |ui: &mut egui::Ui| {
                                (&(*self).fps).ui(ui);
                            },
                        );
                    });
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = WindowAttributes::default()
                .with_inner_size(winit::dpi::LogicalSize::new(400.0, 300.0))
                .with_min_inner_size(winit::dpi::LogicalSize::new(300.0, 200.0))
                .with_title("Dungeon crawler world");

            let window = std::sync::Arc::new(event_loop.create_window(window_attrs).unwrap());
            self.window = Some(window.clone());

            // Initialize GPU resources
            pollster::block_on(self.initialize_wgpu(window));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if let Some(window) = &self.window {
            if window.id() != window_id {
                return;
            }

            // Handle egui input events
            if let Some(egui_winit) = &mut self.egui_winit {
                let response = egui_winit.on_window_event(window, &event);
                if response.consumed {
                    return;
                }
            }

            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(physical_size) => {
                    self.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.render();
                    if self.should_quit {
                        event_loop.exit();
                    }
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl App {
    async fn initialize_wgpu(&mut self, window: std::sync::Arc<Window>) {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default())
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Initialize egui-winit
        let egui_winit = egui_winit::State::new(
            self.egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None, // theme: None uses system theme detection; dark theme is already set in egui_ctx
            Some(2048), // max_texture_side: maximum texture size for egui textures
        );

        // Initialize egui-wgpu renderer
        let egui_wgpu = egui_wgpu::Renderer::new(&device, surface_format, None, 1, false);

        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface_config = Some(config);
        self.egui_winit = Some(egui_winit);
        self.egui_wgpu = Some(egui_wgpu);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if let (Some(surface), Some(device), Some(config)) = 
            (&self.surface, &self.device, &mut self.surface_config) {
            if new_size.width > 0 && new_size.height > 0 {
                config.width = new_size.width;
                config.height = new_size.height;
                surface.configure(device, config);
            }
        }
    }

    fn render(&mut self) {
        if let (Some(window), Some(surface), Some(device), Some(queue), Some(egui_winit), Some(egui_wgpu)) = 
            (&self.window, &self.surface, &self.device, &self.queue, &mut self.egui_winit, &mut self.egui_wgpu) {
            
            let output = surface.get_current_texture().unwrap();
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Begin egui frame
            let raw_input = egui_winit.take_egui_input(window);
            let full_output = self.egui_ctx.run(raw_input, |ctx| {
                self.dungeon_crawler.update(ctx, &mut self.should_quit, window);
            });

            // Handle egui output
            egui_winit.handle_platform_output(window, full_output.platform_output);

            let tris = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
            for (id, image_delta) in &full_output.textures_delta.set {
                egui_wgpu.update_texture(device, queue, *id, image_delta);
            }

            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [window.inner_size().width, window.inner_size().height],
                pixels_per_point: window.scale_factor() as f32,
            };

            // Clean up textures first
            for id in &full_output.textures_delta.free {
                egui_wgpu.free_texture(id);
            }

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            egui_wgpu.update_buffers(device, queue, &mut encoder, &tris, &screen_descriptor);

            let command_buffer = {
                let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

                egui_wgpu.render(
                    &mut render_pass.forget_lifetime(),
                    &tris,
                    &screen_descriptor,
                );
                
                encoder.finish()
            };

            queue.submit(std::iter::once(command_buffer));
            output.present();
        }
    }
}

/// Entry point for the application. Sets up the window and runs the winit event loop.
fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
    
    Ok(())
}
