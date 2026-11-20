//! Configuration definitions and persistence for Camera Utils.

/// Easing transitions for Zoom In / Zoom Out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransitionType {
    Linear,
    #[default]
    EaseOutExponential,
    EaseInOutCubic,
}

impl TransitionType {
    pub const ALL: [TransitionType; 3] = [
        TransitionType::Linear,
        TransitionType::EaseOutExponential,
        TransitionType::EaseInOutCubic,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::EaseOutExponential => "Ease Out Exponential",
            Self::EaseInOutCubic => "Ease In-Out Cubic",
        }
    }

    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseOutExponential => {
                if t >= 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
        }
    }
}

/// Zoom category settings (inspired by Zoomify).
#[derive(Debug, Clone, PartialEq)]
pub struct ZoomSettings {
    pub initial_zoom: f32,
    pub zoom_in_time: f32,
    pub zoom_out_time: f32,
    pub zoom_in_transition: TransitionType,
    pub zoom_out_transition: TransitionType,
    pub turn_multiplier: f32,
    pub enable_scroll_zoom: bool,
    pub zoom_per_step: f32,
    pub scroll_step_count: u32,
    pub scroll_smoothness: f32,
}

impl Default for ZoomSettings {
    fn default() -> Self {
        Self {
            initial_zoom: 4.0,
            zoom_in_time: 1.0,
            zoom_out_time: 0.5,
            zoom_in_transition: TransitionType::EaseOutExponential,
            zoom_out_transition: TransitionType::EaseOutExponential,
            turn_multiplier: 0.25,
            enable_scroll_zoom: true,
            zoom_per_step: 1.5,
            scroll_step_count: 10,
            scroll_smoothness: 0.7,
        }
    }
}

/// Freecam category settings.
#[derive(Debug, Clone, PartialEq)]
pub struct FreecamSettings {
    pub flight_speed: f64,
    pub fast_modifier: f64,
    pub slow_modifier: f64,
    pub smooth_camera: bool,
    pub inertia: f64,
    pub show_player_model: bool,
    pub freeze_player_movement: bool,
}

impl Default for FreecamSettings {
    fn default() -> Self {
        Self {
            flight_speed: 0.5,
            fast_modifier: 2.5,
            slow_modifier: 0.2,
            smooth_camera: true,
            inertia: 0.8,
            show_player_model: true,
            freeze_player_movement: true,
        }
    }
}

/// Freelock category settings.
#[derive(Debug, Clone, PartialEq)]
pub struct FreelockSettings {
    pub toggle_mode: bool,
    pub return_speed: f32,
    pub sensitivity_ratio: f32,
    pub perspective_lock: bool,
}

impl Default for FreelockSettings {
    fn default() -> Self {
        Self {
            toggle_mode: true,
            return_speed: 1.0,
            sensitivity_ratio: 1.0,
            perspective_lock: false,
        }
    }
}

/// Consolidated Camera Utils Configuration.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CameraConfig {
    pub zoom: ZoomSettings,
    pub freecam: FreecamSettings,
    pub freelock: FreelockSettings,
}

impl CameraConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = ZoomSettings::default();
    }

    pub fn reset_freecam(&mut self) {
        self.freecam = FreecamSettings::default();
    }

    pub fn reset_freelock(&mut self) {
        self.freelock = FreelockSettings::default();
    }

    pub fn reset_all(&mut self) {
        *self = Self::default();
    }
}
