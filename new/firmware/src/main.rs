use std::sync::RwLock;

use esp_idf_sys as _;
use log::*;

mod lamp;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities.
    esp_idf_svc::log::EspLogger::initialize_default();

    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(10));

        let random_brightness = rand::random::<f32>() * 255.0;
        let random_temperature = rand::random::<f32>() * 255.0;

        info!(
            "Setting (random) brightness to {} and temperature to {}°.",
            random_brightness, random_temperature
        );
    });
}
