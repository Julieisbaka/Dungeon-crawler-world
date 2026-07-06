#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::sync::Arc;

// Import necessary crates and modules
mod logger;
use egui::{Context, Style, ViewportId, Visuals};
use logger::init_logger;

// Import saves and settings from library modules
use dungeon_crawler_world::input::GameCommand;
use dungeon_crawler_world::logic::settings_logic::{PowerPreference, Settings, VsyncMode};
use dungeon_crawler_world::ui::game_view::{
    show_game_view, GamePanel, GameViewAction, GameViewState,
};
use dungeon_crawler_world::ui::main_menu::{MainMenu, MainMenuAction};
use dungeon_crawler_world::world::WorldSession;

mod fps;
use fps::FpsGraph;
mod render;
use render::terrain::TerrainRenderer;

use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

/// Main app struct with settings state
use std::time::{Duration, Instant};

/// Main application struct holding all UI and game state.
struct DungeonCrawlerworld {
    /// Main menu UI state.
    menu: MainMenu,
    /// Current application settings.
    settings: Settings,
    /// Active world session when a save is loaded.
    world: Option<WorldSession>,
    /// UI state for windows owned by the active world session.
    game_view: GameViewState,
    /// Last known fullscreen state (for toggling fullscreen mode).
    last_fullscreen: Option<bool>,
    /// FPS graph data and rendering.
    fps: FpsGraph,
    /// Flag indicating the app should quit.
    should_quit: bool,
}

