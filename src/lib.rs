// Camera utilities implementation for Horizon (Freecam, Freelock, Zoom)
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
            zoom_fov_multiplier: 4.0,
            zoom_turn_multiplier: 0.25,
            freecam_speed: 0.5,
        }
    }
}

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
            "key.keyboard.c" => {
                self.is_zoomed = pressed;
                ZOOM_PRESSED.store(pressed, Ordering::Relaxed);
            }
            "key.keyboard.x" if pressed => {
                self.is_freelock = !self.is_freelock;
                if self.is_freelock {
                    self.freelock_yaw = current_yaw;
                    self.freelock_pitch = current_pitch;
                }
                FREELOCK_ACTIVE.store(self.is_freelock, Ordering::Relaxed);
            }
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
        Some(ZoomDescriptor {
            factor: 4.0,
            smooth: true,
            turn: 0.25,
        })
    } else {
        None
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
}
