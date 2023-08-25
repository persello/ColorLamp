use std::{
    fmt::Debug,
    sync::{OnceLock, RwLock},
};

pub static LAMP: OnceLock<RwLock<crate::lamp::Lamp>> = OnceLock::new();

/// A lamp that can be controlled.
pub struct Lamp {
    hue: u16,
    brightness: u8,
    change_callback: Option<Box<dyn Fn(&Lamp) + Send + Sync + 'static>>,
}

impl Lamp {
    /// Create a new Lamp.
    ///
    /// Default hue is 0°, corresponding to red.
    /// Default brightness is 0, corresponding to off.
    pub fn new() -> Self {
        Self {
            hue: 0,
            brightness: 0,
            change_callback: None,
        }
    }

    /// Set the hue of the lamp.
    ///
    /// The hue is a value between 0° and 360°.
    pub fn set_hue(&mut self, hue: u16) {
        self.hue = hue.clamp(0, 360);
        if let Some(callback) = &self.change_callback {
            callback(self);
        }
    }

    /// Set the brightness of the lamp.
    ///
    /// The brightness is a value between 0 and 100%.
    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness.clamp(0, 100);
        if let Some(callback) = &self.change_callback {
            callback(self);
        }
    }

    /// Get the hue of the lamp.
    ///
    /// The hue is a value between 0° and 360°.
    pub fn get_hue(&self) -> u16 {
        self.hue
    }

    /// Get the brightness of the lamp.
    ///
    /// The brightness is a value between 0 and 100%.
    pub fn get_brightness(&self) -> u8 {
        self.brightness
    }

    /// Attach a callback that is called whenever the lamp changes state.
    ///
    /// The callback is called with a reference to the lamp.
    pub fn attach_change_callback(&mut self, callback: impl Fn(&Lamp) + Send + Sync + 'static) {
        self.change_callback = Some(Box::new(callback));
    }
}

impl Debug for Lamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lamp")
            .field("hue", &self.hue)
            .field("brightness", &self.brightness)
            .finish()
    }
}
