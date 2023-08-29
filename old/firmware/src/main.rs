use std::sync::RwLock;

use esp_idf_sys as _;
use log::*;

mod bluetooth;
mod lamp;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities.
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initalise the lamp object.
    if crate::lamp::LAMP
        .set(RwLock::new(crate::lamp::Lamp::new()))
        .is_err()
    {
        panic!("Failed to set lamp object.");
    }

    crate::lamp::LAMP
        .get()
        .unwrap()
        .write()
        .unwrap()
        .attach_change_callback(|lamp, notify| {
            // Indicate change.
            if notify {
                crate::bluetooth::indicate_lamp_changes(lamp);
            }
        });

    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(10));

        let random_brightness = rand::random::<f32>() * 255.0;
        let random_temperature = rand::random::<f32>() * 255.0;

        info!(
            "[MAIN] Setting (random) brightness to {} and temperature to {}°.",
            random_brightness, random_temperature
        );

        crate::lamp::LAMP
            .get()
            .unwrap()
            .write()
            .unwrap()
            .set_brightness(random_brightness as u8, true);

        crate::lamp::LAMP
            .get()
            .unwrap()
            .write()
            .unwrap()
            .set_temperature(random_temperature as u8, true);
    });

    // Start the Bluetooth stack.
    unsafe {
        bluetooth::initialise_bluetooth();
    }
}
