use std::sync::{Arc, RwLock};

use bluedroid::{
    gatt_server::{Characteristic, Profile, Service},
    utilities::{Appearance, AttributePermissions, BleUuid, CharacteristicProperties},
};
use esp_idf_sys as _;
use log::*;

mod lamp;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities.
    esp_idf_svc::log::EspLogger::initialize_default();

    let lamp = Arc::new(RwLock::new(lamp::Lamp::new()));

    let lamp_write_brightness_ref = lamp.clone();
    let lamp_read_brightness_ref = lamp.clone();

    let lamp_write_temperature_ref = lamp.clone();
    let lamp_read_temperature_ref = lamp.clone();

    let brightness_characteristic = Characteristic::new(BleUuid::from_uuid128_str(
        "F9DFBD73-0181-433A-8091-372E0CA8A598",
    ))
    .name("Brightness")
    .show_name()
    .max_value_length(1)
    .permissions(AttributePermissions::new().read().write())
    .properties(
        CharacteristicProperties::new()
            .read()
            .write()
            .write_without_response()
            .notify(),
    )
    .on_read(move |_| {
        let brightness = lamp_read_brightness_ref.read().unwrap().get_brightness();
        info!("Read brightness: {}", brightness);
        vec![brightness]
    })
    .on_write(move |val, _| {
        let brightness = val[0];
        info!("Write brightness: {}", brightness);
        lamp_write_brightness_ref
            .write()
            .unwrap()
            .set_brightness(brightness, false);
    })
    .build();

    let temperature_characteristic = Characteristic::new(BleUuid::from_uuid128_str(
        "CA344E9B-7445-43AA-AD20-43A33C8101E9",
    ))
    .name("Brightness")
    .show_name()
    .max_value_length(1)
    .permissions(AttributePermissions::new().read().write())
    .properties(
        CharacteristicProperties::new()
            .read()
            .write()
            .write_without_response()
            .notify(),
    )
    .on_read(move |_| {
        let temperature = lamp_read_temperature_ref.read().unwrap().get_temperature();
        info!("Read brightness: {}", temperature);
        vec![temperature]
    })
    .on_write(move |val, _| {
        let temperature = val[0];
        info!("Write temperature: {}", temperature);
        lamp_write_temperature_ref
            .write()
            .unwrap()
            .set_temperature(temperature, false);
    })
    .build();

    let lamp_service = Service::new(BleUuid::from_uuid128_str(
        "4E0F5E1E-FC5B-4D67-8E30-2A83B336476B",
    ))
    .characteristic(&brightness_characteristic)
    .characteristic(&temperature_characteristic)
    .name("Color Lamp Service")
    .primary()
    .build();

    let main_profile = Profile::new(0)
        .name("Main Profile")
        .service(&lamp_service)
        .build();

    let brightness_for_notify = brightness_characteristic.clone();
    let temperature_for_notify = temperature_characteristic.clone();

    lamp.write()
        .unwrap()
        .attach_change_callback(move |lamp, notify| {
            if notify {
                brightness_for_notify
                    .write()
                    .unwrap()
                    .set_value(vec![lamp.get_brightness()]);

                temperature_for_notify
                    .write()
                    .unwrap()
                    .set_value(vec![lamp.get_temperature()]);
            }
        });

    bluedroid::gatt_server::GLOBAL_GATT_SERVER
        .lock()
        .unwrap()
        .advertise_service(&lamp_service)
        .profile(main_profile)
        .appearance(Appearance::LEDLamp)
        .device_name("Color Lamp")
        .start();

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(10));

        let random_brightness = rand::random::<f32>() * 255.0;
        let random_temperature = rand::random::<f32>() * 255.0;

        info!(
            "Setting (random) brightness to {} and temperature to {}°.",
            random_brightness, random_temperature
        );

        lamp.write()
            .unwrap()
            .set_brightness(random_brightness as u8, true);
        lamp.write()
            .unwrap()
            .set_temperature(random_temperature as u8, true);
    });
}
