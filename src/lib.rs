// camera-utils runtime for Horizon (Freecam, Freelock, Zoom)
// Standalone testable Cargo library and injected crate::game::camera_utils module.

#[cfg(not(test))]
#[derive(Default)]
pub struct CameraUtilsMod;

#[cfg(not(test))]
#[mod_api::horizon_mod]
impl mod_api::Hooks for CameraUtilsMod {}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub static ZOOM_PRESSED: AtomicBool = AtomicBool::new(false);
pub static FREELOCK_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static FREECAM_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoomDescriptor {
    pub factor: f32,
    pub smooth: bool,
    pub turn: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CameraConfig {
    pub zoom_fov_multiplier: f32,
    pub zoom_turn_multiplier: f32,
    pub freecam_speed: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            zoom_fov_multiplier: 4.0, // 4x zoom
            zoom_turn_multiplier: 0.25,
            freecam_speed: 0.5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CameraState {
    pub is_zoomed: bool,

    // Freelock
    pub is_freelock: bool,
    pub freelock_yaw: f32,
    pub freelock_pitch: f32,

    // Freecam
    pub is_freecam: bool,
    pub freecam_pos: [f64; 3],
    pub freecam_yaw: f32,
    pub freecam_pitch: f32,
}

impl CameraState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key(&mut self, key: &str, pressed: bool, current_pos: [f64; 3], current_yaw: f32, current_pitch: f32) {
        match key {
            // Key C for Zoom (Hold to zoom)
            "key.keyboard.c" => {
                self.is_zoomed = pressed;
                ZOOM_PRESSED.store(pressed, Ordering::Relaxed);
            }
            // Key X for Freelock (Toggle)
            "key.keyboard.x" if pressed => {
                self.is_freelock = !self.is_freelock;
                if self.is_freelock {
                    self.freelock_yaw = current_yaw;
                    self.freelock_pitch = current_pitch;
                }
                FREELOCK_ACTIVE.store(self.is_freelock, Ordering::Relaxed);
            }
            // Key U for Freecam (Toggle)
            "key.keyboard.u" if pressed => {
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

    pub fn move_freecam(&mut self, forward: f64, strafe: f64, up: f64, speed: f64) {
        if !self.is_freecam {
            return;
        }
        let rad_yaw = (self.freecam_yaw as f64).to_radians();
        let sin_yaw = rad_yaw.sin();
        let cos_yaw = rad_yaw.cos();

        self.freecam_pos[0] += (-sin_yaw * forward + cos_yaw * strafe) * speed;
        self.freecam_pos[1] += up * speed;
        self.freecam_pos[2] += (cos_yaw * forward + sin_yaw * strafe) * speed;
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

// Global state accessed safely with Mutex
static GLOBAL_CAMERA: Mutex<CameraState> = Mutex::new(CameraState {
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
        Some(ZoomDescriptor {
            factor: 4.0,
            smooth: true,
            turn: 0.25,
        })
    } else {
        None
    }
}

pub fn on_native_key(key: &str, pressed: bool, current_pos: [f64; 3], current_yaw: f32, current_pitch: f32) {
    if let Ok(mut cam) = GLOBAL_CAMERA.lock() {
        cam.handle_key(key, pressed, current_pos, current_yaw, current_pitch);
    }
}

pub fn is_freelock_or_freecam() -> bool {
    FREELOCK_ACTIVE.load(Ordering::Relaxed) || FREECAM_ACTIVE.load(Ordering::Relaxed)
}

pub fn on_turn_input(xo: f64, yo: f64) {
    if let Ok(mut cam) = GLOBAL_CAMERA.lock() {
        let mut dummy_yaw = 0.0;
        let mut dummy_pitch = 0.0;
        cam.turn_camera(xo, yo, &mut dummy_yaw, &mut dummy_pitch);
    }
}

pub fn is_freecam_active() -> bool {
    FREECAM_ACTIVE.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zoom_key_handling() {
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
    fn test_freelock_turning() {
        let mut state = CameraState::new();
        let mut player_yaw = 90.0;
        let mut player_pitch = 0.0;

        // Toggle freelock on
        state.handle_key("key.keyboard.x", true, [0.0; 3], player_yaw, player_pitch);
        assert!(state.is_freelock);
        assert_eq!(state.freelock_yaw, 90.0);

        // Turn camera when freelock is on
        state.turn_camera(10.0, 5.0, &mut player_yaw, &mut player_pitch);

        // Player look unchanged
        assert_eq!(player_yaw, 90.0);
        assert_eq!(player_pitch, 0.0);

        // Camera look changed
        assert_eq!(state.freelock_yaw, 91.5);
        assert_eq!(state.freelock_pitch, 0.75);
    }

    #[test]
    fn test_freecam_view_override() {
        let mut state = CameraState::new();
        state.handle_key("key.keyboard.u", true, [100.0, 64.0, 100.0], 45.0, 10.0);
        assert!(state.is_freecam);

        let (cam_pos, cam_yaw, cam_pitch) =
            state.override_camera_view([0.0, 64.0, 0.0], 0.0, 0.0);

        assert_eq!(cam_pos, [100.0, 64.0, 100.0]);
        assert_eq!(cam_yaw, 45.0);
        assert_eq!(cam_pitch, 10.0);
    }
}
