// Camera utilities implementation for Horizon (Freecam, Freelock, Zoom)
pub mod config;
pub mod freecam;
pub mod gui;

pub use config::CameraConfig;
pub use freecam::turn;
pub use gui::ConfigScreen;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[allow(dead_code)]
const VIEWS: [mod_api::CameraType; 3] = [
    mod_api::CameraType::FirstPerson,
    mod_api::CameraType::ThirdPersonBack,
    mod_api::CameraType::ThirdPersonFront,
];

#[derive(Default)]
pub struct CameraUtilsMod;

pub static ZOOM_PRESSED: AtomicBool = AtomicBool::new(false);
pub static ZOOM_TOGGLED: AtomicBool = AtomicBool::new(false);
pub static FREELOCK_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static FREECAM_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static GUI_OPEN: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, PartialEq)]
pub struct ZoomRuntime {
    pub current_step: i32,
    pub is_active: bool,
}

impl Default for ZoomRuntime {
    fn default() -> Self {
        Self {
            current_step: 0,
            is_active: false,
        }
    }
}

pub static ZOOM_RUNTIME: Mutex<ZoomRuntime> = Mutex::new(ZoomRuntime {
    current_step: 0,
    is_active: false,
});

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoomDescriptor {
    pub factor: f32,
    pub smooth: bool,
    pub turn: f32,
}

pub static CONFIG: Mutex<CameraConfig> = Mutex::new(CameraConfig {
    zoom: config::ZoomSettings {
        initial_zoom: 4.0,
        enable_scroll_zoom: true,
        scroll_step_count: 10,
        zoom_per_step: 1.5,
        remember_zoom_steps: false,
        key_behavior: config::KeyBehavior::Hold,
        relative_sensitivity: 100.0,
        zoom_in_time: 1.0,
        zoom_out_time: 0.5,
        zoom_in_transition: config::TransitionType::EaseOutExponential,
        zoom_out_transition: config::TransitionType::EaseOutExponential,
        scroll_smoothness: 0.7,
    },
    freecam: config::FreecamSettings {
        flight_speed: 0.5,
        fast_modifier: 2.5,
        slow_modifier: 0.2,
        smooth_camera: true,
        inertia: 0.8,
        show_player_model: true,
        freeze_player_movement: true,
    },
    freelock: config::FreelockSettings {
        toggle_mode: true,
        return_speed: 1.0,
        sensitivity_ratio: 1.0,
        perspective_lock: false,
    },
});

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CameraState {
    pub is_zoomed: bool,
    pub is_freelock: bool,
    pub freelock_yaw: f32,
    pub freelock_pitch: f32,
    pub is_freecam: bool,
    pub freecam_pos: [f64; 3],
    pub freecam_yaw: f32,
    pub freecam_pitch: f32,
}

impl CameraState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key(
        &mut self,
        key: &str,
        pressed: bool,
        current_pos: [f64; 3],
        current_yaw: f32,
        current_pitch: f32,
    ) {
        match key {
            "key.keyboard.c" | "zoom" => {
                self.is_zoomed = pressed;
                ZOOM_PRESSED.store(pressed, Ordering::Relaxed);
            }
            "key.keyboard.x" | "freelock" if pressed => {
                self.is_freelock = !self.is_freelock;
                if self.is_freelock {
                    self.freelock_yaw = current_yaw;
                    self.freelock_pitch = current_pitch;
                }
                FREELOCK_ACTIVE.store(self.is_freelock, Ordering::Relaxed);
            }
            "key.keyboard.u" | "freecam" if pressed => {
                self.is_freecam = !self.is_freecam;
                if self.is_freecam {
                    self.freecam_pos = current_pos;
                    self.freecam_yaw = current_yaw;
                    self.freecam_pitch = current_pitch;
                }
                FREECAM_ACTIVE.store(self.is_freecam, Ordering::Relaxed);
            }
            _ => {}
        }
    }

    pub fn turn_camera(&mut self, xo: f64, yo: f64, player_yaw: &mut f32, player_pitch: &mut f32) {
        if self.is_freecam {
            self.freecam_yaw = (self.freecam_yaw + xo as f32 * 0.15).rem_euclid(360.0);
            self.freecam_pitch = (self.freecam_pitch + yo as f32 * 0.15).clamp(-90.0, 90.0);
        } else if self.is_freelock {
            self.freelock_yaw = (self.freelock_yaw + xo as f32 * 0.15).rem_euclid(360.0);
            self.freelock_pitch = (self.freelock_pitch + yo as f32 * 0.15).clamp(-90.0, 90.0);
        } else {
            *player_yaw = (*player_yaw + xo as f32 * 0.15).rem_euclid(360.0);
            *player_pitch = (*player_pitch + yo as f32 * 0.15).clamp(-90.0, 90.0);
        }
    }

    pub fn override_camera_view(
        &self,
        original_pos: [f64; 3],
        original_yaw: f32,
        original_pitch: f32,
    ) -> ([f64; 3], f32, f32) {
        if self.is_freecam {
            (self.freecam_pos, self.freecam_yaw, self.freecam_pitch)
        } else if self.is_freelock {
            (original_pos, self.freelock_yaw, self.freelock_pitch)
        } else {
            (original_pos, original_yaw, original_pitch)
        }
    }
}

