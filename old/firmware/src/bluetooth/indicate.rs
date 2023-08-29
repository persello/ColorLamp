use esp_idf_sys::*;

pub fn indicate_lamp_changes(lamp: &crate::lamp::Lamp) {
    let values = crate::bluetooth::CONNECTION_VALUES
        .get()
        .unwrap()
        .read()
        .unwrap()
        .clone();

    if !values.is_connected() {
        return;
    }

    unsafe {
        // These operations are allowed to fail.
        esp_ble_gatts_send_indicate(
            values.get_gatts_if(),
            values.get_conn_id(),
            values.get_brightness_handle(),
            1,
            lamp.get_brightness().to_le_bytes().to_vec().as_ptr() as _,
            false,
        );

        esp_ble_gatts_send_indicate(
            values.get_gatts_if(),
            values.get_conn_id(),
            values.get_temperature_handle(),
            2,
            lamp.get_temperature().to_le_bytes().to_vec().as_ptr() as _,
            false,
        );
    }
}