impl DungeonCrawlerworld {
    /// Creates a new default instance of the main application struct, initializing all state.
    fn new() -> Self {
        let _ = init_logger();
        Self {
            menu: MainMenu::new(),
            settings: Settings::default(),
            world: None,
            game_view: GameViewState::default(),
            last_fullscreen: None,
            fps: FpsGraph::default(),
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

        if self.world.is_some() {
            self.update_world(ctx);
        } else {
            match self.menu.show(ctx, &mut self.settings) {
                MainMenuAction::None => {}
                MainMenuAction::Quit => self.should_quit = true,
                MainMenuAction::LoadSave(save_name) => self.load_world(save_name),
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

    fn load_world(&mut self, save_name: String) {
        match WorldSession::load(save_name) {
            Ok(world) => {
                if let Ok(mut current_save) = dungeon_crawler_world::CURRENT_SAVE.lock() {
                    *current_save = Some(world.save_name.clone());
                }
                self.world = Some(world);
                self.game_view = GameViewState::default();
            }
            Err(error) => {
                log::error!("Failed to load world: {error}");
            }
        }
    }

    fn update_world(&mut self, ctx: &Context) {
        let Some(world) = self.world.as_mut() else {
            return;
        };

        handle_world_input(ctx, world, &mut self.game_view);

        if show_game_view(ctx, world, &mut self.game_view) == GameViewAction::ExitToMenu {
            self.world = None;
            self.game_view = GameViewState::default();
        }
    }
}

fn handle_world_input(ctx: &Context, world: &mut WorldSession, ui_state: &mut GameViewState) {
    let wants_keyboard = ctx.wants_keyboard_input();
    let dt_seconds = ctx.input(|input| input.stable_dt).max(0.0);
    world.update_physics(dt_seconds);

    let escape_pressed = ctx.input(|input| input.key_pressed(egui::Key::Escape));

    if escape_pressed {
        if ui_state.active_panel != GamePanel::None {
            ui_state.close_panel();
        } else {
            world.toggle_pause();
        }
        return;
    }

    if wants_keyboard || world.paused {
        return;
    }

    if ctx.input(|input| input.key_pressed(ui_state.keybindings.key_for(GameCommand::Skills))) {
        ui_state.open_panel(GamePanel::Skills);
    }
    if ctx.input(|input| input.key_pressed(ui_state.keybindings.key_for(GameCommand::Inventory))) {
        ui_state.open_panel(GamePanel::Inventory);
    }

    let movement = ctx.input(|input| {
        let mut movement = [0.0, 0.0];
        if input.key_down(egui::Key::W) || input.key_down(egui::Key::ArrowUp) {
            movement[1] -= 1.0;
        }
        if input.key_down(egui::Key::S) || input.key_down(egui::Key::ArrowDown) {
            movement[1] += 1.0;
        }
        if input.key_down(egui::Key::A) || input.key_down(egui::Key::ArrowLeft) {
            movement[0] -= 1.0;
        }
        if input.key_down(egui::Key::D) || input.key_down(egui::Key::ArrowRight) {
            movement[0] += 1.0;
        }
        movement
    });
    world.move_player_planar(movement, dt_seconds);

    if ctx.input(|input| input.key_pressed(egui::Key::Space)) {
        world.jump();
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
    terrain_renderer: Option<TerrainRenderer>,
    app: Option<DungeonCrawlerworld>,
    /// Time the most recent frame was rendered (for FPS-cap frame pacing).
    last_frame_time: Option<Instant>,
    /// wgpu adapter — kept alive so live VSync reconfiguration can query surface capabilities.
    adapter: Option<egui_wgpu::wgpu::Adapter>,
    /// Last applied VSync setting (to detect changes and reconfigure the surface).
    last_vsync: Option<VsyncMode>,
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
            terrain_renderer: None,
            app: None,
            last_frame_time: None,
            adapter: None,
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
        // Apply VSync setting from the loaded settings, gated against surface capabilities.
        let surface_caps = surface.get_capabilities(&adapter);
        let requested_present_mode = match startup_settings.vsync_mode {
            VsyncMode::On => wgpu::PresentMode::Fifo,
            VsyncMode::Adaptive => wgpu::PresentMode::FifoRelaxed,
            VsyncMode::Off => wgpu::PresentMode::AutoNoVsync,
        };
        surface_config.present_mode = if surface_caps
            .present_modes
            .contains(&requested_present_mode)
        {
            requested_present_mode
        } else {
            let fallback = if surface_caps
                .present_modes
                .contains(&wgpu::PresentMode::Fifo)
            {
                wgpu::PresentMode::Fifo
            } else {
                surface_caps
                    .present_modes
                    .first()
                    .copied()
                    .unwrap_or(wgpu::PresentMode::Fifo)
            };
            log::warn!(
                "Requested present mode {:?} for {:?} is not supported by this surface; falling back to {:?}",
                requested_present_mode,
                startup_settings.vsync_mode,
                fallback
            );
            fallback
        };
        surface.configure(&device, &surface_config);
        self.last_vsync = Some(startup_settings.vsync_mode);

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
        let terrain_renderer = TerrainRenderer::new(&device, surface_config.format);

        // Create app
        let app: DungeonCrawlerworld = DungeonCrawlerworld::new();

        self.window = Some(window);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.adapter = Some(adapter);
        self.egui_ctx = Some(egui_ctx);
        self.egui_winit_state = Some(egui_winit_state);
        self.egui_renderer = Some(egui_renderer);
        self.terrain_renderer = Some(terrain_renderer);
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
                event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame));
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
            let new_vsync = self.app.as_ref().map(|a| a.settings.vsync_mode);
            if new_vsync != self.last_vsync {
                if let (
                    Some(vsync_mode),
                    Some(surface),
                    Some(adapter),
                    Some(device),
                    Some(config),
                ) = (
                    new_vsync,
                    self.surface.as_ref(),
                    self.adapter.as_ref(),
                    self.device.as_ref(),
                    self.surface_config.as_mut(),
                ) {
                    let requested_present_mode = match vsync_mode {
                        VsyncMode::On => wgpu::PresentMode::Fifo,
                        VsyncMode::Adaptive => wgpu::PresentMode::FifoRelaxed,
                        VsyncMode::Off => wgpu::PresentMode::AutoNoVsync,
                    };
                    let caps = surface.get_capabilities(adapter);
                    config.present_mode = if caps.present_modes.contains(&requested_present_mode) {
                        requested_present_mode
                    } else {
                        let fallback = if caps.present_modes.contains(&wgpu::PresentMode::Fifo) {
                            wgpu::PresentMode::Fifo
                        } else {
                            caps.present_modes
                                .first()
                                .copied()
                                .unwrap_or(config.present_mode)
                        };
                        log::warn!(
                            "Requested present mode {:?} for {:?} is not supported; falling back to {:?}",
                            requested_present_mode,
                            vsync_mode,
                            fallback
                        );
                        fallback
                    };
                    surface.configure(device, config);
                    self.last_vsync = Some(vsync_mode);
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
        let terrain_renderer: &mut TerrainRenderer = self.terrain_renderer.as_mut().unwrap();
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
                label: Some("main frame encoder"),
            });

        if let Some(world) = app.world.as_ref() {
            terrain_renderer.render_world(
                device,
                queue,
                &mut encoder,
                &output_view,
                surface_config,
                world,
            );
        } else {
            terrain_renderer.clear(&mut encoder, &output_view, None);
        }

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
                            load: egui_wgpu::wgpu::LoadOp::Load,
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
