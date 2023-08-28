use std::{
    fmt::Debug,
    sync::{OnceLock, RwLock},
};

pub static LAMP: OnceLock<RwLock<crate::lamp::Lamp>> = OnceLock::new();

/// A lamp that can be controlled.
pub struct Lamp {
    temperature: u8,
    brightness: u8,
    change_callback: Option<Box<dyn Fn(&Lamp) + Send + Sync + 'static>>,
}

impl Lamp {
    /// Create a new Lamp.
    ///
    /// Default temperature is 0, corresponding to the warmest white.
    /// Default brightness is 0, corresponding to off.
    pub fn new() -> Self {
        Self {
            temperature: 0,
            brightness: 0,
            change_callback: None,
        }
    }

    /// Set the temperature of the lamp.
    ///
    /// The temperature is a value between 0 and 255.
    pub fn set_temperature(&mut self, temperature: u8) {
        self.temperature = temperature;
        if let Some(callback) = &self.change_callback {
            callback(self);
        }
    }

    /// Set the brightness of the lamp.
    ///
    /// The brightness is a value between 0 and 255.
    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
        if let Some(callback) = &self.change_callback {
            callback(self);
        }
    }

    /// Get the temperature of the lamp.
    ///
    /// The temperature is a value between 0 and 255.
    pub fn get_temperature(&self) -> u8 {
        self.temperature
    }

    /// Get the brightness of the lamp.
    ///
    /// The brightness is a value between 0 and 255.
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
            .field("temperature", &self.temperature)
            .field("brightness", &self.brightness)
            .finish()
    }
}
