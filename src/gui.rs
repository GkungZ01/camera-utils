//! UI widgets and layout for Camera Utils Config Screen (YACL style).

use crate::config::{CameraConfig, TransitionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigCategory {
    Zoom,
    Freecam,
    Freelock,
}

impl ConfigCategory {
    pub const ALL: [ConfigCategory; 3] = [
        ConfigCategory::Zoom,
        ConfigCategory::Freecam,
        ConfigCategory::Freelock,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Zoom => "Zoom",
            Self::Freecam => "Freecam",
            Self::Freelock => "Freelock",
        }
    }
}

/// Slider widget state and logic.
#[derive(Debug, Clone, PartialEq)]
pub struct SliderWidget {
    pub label: &'static str,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub unit: &'static str,
    pub value: f32,
    pub default_value: f32,
}

impl SliderWidget {
    pub fn new(label: &'static str, min: f32, max: f32, step: f32, unit: &'static str, value: f32) -> Self {
        Self {
            label,
            min,
            max,
            step,
            unit,
            value,
            default_value: value,
        }
    }

    pub fn set_normalized(&mut self, norm: f32) {
        let norm = norm.clamp(0.0, 1.0);
        let raw = self.min + (self.max - self.min) * norm;
        if self.step > 0.0 {
            self.value = (raw / self.step).round() * self.step;
        } else {
            self.value = raw;
        }
    }

    pub fn normalized(&self) -> f32 {
        if self.max <= self.min {
            0.0
        } else {
            ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default_value;
    }
}

/// Toggle / Checkbox widget state.
#[derive(Debug, Clone, PartialEq)]
pub struct ToggleWidget {
    pub label: &'static str,
    pub value: bool,
    pub default_value: bool,
}

impl ToggleWidget {
    pub fn new(label: &'static str, value: bool) -> Self {
        Self {
            label,
            value,
            default_value: value,
        }
    }

    pub fn toggle(&mut self) {
        self.value = !self.value;
    }

    pub fn reset(&mut self) {
        self.value = self.default_value;
    }
}

/// Selector dropdown widget for transitions.
#[derive(Debug, Clone, PartialEq)]
pub struct TransitionSelectorWidget {
    pub label: &'static str,
    pub selected: TransitionType,
    pub default_value: TransitionType,
}

impl TransitionSelectorWidget {
    pub fn new(label: &'static str, selected: TransitionType) -> Self {
        Self {
            label,
            selected,
            default_value: selected,
        }
    }

    pub fn cycle_next(&mut self) {
        self.selected = match self.selected {
            TransitionType::Linear => TransitionType::EaseOutExponential,
            TransitionType::EaseOutExponential => TransitionType::EaseInOutCubic,
            TransitionType::EaseInOutCubic => TransitionType::Linear,
        };
    }

    pub fn reset(&mut self) {
        self.selected = self.default_value;
    }
}

/// YACL-style Screen Layout representation for rendering & interactions.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigScreen {
    pub active_category: ConfigCategory,
    pub is_open: bool,
    pub config: CameraConfig,
}

impl ConfigScreen {
    pub fn new(config: CameraConfig) -> Self {
        Self {
            active_category: ConfigCategory::Zoom,
            is_open: false,
            config,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn switch_category(&mut self, cat: ConfigCategory) {
        self.active_category = cat;
    }

    pub fn reset_current_category(&mut self) {
        match self.active_category {
            ConfigCategory::Zoom => self.config.reset_zoom(),
            ConfigCategory::Freecam => self.config.reset_freecam(),
            ConfigCategory::Freelock => self.config.reset_freelock(),
        }
    }
}
