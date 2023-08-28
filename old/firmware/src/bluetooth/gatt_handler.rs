use esp_idf_sys::*;
use log::*;

pub extern "C" fn gatts_event_handler(
    event: esp_gatts_cb_event_t,
    gatts_if: esp_gatt_if_t,
    param: *mut esp_ble_gatts_cb_param_t,
) {
    let mut constants = crate::bluetooth::constants::Constants::default();

    #[allow(non_upper_case_globals)]
    match event {
        esp_gatts_cb_event_t_ESP_GATTS_CONNECT_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_connect_evt_param =
                unsafe { (*param).connect };

            info!(
                "[GATTS] GATT client connected: {:?}",
                param.remote_bda.to_vec()
            );

            let lock = crate::bluetooth::CONNECTION_VALUES.get().unwrap();
            let mut values = lock.write().unwrap();

            values.set_gatts_if(gatts_if);
            values.set_conn_id(param.conn_id);
        }

        esp_gatts_cb_event_t_ESP_GATTS_DISCONNECT_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_disconnect_evt_param =
                unsafe { (*param).disconnect };

            info!(
                "[GATTS] GATT client disconnected: {:?}",
                param.remote_bda.to_vec()
            );

            unsafe {
                esp_nofail!(esp_ble_gap_start_advertising(
                    &mut constants.advertisement_parameters
                ));
            }

            let lock = crate::bluetooth::CONNECTION_VALUES.get().unwrap();
            let mut values = lock.write().unwrap();

            values.set_gatts_if(ESP_GATT_IF_NONE as u8);
            values.set_conn_id(u16::MAX);
        }

        esp_gatts_cb_event_t_ESP_GATTS_REG_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_reg_evt_param = unsafe { (*param).reg };

            info!(
                "[GATTS] GATT application/profile registered: {}",
                param.app_id
            );

            info!("[GATTS] Registering \"Color Lamp\" service.");
            crate::bluetooth::register_lamp_service(gatts_if);
        }

        esp_gatts_cb_event_t_ESP_GATTS_CREATE_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_create_evt_param = unsafe { (*param).create };

            info!(
                "[GATTS] Service created with handle {}.",
                param.service_handle
            );
            info!(
                "[GATTS] Starting service with handle {}.",
                param.service_handle
            );

            crate::bluetooth::start_lamp_service(param.service_handle);
        }

        esp_gatts_cb_event_t_ESP_GATTS_START_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_start_evt_param = unsafe { (*param).start };

            info!(
                "[GATTS] Service started with handle {}.",
                param.service_handle
            );
            info!(
                "[GATTS] Registering brightness characteristic for service with handle {}.",
                param.service_handle
            );

            crate::bluetooth::register_brightness_characteristic(param.service_handle);
        }

        esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_add_char_evt_param =
                unsafe { (*param).add_char };

            info!(
                "[GATTS] Characteristic added with handle {}.",
                param.attr_handle
            );

            // Keep track of the handles.
            unsafe {
                if param.char_uuid.uuid.uuid128
                    == crate::bluetooth::constants::Constants::default()
                        .brightness_char_id
                        .uuid
                        .uuid128
                {
                    crate::bluetooth::CONNECTION_VALUES
                        .get()
                        .unwrap()
                        .write()
                        .unwrap()
                        .set_brightness_handle(param.attr_handle);

                    info!("[GATTS] Registered brightness characteristic.");

                    // Register temperature characteristic.
                    info!(
                        "[GATTS] Registering temperature characteristic for service with handle {}.",
                        param.service_handle
                    );
                    crate::bluetooth::register_temperature_characteristic(param.service_handle);
                } else if param.char_uuid.uuid.uuid128
                    == crate::bluetooth::constants::Constants::default()
                        .color_char_id
                        .uuid
                        .uuid128
                {
                    crate::bluetooth::CONNECTION_VALUES
                        .get()
                        .unwrap()
                        .write()
                        .unwrap()
                        .set_temperature_handle(param.attr_handle);

                    info!("[GATTS] Registered color characteristic.");
                }
            }

            // Add the CCCD.

            let mut cccd_id = esp_bt_uuid_t {
                len: ESP_UUID_LEN_16 as u16,
                uuid: esp_bt_uuid_t__bindgen_ty_1 {
                    uuid16: ESP_GATT_UUID_CHAR_CLIENT_CONFIG as u16,
                },
            };

            let cccd_perm: esp_gatt_perm_t = (ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE) as u16;

            let mut cccd_value: esp_attr_value_t = esp_attr_value_t {
                attr_len: 2,
                attr_max_len: 2,
                attr_value: Box::into_raw(Box::new([0x00, 0x00])).cast::<u8>(),
            };

            let mut cccd_control: esp_attr_control_t = esp_attr_control_t {
                auto_rsp: ESP_GATT_AUTO_RSP as u8,
            };

            unsafe {
                esp_nofail!(esp_ble_gatts_add_char_descr(
                    param.service_handle,
                    &mut cccd_id,
                    cccd_perm,
                    &mut cccd_value,
                    &mut cccd_control,
                ));
            }
        }

        esp_gatts_cb_event_t_ESP_GATTS_READ_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_read_evt_param = unsafe { (*param).read };

            info!("[GATTS] Read characteristic with handle {}.", param.handle);

            let value = if param.handle
                == crate::bluetooth::CONNECTION_VALUES
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_brightness_handle()
            {
                let brightness = crate::lamp::LAMP
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_brightness();

                info!("[GATTS] Brightness is {}%.", brightness);

                brightness.to_le_bytes().to_vec()
            } else if param.handle
                == crate::bluetooth::CONNECTION_VALUES
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_temperature_handle()
            {
                let temperature = crate::lamp::LAMP
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_temperature();

                info!("[GATTS] Temperature is {}°.", temperature);

                temperature.to_le_bytes().to_vec()
            } else {
                vec![]
            };

            let mut response = [0u8; 600];
            response[..value.len()].copy_from_slice(&value);

            let mut esp_rsp = esp_gatt_rsp_t {
                attr_value: esp_gatt_value_t {
                    auth_req: 0,
                    handle: param.handle,
                    len: value.len() as u16,
                    offset: 0,
                    value: response,
                },
            };

            unsafe {
                esp_nofail!(esp_ble_gatts_send_response(
                    gatts_if,
                    param.conn_id,
                    param.trans_id,
                    // TODO: Allow different statuses.
                    esp_gatt_status_t_ESP_GATT_OK,
                    &mut esp_rsp
                ));
            }
        }

        esp_gatts_cb_event_t_ESP_GATTS_WRITE_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_write_evt_param = unsafe { (*param).write };

            info!(
                "[GATTS] Write characteristic with handle {}. Needs response: {}.",
                param.handle, param.need_rsp
            );

            let value = unsafe { std::slice::from_raw_parts(param.value, param.len as usize) };
            let mut buffer = [0u8; 1];

            if value.len() > buffer.len() {
                warn!("[GATTS] Value is too long, truncating to 1 byte.");
                buffer[..].copy_from_slice(&value[..1]);
            } else {
                buffer[..value.len()].copy_from_slice(value);
            }

            info!("[GATTS] Received value: {:?}.", value);

            let result_buffer = if param.handle
                == crate::bluetooth::CONNECTION_VALUES
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_brightness_handle()
            {
                let brightness = buffer[0];

                info!("[GATTS] Setting brightness to {}%.", brightness);

                crate::lamp::LAMP
                    .get()
                    .unwrap()
                    .write()
                    .unwrap()
                    .set_brightness(brightness);

                crate::lamp::LAMP
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_brightness()
                    .to_le_bytes()
                    .to_vec()
            } else if param.handle
                == crate::bluetooth::CONNECTION_VALUES
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_temperature_handle()
            {
                let temperature = u8::from_le_bytes(buffer);

                info!("[GATTS] Setting temperature to {}°.", temperature);

                crate::lamp::LAMP
                    .get()
                    .unwrap()
                    .write()
                    .unwrap()
                    .set_temperature(temperature);

                crate::lamp::LAMP
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get_temperature()
                    .to_le_bytes()
                    .to_vec()
            } else {
                vec![]
            };

            if param.need_rsp {
                let mut response = [0u8; 600];
                response[..result_buffer.len()].copy_from_slice(&result_buffer);

                let mut esp_rsp = esp_gatt_rsp_t {
                    attr_value: esp_gatt_value_t {
                        auth_req: 0,
                        handle: param.handle,
                        len: value.len() as u16,
                        offset: 0,
                        value: response,
                    },
                };

                unsafe {
                    esp_nofail!(esp_ble_gatts_send_response(
                        gatts_if,
                        param.conn_id,
                        param.trans_id,
                        // TODO: Allow different statuses.
                        esp_gatt_status_t_ESP_GATT_OK,
                        &mut esp_rsp
                    ));
                }
            }
        }

        esp_gatts_cb_event_t_ESP_GATTS_RESPONSE_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_rsp_evt_param = unsafe { (*param).rsp };

            info!(
                "[GATTS] Response sent for attribute with handle {}.",
                param.handle
            );
        }

        esp_gatts_cb_event_t_ESP_GATTS_SET_ATTR_VAL_EVT => {
            let param: esp_ble_gatts_cb_param_t_gatts_set_attr_val_evt_param =
                unsafe { (*param).set_attr_val };

            info!(
                "[GATTS] Attribute value set for attribute with handle {}.",
                param.attr_handle
            );
        }

        _ => {}
    }
}
