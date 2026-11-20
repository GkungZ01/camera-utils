// Camera utilities implementation for Horizon (Freecam, Freelock, Zoom)
pub mod config;
pub mod gui;

pub use config::CameraConfig;
pub use gui::ConfigScreen;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Default)]
pub struct CameraUtilsMod;

pub static ZOOM_PRESSED: AtomicBool = AtomicBool::new(false);
pub static ZOOM_TOGGLED: AtomicBool = AtomicBool::new(false);
pub static FREELOCK_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static FREECAM_ACTIVE: AtomicBool = AtomicBool::new(false);

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

        // Freecam Category
        let _ = r.register_setting(
            mod_api::SettingSpec::range(
                "freecam_speed",
                "[Freecam] Flight Speed",
                0.1,
                5.0,
                0.1,
                0.5,
            )
            .tooltip("Detached camera flight speed"),
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
                        let factor = (initial_zoom * zoom_per_step.powi(step)).max(1.0);
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

        let freelock_clicked = host.consume_click("camera_utils", "freelock");
        let freecam_clicked = host.consume_click("camera_utils", "freecam");

        if freelock_clicked || freecam_clicked {
            if let Ok(mut state) = GLOBAL_CAMERA.lock() {
                if freelock_clicked {
                    state.handle_key("freelock", true, pos, yaw, pitch);
                }
                if freecam_clicked {
                    state.handle_key("freecam", true, pos, yaw, pitch);
                }
            }
        }
    }

    fn input_key(&self, host: &mut dyn mod_api::ClientHost, event: &mut mod_api::InputKeyEvent) {
        let pressed = match event.action {
            mod_api::KeyAction::Press => true,
            mod_api::KeyAction::Release => false,
            _ => return,
        };

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
                    runtime.current_step = (runtime.current_step + 1).min(step_count);
                } else {
                    // Prevent decreasing step further if factor is already <= 1.0
                    let current_factor = initial_zoom * zoom_per_step.powi(runtime.current_step);
                    if current_factor > 1.0 {
                        runtime.current_step = (runtime.current_step - 1).max(-step_count);
                    }
                }

                let factor = (initial_zoom * zoom_per_step.powi(runtime.current_step)).max(1.0);
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
        let factor = (initial_zoom * zoom_per_step.powi(step)).max(1.0);

        // Relative sensitivity: inversely proportional to factor, scaled by relative_sens %
        // If factor <= 1.0, keep full sensitivity (1.0) so camera movement is never restricted
        let turn = if factor <= 1.001 {
            1.0
        } else {
            let base_turn = 1.0 / factor;
            (base_turn * (relative_sens / 100.0)).clamp(0.01, 2.0)
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
        let freelock = FREELOCK_ACTIVE.load(Ordering::Relaxed);
        let freecam = FREECAM_ACTIVE.load(Ordering::Relaxed);

        if freelock || freecam {
            if let Ok(state) = GLOBAL_CAMERA.lock() {
                let (new_pos, new_yaw, new_pitch) =
                    state.override_camera_view(*event.position, event.yaw, event.pitch);
                *event.position = new_pos;
                event.yaw = new_yaw;
                event.pitch = new_pitch;
            }
        }
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
        let mut state = CameraState::new();
        state.handle_key("key.keyboard.u", true, [10.0, 20.0, 30.0], 45.0, 15.0);
        assert!(state.is_freecam);

        let (pos, yaw, pitch) = state.override_camera_view([0.0; 3], 0.0, 0.0);
        assert_eq!(pos, [10.0, 20.0, 30.0]);
        assert_eq!(yaw, 45.0);
        assert_eq!(pitch, 15.0);
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
        let factor_back_up = (initial_zoom * zoom_per_step.powi(runtime.current_step)).max(1.0);
        assert!((factor_back_up - (4.0 / 1.5f32.powi(3))).abs() < 1e-4);

        // Scroll up to max step 10 -> factor must not be capped at 50.0x
        runtime.current_step = step_count;
        let factor_step10 = (initial_zoom * zoom_per_step.powi(runtime.current_step)).max(1.0);
        let expected_step10 = 4.0 * 1.5f32.powi(10);
        assert!((factor_step10 - expected_step10).abs() < 1e-3);
        assert!(factor_step10 > 200.0);
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
