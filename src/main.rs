#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

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

// Import necessary crates and modules
mod logger;
use egui::{CentralPanel, Context, RichText, Style, Visuals, ViewportId};
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

use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::event::WindowEvent;

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
    /// Quit confirmation dialog state.
    quit_confirm: bool,
    /// Flag indicating the app should quit.
    should_quit: bool,
}

impl DungeonCrawlerworld {
    /// Creates a new default instance of the main application struct, initializing all state.
    fn new() -> Self {
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
            quit_confirm: false,
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
        if (*self).last_fullscreen != Some((*self).settings.fullscreen) {
            (*self).last_fullscreen = Some((*self).settings.fullscreen);
            window.set_fullscreen(if (*self).settings.fullscreen {
                Some(winit::window::Fullscreen::Borderless(None))
            } else {
                None
            });
        }

        // Update FPS graph with delta time in ms
        let dt_ms: f32 = ctx.input(|i: &egui::InputState| -> f32 { (*i).stable_dt }) * 1000.0;
        (&mut (*self).fps).push_frame_time(dt_ms);

        // ESCAPE KEY HANDLING
        let escape_pressed: bool =
            ctx.input(|i: &egui::InputState| -> bool { i.key_pressed(egui::Key::Escape) });

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
                                (*self).quit_confirm = true;
                            }
                        }
                        // Quit confirmation dialog
                        if (*self).quit_confirm {
                            egui::Window::new("Quit Game?")
                                .collapsible(false)
                                .resizable(false)
                                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                                .show(ctx, |ui: &mut egui::Ui| {
                                    ui.label("Are you sure you want to quit?");
                                    ui.horizontal(|ui: &mut egui::Ui| {
                                        if (&ui.button("Yes")).clicked() {
                                            (*self).should_quit = true;
                                            (*self).quit_confirm = false;
                                        }
                                        if (&ui.button("No")).clicked() {
                                            (*self).quit_confirm = false;
                                        }
                                    });
                                });
                        }
                    },
                );
            });

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
                .show(ctx, |ui: &mut egui::Ui| {
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
        }
    }
}

impl winit::application::ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        use egui_wgpu::wgpu;
        
        if self.window.is_some() {
            return;
        }
        
        // Create window
        let window_attrs: winit::window::WindowAttributes = winit::window::WindowAttributes::default()
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
        
        let adapter: wgpu::Adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: Default::default(),
            },
        ))
        .unwrap();
        
        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
        let surface_config: wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>> = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);
        
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
        
        let egui_renderer: egui_wgpu::Renderer = egui_wgpu::Renderer::new(
            &device,
            surface_config.format,
            None,
            1,
            true,
        );
        
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
                
                // Check if app wants to quit
                if self.app.as_ref().unwrap().should_quit() {
                    event_loop.exit();
                    return;
                }
                
                // Request continuous repaints
                if let Some(win) = &self.window {
                    win.request_redraw();
                }
            }
            _ => {}
        }
    }
    
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl WinitApp {
    fn render(&mut self) {
        let window: &Arc<Window> = self.window.as_ref().unwrap();
        let device: &egui_wgpu::wgpu::Device = self.device.as_ref().unwrap();
        let queue: &egui_wgpu::wgpu::Queue = self.queue.as_ref().unwrap();
        let surface: &egui_wgpu::wgpu::Surface<'_> = self.surface.as_ref().unwrap();
        let surface_config: &egui_wgpu::wgpu::wgt::SurfaceConfiguration<Vec<egui_wgpu::wgpu::TextureFormat>> = self.surface_config.as_ref().unwrap();
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
        let paint_jobs: Vec<egui::ClippedPrimitive> = egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        
        let screen_descriptor: egui_wgpu::ScreenDescriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [surface_config.width, surface_config.height],
            pixels_per_point: window.scale_factor() as f32,
        };
        
        let mut encoder: egui_wgpu::wgpu::CommandEncoder = device.create_command_encoder(&egui_wgpu::wgpu::CommandEncoderDescriptor {
            label: Some("egui encoder"),
        });
        
        // Upload egui textures
        for (id, image_delta) in &full_output.textures_delta.set {
            egui_renderer.update_texture(device, queue, *id, image_delta);
        }
        
        // Update buffers
        egui_renderer.update_buffers(device, queue, &mut encoder, &paint_jobs, &screen_descriptor);
        
        // Render to texture
        // SAFETY: Working around egui-wgpu 0.32 API bug where render() requires 'static
        // but the RenderPass actually only needs to live until it's dropped.
        // This is safe because we immediately drop rpass before finishing encoder.
        {
            let mut rpass: egui_wgpu::wgpu::RenderPass<'_> = encoder.begin_render_pass(&egui_wgpu::wgpu::RenderPassDescriptor {
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
            let rpass_static: &mut egui_wgpu::wgpu::RenderPass<'static> = unsafe {
                std::mem::transmute(&mut rpass)
            };
            egui_renderer.render(rpass_static, &paint_jobs, &screen_descriptor);
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