pub static GLOBAL_CAMERA: Mutex<CameraState> = Mutex::new(CameraState {
    is_zoomed: false,
    is_freelock: false,
    freelock_yaw: 0.0,
    freelock_pitch: 0.0,
    is_freecam: false,
    freecam_pos: [0.0, 0.0, 0.0],
    freecam_yaw: 0.0,
    freecam_pitch: 0.0,
});

pub fn get_zoom_descriptor() -> Option<ZoomDescriptor> {
    if ZOOM_PRESSED.load(Ordering::Relaxed) {
        let (factor, turn) = if let Ok(cfg) = CONFIG.lock() {
            let f = cfg.zoom.initial_zoom;
            let t = (1.0 / f) * (cfg.zoom.relative_sensitivity / 100.0);
            (f, t)
        } else {
            (4.0, 0.25)
        };
        Some(ZoomDescriptor {
            factor,
            smooth: true,
            turn,
        })
    } else {
        None
    }
}

// Horizon Native Mod Hooks Implementation
#[cfg(not(test))]
#[mod_api::horizon_mod]
impl mod_api::Hooks for CameraUtilsMod {
    fn register(&self, r: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        // Key Mappings
        r.register_key_mapping(
            mod_api::KeyRegistration::new("zoom", "key.keyboard.c")
                .label("key.camera_utils.zoom")
                .context(mod_api::KeyConflictContext::InGame),
        );
        r.register_key_mapping(
            mod_api::KeyRegistration::new("freelock", "key.keyboard.x")
                .label("key.camera_utils.freelock")
                .context(mod_api::KeyConflictContext::InGame),
        );
        r.register_key_mapping(
            mod_api::KeyRegistration::new("freecam", "key.keyboard.u")
                .label("key.camera_utils.freecam")
                .context(mod_api::KeyConflictContext::InGame),
        );
        r.register_key_mapping(
            mod_api::KeyRegistration::new("config_gui", "key.keyboard.k")
                .label("key.camera_utils.config_gui")
                .context(mod_api::KeyConflictContext::InGame),
        );

        // Native Settings (options > mods > camera-utils config)
        // Zoom Category
        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "zoom_initial",
                "[Zoom] Initial Zoom",
                1.0,
                10.0,
                0.5,
                4.0,
            )
            .tooltip("The zoom level when you first press the keybind."),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle(
                "zoom_scroll_enabled",
                "[Zoom] Enable Scroll Zoom",
                true,
            )
            .tooltip("Allow mouse wheel scrolling to adjust zoom level"),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "zoom_scroll_step_count",
                "[Zoom] Scroll Step Count",
                3.0,
                20.0,
                1.0,
                10.0,
            )
            .tooltip("How many scroll steps in each direction. You can scroll to zoom in or out from the initial zoom."),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "zoom_per_step",
                "[Zoom] Zoom Per Step",
                1.1,
                2.0,
                0.05,
                1.5,
            )
            .tooltip("The zoom multiplier applied per scroll step. 1.5x means each step zooms in 50% more."),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle(
                "zoom_remember_steps",
                "[Zoom] Remember Zoom Steps",
                false,
            )
            .tooltip("Keep scroll zoom amount between zooming in and out."),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::choice(
                "zoom_key_behavior",
                "[Zoom] Key Behavior",
                ["Hold", "Toggle"],
                0,
            )
            .tooltip("Hold key to zoom, or press key to toggle zoom on/off"),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "zoom_relative_sensitivity",
                "[Zoom] Relative Sensitivity",
                0.0,
                150.0,
                5.0,
                100.0,
            )
            .tooltip("Mouse sensitivity percentage relative to zoom level (0% to 150%)"),
        );

        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "freecam_speed",
                "[Freecam] Flight Speed",
                freecam::MIN_SPEED,
                freecam::MAX_SPEED,
                freecam::SPEED_STEP,
                freecam::DEFAULT_SPEED,
            )
            .tooltip("Detached camera flight speed"),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle(
                "freecam_noclip",
                "[Freecam] NoClip",
                true,
            )
            .tooltip("Fly through blocks"),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle(
                "freeze_player",
                "[Freecam] Freeze Player Movement",
                true,
            )
            .tooltip("Prevent player character from moving while in Freecam"),
        );

        // Freelock Category
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle(
                "toggle_mode",
                "[Freelock] Toggle Mode",
                true,
            )
            .tooltip("Press to toggle Freelock instead of holding key"),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "return_speed",
                "[Freelock] Return Speed",
                0.1,
                2.0,
                0.1,
                1.0,
            )
            .tooltip("Speed of camera return transition when exiting Freelock"),
        );
    }

    fn client_tick_post(&self, host: &mut dyn mod_api::ClientHost, _: &mut mod_api::ClientTickPostEvent) {
        if !host.playing() {
            return;
        }

        // Poll registered key state
        let zoom_down = host.is_down("camera_utils", "zoom");
        let zoom_clicked = host.consume_click("camera_utils", "zoom");

        let key_behavior = if let Ok(cfg) = CONFIG.lock() {
            cfg.zoom.key_behavior
        } else {
            config::KeyBehavior::Hold
        };

        let remember = host.setting_bool("zoom_remember_steps").unwrap_or_else(|| {
            if let Ok(cfg) = CONFIG.lock() {
                cfg.zoom.remember_zoom_steps
            } else {
                false
            }
        });

        match key_behavior {
            config::KeyBehavior::Hold => {
                let prev_down = ZOOM_PRESSED.swap(zoom_down, Ordering::Relaxed);
                if zoom_down && !prev_down {
                    if !remember {
                        if let Ok(mut runtime) = ZOOM_RUNTIME.lock() {
                            runtime.current_step = 0;
                            runtime.is_active = false;
                        }
                    }

                    let (initial_zoom, zoom_per_step) = if let Ok(cfg) = CONFIG.lock() {
                        (cfg.zoom.initial_zoom, cfg.zoom.zoom_per_step)
                    } else {
                        (4.0, 1.5)
                    };
                    let step = if let Ok(runtime) = ZOOM_RUNTIME.lock() { runtime.current_step } else { 0 };
                    let factor = (initial_zoom * zoom_per_step.powi(step)).clamp(1.0, 50.0);
                    println!("[CameraUtils] Zoom Started: factor={factor:.2}x (step={step}, remember={remember})");
                } else if !zoom_down && prev_down {
                    println!("[CameraUtils] Zoom Ended");
                    if !remember {
                        if let Ok(mut runtime) = ZOOM_RUNTIME.lock() {
                            runtime.current_step = 0;
                            runtime.is_active = false;
                        }
                    }
                }
            }
            config::KeyBehavior::Toggle => {
                if zoom_clicked {
                    let prev = ZOOM_TOGGLED.load(Ordering::Relaxed);
                    let next = !prev;
                    ZOOM_TOGGLED.store(next, Ordering::Relaxed);
                    if next {
                        if !remember {
                            if let Ok(mut runtime) = ZOOM_RUNTIME.lock() {
                                runtime.current_step = 0;
                                runtime.is_active = false;
                            }
                        }

                        let (initial_zoom, zoom_per_step) = if let Ok(cfg) = CONFIG.lock() {
                            (cfg.zoom.initial_zoom, cfg.zoom.zoom_per_step)
                        } else {
                            (4.0, 1.5)
                        };
                        let step = if let Ok(runtime) = ZOOM_RUNTIME.lock() { runtime.current_step } else { 0 };
                        let factor = (initial_zoom * zoom_per_step.powi(step)).clamp(1.0, 10.0);
                        println!("[CameraUtils] Zoom Toggled ON: factor={factor:.2}x (step={step}, remember={remember})");
                    } else {
                        println!("[CameraUtils] Zoom Toggled OFF");
                        if !remember {
                            if let Ok(mut runtime) = ZOOM_RUNTIME.lock() {
                                runtime.current_step = 0;
                                runtime.is_active = false;
                            }
                        }
                    }
                }
            }
        }

        let cam = host.camera();
        let pos = cam.as_ref().map_or([0.0; 3], |c| c.position);
        let yaw = cam.as_ref().map_or(0.0, |c| c.yaw);
        let pitch = cam.as_ref().map_or(0.0, |c| c.pitch);

        while host.consume_click("camera_utils", "config_gui") {
            let current = GUI_OPEN.load(Ordering::Relaxed);
            GUI_OPEN.store(!current, Ordering::Relaxed);
        }

        let freelock_clicked = host.consume_click("camera_utils", "freelock");
        while host.consume_click("camera_utils", "freecam") {
            if freecam::active() {
                if let Some(view) = freecam::stop() {
                    host.set_camera_type(VIEWS[usize::from(view).min(VIEWS.len() - 1)]);
                }
                FREECAM_ACTIVE.store(false, Ordering::Relaxed);
            } else {
                if let Some(cam) = host.camera() {
                    let view = VIEWS
                        .iter()
                        .position(|v| *v == host.camera_type())
                        .unwrap_or(0) as u8;
                    let noclip = host.setting_bool("freecam_noclip").unwrap_or(true);
                    let speed = host.setting_f64("freecam_speed").unwrap_or(freecam::DEFAULT_SPEED);
                    freecam::set_settings(freecam::Settings::new(noclip, speed));
                    freecam::start(cam.position, cam.yaw, cam.pitch, view);
                    host.set_camera_type(mod_api::CameraType::ThirdPersonBack);
                    FREECAM_ACTIVE.store(true, Ordering::Relaxed);
                }
            }
        }

        if freecam::active() {
            if host.camera_entity().is_none() {
                if let Some(view) = freecam::stop() {
                    host.set_camera_type(VIEWS[usize::from(view).min(VIEWS.len() - 1)]);
                }
                FREECAM_ACTIVE.store(false, Ordering::Relaxed);
            } else {
                host.set_camera_type(mod_api::CameraType::ThirdPersonBack);
                let noclip = host.setting_bool("freecam_noclip").unwrap_or(true);
                let speed = host.setting_f64("freecam_speed").unwrap_or(freecam::DEFAULT_SPEED);
                let current = freecam::Settings::new(noclip, speed);
                freecam::set_settings(current);
                let host_ref = &*host;
                freecam::tick(|block| !current.noclip && !host_ref.block_collision(block).is_empty());
            }
        }

        if freelock_clicked {
            if let Ok(mut state) = GLOBAL_CAMERA.lock() {
                state.handle_key("freelock", true, pos, yaw, pitch);
            }
        }
    }

    fn input_key(&self, host: &mut dyn mod_api::ClientHost, event: &mut mod_api::InputKeyEvent) {
        let pressed = match event.action {
            mod_api::KeyAction::Press => true,
            mod_api::KeyAction::Release => false,
            _ => return,
        };

        if freecam::active()
            && let Some(input) = freecam::MOVES.iter().position(|m| *m == event.key)
            && (!pressed || host.playing())
        {
            freecam::hold(input, pressed);
        }

        let cam = host.camera();
        let pos = cam.as_ref().map_or([0.0; 3], |c| c.position);
        let yaw = cam.as_ref().map_or(0.0, |c| c.yaw);
        let pitch = cam.as_ref().map_or(0.0, |c| c.pitch);

        if let Ok(mut state) = GLOBAL_CAMERA.lock() {
            state.handle_key(event.key, pressed, pos, yaw, pitch);
            if let Some(layout) = event.layout_key {
                state.handle_key(layout, pressed, pos, yaw, pitch);
            }
        }
    }

    fn movement_input(&self, _host: &dyn mod_api::ClientHost, event: &mut mod_api::MovementInputEvent) {
        if freecam::active() {
            let freeze = if let Ok(cfg) = CONFIG.lock() {
                cfg.freecam.freeze_player_movement
            } else {
                true
            };
            if freeze {
                event.forward = 0.0;
                event.strafe = 0.0;
                event.jump = false;
                event.sneak = false;
                event.forward_key = false;
                event.backward_key = false;
                event.left_key = false;
                event.right_key = false;
                event.sprint = false;
            }
        }
    }

    fn mouse_scrolling(&self, host: &mut dyn mod_api::ClientHost, event: &mut mod_api::MouseScrollingEvent) -> mod_api::Flow {
        if !host.playing() {
            return mod_api::Flow::Continue;
        }

        let is_zooming = {
            let key_behavior = if let Ok(cfg) = CONFIG.lock() {
                cfg.zoom.key_behavior
            } else {
                config::KeyBehavior::Hold
            };

            match key_behavior {
                config::KeyBehavior::Hold => host.is_down("camera_utils", "zoom") || ZOOM_PRESSED.load(Ordering::Relaxed),
                config::KeyBehavior::Toggle => ZOOM_TOGGLED.load(Ordering::Relaxed),
            }
        };

        if !is_zooming {
            if freecam::active() {
                let steps = event.accumulated[1];
                if steps != 0 {
                    if let Some(changed) = freecam::wheel(steps as f32) {
                        host.set_setting("camera_utils", "freecam_speed", &changed.speed.to_string());
                        return mod_api::Flow::Cancel;
                    }
                }
            }
            return mod_api::Flow::Continue;
        }

        let (enabled, step_count) = if let Ok(cfg) = CONFIG.lock() {
            (cfg.zoom.enable_scroll_zoom, cfg.zoom.scroll_step_count as i32)
        } else {
            (true, 10)
        };

        if !enabled {
            return mod_api::Flow::Continue;
        }

        let wheel_delta = event.scroll[1];
        if wheel_delta != 0.0 {
            let (initial_zoom, zoom_per_step) = if let Ok(cfg) = CONFIG.lock() {
                (cfg.zoom.initial_zoom, cfg.zoom.zoom_per_step)
            } else {
                (4.0, 1.5)
            };

            if let Ok(mut runtime) = ZOOM_RUNTIME.lock() {
                runtime.is_active = true;
                if wheel_delta > 0.0 {
                    // Prevent increasing step further if factor is already >= 10.0
                    let current_factor = initial_zoom * zoom_per_step.powi(runtime.current_step);
                    if current_factor < 10.0 {
                        runtime.current_step = (runtime.current_step + 1).min(step_count);
                    }
                } else {
                    // Prevent decreasing step further if factor is already <= 1.0
                    let current_factor = initial_zoom * zoom_per_step.powi(runtime.current_step);
                    if current_factor > 1.0 {
                        runtime.current_step = (runtime.current_step - 1).max(-step_count);
                    }
                }

                let factor = (initial_zoom * zoom_per_step.powi(runtime.current_step)).clamp(1.0, 10.0);
                println!("[CameraUtils] Zoom Scroll: delta={wheel_delta:.1}, step={}, factor={factor:.2}x", runtime.current_step);
            }
            return mod_api::Flow::Cancel;
        }

        mod_api::Flow::Continue
    }

    fn zoom(&self, host: &dyn mod_api::ClientHost, _event: &mut mod_api::ZoomEvent) -> Option<mod_api::Zoom> {
        let (key_behavior, initial_zoom, zoom_per_step, relative_sens, remember) = if let Ok(cfg) = CONFIG.lock() {
            (
                cfg.zoom.key_behavior,
                cfg.zoom.initial_zoom,
                cfg.zoom.zoom_per_step,
                cfg.zoom.relative_sensitivity,
                cfg.zoom.remember_zoom_steps,
            )
        } else {
            (config::KeyBehavior::Hold, 4.0, 1.5, 100.0, false)
        };

        let is_zooming = match key_behavior {
            config::KeyBehavior::Hold => host.is_down("camera_utils", "zoom") || ZOOM_PRESSED.load(Ordering::Relaxed),
            config::KeyBehavior::Toggle => ZOOM_TOGGLED.load(Ordering::Relaxed),
        };

        if !is_zooming {
            if !remember {
                if let Ok(mut runtime) = ZOOM_RUNTIME.lock() {
                    if runtime.current_step != 0 || runtime.is_active {
                        runtime.current_step = 0;
                        runtime.is_active = false;
                    }
                }
            }
            return None;
        }

        let step = if let Ok(runtime) = ZOOM_RUNTIME.lock() {
            runtime.current_step
        } else {
            0
        };

        // Calculate final magnification factor: initial_zoom * (zoom_per_step ^ step)
        // Horizon engine clamps native zoom factor to 1.0..=10.0
        let factor = (initial_zoom * zoom_per_step.powi(step)).clamp(1.0, 10.0);

        // Relative sensitivity: inversely proportional to factor, scaled by relative_sens %
        // If factor <= 1.0, keep full sensitivity (1.0) so camera movement is never restricted
        let turn = if factor <= 1.001 {
            1.0
        } else {
            let base_turn = 1.0 / factor;
            (base_turn * (relative_sens / 100.0)).clamp(0.05, 1.0)
        };

        Some(mod_api::Zoom {
            factor,
            smooth: true,
            turn,
        })
    }

    fn setting_changed(&self, host: &mut dyn mod_api::ClientHost, event: &mut mod_api::SettingChangedEvent<'_>) {
        if let Ok(mut cfg) = CONFIG.lock() {
            match event.key {
                "zoom_initial" | "zoom_divisor" => {
                    if let Some(val) = host.setting_f64("zoom_initial").or_else(|| host.setting_f64("zoom_divisor")) {
                        cfg.zoom.initial_zoom = val as f32;
                    }
                }
                "zoom_scroll_enabled" | "scroll_zoom" => {
                    if let Some(val) = host.setting_bool("zoom_scroll_enabled").or_else(|| host.setting_bool("scroll_zoom")) {
                        cfg.zoom.enable_scroll_zoom = val;
                    }
                }
                "zoom_scroll_step_count" => {
                    if let Some(val) = host.setting_f64("zoom_scroll_step_count") {
                        cfg.zoom.scroll_step_count = val as u32;
                    }
                }
                "zoom_per_step" => {
                    if let Some(val) = host.setting_f64("zoom_per_step") {
                        cfg.zoom.zoom_per_step = val as f32;
                    }
                }
                "zoom_remember_steps" => {
                    if let Some(val) = host.setting_bool("zoom_remember_steps") {
                        cfg.zoom.remember_zoom_steps = val;
                    }
                }
                "zoom_key_behavior" => {
                    if let Some(choice_idx) = host.setting_choice("zoom_key_behavior") {
                        cfg.zoom.key_behavior = match choice_idx {
                            1 => config::KeyBehavior::Toggle,
                            _ => config::KeyBehavior::Hold,
                        };
                    }
                }
                "zoom_relative_sensitivity" => {
                    if let Some(val) = host.setting_f64("zoom_relative_sensitivity") {
                        cfg.zoom.relative_sensitivity = val as f32;
                    }
                }
                "freecam_speed" => {
                    if let Some(val) = host.setting_f64("freecam_speed") {
                        cfg.freecam.flight_speed = val;
                    }
                }
                "freeze_player" => {
                    if let Some(val) = host.setting_bool("freeze_player") {
                        cfg.freecam.freeze_player_movement = val;
                    }
                }
                "toggle_mode" => {
                    if let Some(val) = host.setting_bool("toggle_mode") {
                        cfg.freelock.toggle_mode = val;
                    }
                }
                "return_speed" => {
                    if let Some(val) = host.setting_f64("return_speed") {
                        cfg.freelock.return_speed = val as f32;
                    }
                }
                _ => {}
            }
        }
    }

    fn camera_pose(&self, _host: &dyn mod_api::ClientHost, event: &mut mod_api::CameraPoseEvent) {
        if let Some((at, yaw, pitch)) = freecam::view(event.partial) {
            *event.position = at;
            event.yaw = yaw;
            event.pitch = pitch;
            return;
        }

        let freelock = FREELOCK_ACTIVE.load(Ordering::Relaxed);
        if freelock {
            if let Ok(state) = GLOBAL_CAMERA.lock() {
                let (new_pos, new_yaw, new_pitch) =
                    state.override_camera_view(*event.position, event.yaw, event.pitch);
                *event.position = new_pos;
                event.yaw = new_yaw;
                event.pitch = new_pitch;
            }
        }
    }

    fn render_gui_post(
        &self,
        host: &dyn mod_api::ClientHost,
        event: &mut mod_api::RenderGuiPostEvent<'_>,
    ) {
        if !GUI_OPEN.load(Ordering::Relaxed) {
            return;
        }

        // Draw In-Game Quick Menu Panel
        let (window_w, _window_h) = {
            let s = event.draw.size();
            (s[0], s[1])
        };

        let panel_w = 260.0;
        let panel_h = 190.0;
        let x = (window_w - panel_w) / 2.0;
        let y = 30.0;

        // Dark translucent background with border
        event.draw.fill([x, y, panel_w, panel_h], 0xDD111118);
        event.draw.fill([x - 1.0, y - 1.0, panel_w + 2.0, 1.0], 0xFF4A90E2);
        event.draw.fill([x - 1.0, y + panel_h, panel_w + 2.0, 1.0], 0xFF4A90E2);
        event.draw.fill([x - 1.0, y, 1.0, panel_h], 0xFF4A90E2);
        event.draw.fill([x + panel_w, y, 1.0, panel_h], 0xFF4A90E2);

        // Header
        event.draw.text("§6§l[ Camera Utils Menu ]", [x + 10.0, y + 8.0], 0xFFFFFFFF, true);
        event.draw.text("§7Press 'K' to toggle", [x + panel_w - 95.0, y + 9.0], 0xFFAAAAAA, false);

        // Divider
        event.draw.fill([x + 10.0, y + 24.0, panel_w - 20.0, 1.0], 0x44FFFFFF);

        // Zoom Info
        let (initial_zoom, zoom_per_step) = if let Ok(cfg) = CONFIG.lock() {
            (cfg.zoom.initial_zoom, cfg.zoom.zoom_per_step)
        } else {
            (4.0, 1.5)
        };
        let zoom_step = if let Ok(rt) = ZOOM_RUNTIME.lock() { rt.current_step } else { 0 };
        let cur_factor = (initial_zoom * zoom_per_step.powi(zoom_step)).clamp(1.0, 10.0);

        event.draw.text("§e§l🔍 Zoom", [x + 10.0, y + 32.0], 0xFFFFFFFF, true);
        let zoom_str = format!("Factor: §b{cur_factor:.2}x §7(Step: §f{zoom_step}§7, Base: §f{initial_zoom:.1}x§7)");
        event.draw.text(&zoom_str, [x + 20.0, y + 46.0], 0xFFDDDDDD, true);
        event.draw.text("§8Key: §7C §8| Scroll: §7Adjust Zoom", [x + 20.0, y + 58.0], 0xFFAAAAAA, false);

        // Freecam Info
        let freecam_on = freecam::active();
        let fc_speed = host.setting_f64("freecam_speed").unwrap_or(freecam::DEFAULT_SPEED);
        let fc_status = if freecam_on { "§aActive" } else { "§cDisabled" };

        event.draw.text("§e§l🦅 Freecam", [x + 10.0, y + 76.0], 0xFFFFFFFF, true);
        let fc_str = format!("Status: {fc_status} §7| Speed: §b{fc_speed:.2} §7| NoClip: §aON");
        event.draw.text(&fc_str, [x + 20.0, y + 90.0], 0xFFDDDDDD, true);
        event.draw.text("§8Key: §7U §8| Move: §7WASD/Space/Shift §8| Fast: §7Ctrl", [x + 20.0, y + 102.0], 0xFFAAAAAA, false);

        // Freelock Info
        let freelock_on = FREELOCK_ACTIVE.load(Ordering::Relaxed);
        let fl_status = if freelock_on { "§aActive" } else { "§cDisabled" };

        event.draw.text("§e§l🔒 Freelock", [x + 10.0, y + 120.0], 0xFFFFFFFF, true);
        let fl_str = format!("Status: {fl_status} §7(Decouple camera from player body)");
        event.draw.text(&fl_str, [x + 20.0, y + 134.0], 0xFFDDDDDD, true);
        event.draw.text("§8Key: §7X §8(Press to look around freely)", [x + 20.0, y + 146.0], 0xFFAAAAAA, false);

        // Footer Hint
        event.draw.fill([x + 10.0, y + 162.0, panel_w - 20.0, 1.0], 0x44FFFFFF);
        event.draw.text("§8More settings in: §6Options > Mods > Camera Utils", [x + 15.0, y + 170.0], 0xFF888888, false);
    }

    fn calculate_player_turn(&self, _host: &dyn mod_api::ClientHost, event: &mut mod_api::CalculatePlayerTurnEvent) {
        let freelock = FREELOCK_ACTIVE.load(Ordering::Relaxed);
        let freecam = FREECAM_ACTIVE.load(Ordering::Relaxed);

        if freelock || freecam {
            // Decouple body turning when looking around freely
            event.mouse_sensitivity = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zoom_key() {
        let mut state = CameraState::new();
        state.handle_key("key.keyboard.c", true, [0.0; 3], 0.0, 0.0);
        assert!(state.is_zoomed);
        assert!(ZOOM_PRESSED.load(Ordering::Relaxed));
        assert_eq!(get_zoom_descriptor().unwrap().factor, 4.0);

        state.handle_key("key.keyboard.c", false, [0.0; 3], 0.0, 0.0);
        assert!(!state.is_zoomed);
        assert!(!ZOOM_PRESSED.load(Ordering::Relaxed));
        assert!(get_zoom_descriptor().is_none());
    }

    #[test]
    fn test_freelock() {
        let mut state = CameraState::new();
        let mut p_yaw = 90.0;
        let mut p_pitch = 0.0;

        state.handle_key("key.keyboard.x", true, [0.0; 3], p_yaw, p_pitch);
        assert!(state.is_freelock);

        state.turn_camera(10.0, 5.0, &mut p_yaw, &mut p_pitch);
        assert_eq!(p_yaw, 90.0);
        assert_eq!(state.freelock_yaw, 91.5);
    }

    #[test]
    fn test_freecam() {
        freecam::start([10.0, 20.0, 30.0], 45.0, 15.0, 0);
        assert!(freecam::active());

        let (pos, yaw, pitch) = freecam::view(1.0).unwrap();
        assert_eq!(pos, [10.0, 20.0, 30.0]);
        assert_eq!(yaw, 45.0);
        assert_eq!(pitch, 15.0);

        // Test turn
        assert!(freecam::turn(10.0, 5.0));
        let (_, yaw_turned, pitch_turned) = freecam::view(1.0).unwrap();
        assert!((yaw_turned - (45.0 + 10.0 * 0.15)).abs() < 1e-4);
        assert!((pitch_turned - (15.0 + 5.0 * 0.15)).abs() < 1e-4);

        // Test stop
        let orig_view = freecam::stop();
        assert_eq!(orig_view, Some(0));
        assert!(!freecam::active());
    }

    #[test]
    fn test_zoom_scroll() {
        let mut runtime = ZoomRuntime::default();
        let step_count: i32 = 10;
        let zoom_per_step: f32 = 1.5;
        let initial_zoom: f32 = 4.0;

        // Scroll in (step + 1)
        runtime.current_step = (runtime.current_step + 1).min(step_count);
        let factor_step1 = (initial_zoom * zoom_per_step.powi(runtime.current_step)).max(1.0);
        assert!((factor_step1 - 6.0).abs() < 1e-4);

        // Scroll back to 0 -> should recover exact initial zoom
        runtime.current_step = (runtime.current_step - 1).max(-step_count);
        let factor_step0 = (initial_zoom * zoom_per_step.powi(runtime.current_step)).max(1.0);
        assert_eq!(runtime.current_step, 0);
        assert!((factor_step0 - 4.0).abs() < 1e-4);

        // Scroll down until factor reaches 1.0x (should stop at step where factor > 1.0)
        for _ in 0..20 {
            let current_factor = initial_zoom * zoom_per_step.powi(runtime.current_step);
            if current_factor > 1.0 {
                runtime.current_step = (runtime.current_step - 1).max(-step_count);
            }
        }
        assert_eq!(runtime.current_step, -4);
        let clamped_factor = (initial_zoom * zoom_per_step.powi(runtime.current_step)).max(1.0);
        assert!((clamped_factor - 1.0).abs() < 1e-4);

        // Scroll back up 1 step: immediately restores step -3 (1.185x) with NO dead scrolls
        runtime.current_step = (runtime.current_step + 1).min(step_count);
        assert_eq!(runtime.current_step, -3);
        let factor_back_up = (initial_zoom * zoom_per_step.powi(runtime.current_step)).clamp(1.0, 10.0);
        assert!((factor_back_up - (4.0 / 1.5f32.powi(3))).abs() < 1e-4);

        // Scroll in continuously until factor reaches 10.0x ceiling
        for _ in 0..20 {
            let current_factor = initial_zoom * zoom_per_step.powi(runtime.current_step);
            if current_factor < 10.0 {
                runtime.current_step = (runtime.current_step + 1).min(step_count);
            }
        }
        // At initial=4.0 and per_step=1.5:
        // step 0: 4.0
        // step 1: 6.0
        // step 2: 9.0
        // step 3: 13.5 (current_factor at step 2 was 9.0 < 10.0, so it increased to 3, factor clamped to 10.0)
        // Next attempts won't increase step further because current_factor (13.5) is not < 10.0.
        assert_eq!(runtime.current_step, 3);
        let max_clamped_factor = (initial_zoom * zoom_per_step.powi(runtime.current_step)).clamp(1.0, 10.0);
        assert_eq!(max_clamped_factor, 10.0);

        // Scroll out 1 step from ceiling: immediately drops back to step 2 (9.0x) with NO dead scrolls
        let current_factor = initial_zoom * zoom_per_step.powi(runtime.current_step);
        if current_factor > 1.0 {
            runtime.current_step = (runtime.current_step - 1).max(-step_count);
        }
        assert_eq!(runtime.current_step, 2);
        let factor_drop = (initial_zoom * zoom_per_step.powi(runtime.current_step)).clamp(1.0, 10.0);
        assert!((factor_drop - 9.0).abs() < 1e-4);
    }

    #[test]
    fn test_config_and_gui() {
        let cfg = CameraConfig::default();
        assert_eq!(cfg.zoom.initial_zoom, 4.0);
        assert_eq!(cfg.freecam.flight_speed, 0.5);
        assert_eq!(cfg.freelock.return_speed, 1.0);

        let mut screen = ConfigScreen::new(cfg);
        assert_eq!(screen.active_category, gui::ConfigCategory::Zoom);
        screen.switch_category(gui::ConfigCategory::Freecam);
        assert_eq!(screen.active_category, gui::ConfigCategory::Freecam);

        // Test slider widget logic
        let mut slider = gui::SliderWidget::new("Test", 1.0, 10.0, 0.5, "x", 4.0);
        assert_eq!(slider.normalized(), (4.0 - 1.0) / 9.0);
        slider.set_normalized(1.0);
        assert_eq!(slider.value, 10.0);
        slider.reset();
        assert_eq!(slider.value, 4.0);

        // Test transition easing
        use config::TransitionType;
        assert_eq!(TransitionType::Linear.apply(0.5), 0.5);
        assert!(TransitionType::EaseOutExponential.apply(0.5) > 0.5);
    }
}
