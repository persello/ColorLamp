use esp_idf_sys::*;

pub struct Constants {
    pub advertisement_parameters: esp_ble_adv_params_t,
    pub advertisement_data: esp_ble_adv_data_t,
    pub scan_response_data: esp_ble_adv_data_t,
    pub device_name: String,
    pub lamp_service_id: esp_gatt_srvc_id_t,
    pub brightness_char_id: esp_bt_uuid_t,
    pub temperature_char_id: esp_bt_uuid_t,
}

const SERVICE_UUID: [u8; 16] = [
    0x6B, 0x47, 0x36, 0xB3, 0x83, 0x2A, 0x30, 0x8E, 0x67, 0x4D, 0x5B, 0xFC, 0x1E, 0x5E, 0x0F, 0x4E,
];

const BRIGHTNESS_CHAR_UUID: [u8; 16] = [
    0x98, 0xA5, 0xA8, 0x0C, 0x2E, 0x37, 0x91, 0x80, 0x3A, 0x43, 0x81, 0x01, 0x73, 0xBD, 0xDF, 0xF9,
];

const TEMPERATURE_CHAR_UUID: [u8; 16] = [
    0xE9, 0x01, 0x81, 0x3C, 0xA3, 0x43, 0x20, 0xAD, 0xAA, 0x43, 0x45, 0x74, 0x9B, 0x4E, 0x34, 0xCA,
];

impl Default for Constants {
    fn default() -> Self {
        Self {
            advertisement_parameters: esp_ble_adv_params_t {
                adv_int_min: 0x20,
                adv_int_max: 0x40,
                adv_type: esp_ble_adv_type_t_ADV_TYPE_IND,
                own_addr_type: esp_ble_addr_type_t_BLE_ADDR_TYPE_RPA_PUBLIC,
                channel_map: esp_ble_adv_channel_t_ADV_CHNL_ALL,
                adv_filter_policy: esp_ble_adv_filter_t_ADV_FILTER_ALLOW_SCAN_ANY_CON_ANY,
                ..Default::default()
            },
            advertisement_data: esp_ble_adv_data_t {
                set_scan_rsp: false,
                include_name: true,
                include_txpower: true,
                min_interval: 0x0006,
                max_interval: 0x0010,
                appearance: 0x0597, /* Bulb */
                manufacturer_len: 0,
                p_manufacturer_data: std::ptr::null_mut(),
                service_data_len: 0,
                p_service_data: std::ptr::null_mut(),
                service_uuid_len: 0,
                p_service_uuid: std::ptr::null_mut(),
                flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
            },
            scan_response_data: esp_ble_adv_data_t {
                set_scan_rsp: true,
                include_name: false,
                include_txpower: false,
                min_interval: 0x0006,
                max_interval: 0x0010,
                appearance: 0x0597, /* Bulb */
                manufacturer_len: 0,
                p_manufacturer_data: std::ptr::null_mut(),
                service_data_len: 0,
                p_service_data: std::ptr::null_mut(),
                service_uuid_len: 16,
                p_service_uuid: SERVICE_UUID.as_ptr() as *mut u8,
                flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
            },
            device_name: "Color Lamp".to_string(),
            lamp_service_id: esp_gatt_srvc_id_t {
                id: esp_gatt_id_t {
                    uuid: esp_bt_uuid_t {
                        len: ESP_UUID_LEN_128 as u16,
                        uuid: esp_bt_uuid_t__bindgen_ty_1 {
                            uuid128: SERVICE_UUID,
                        },
                    },
                    inst_id: 0,
                },
                is_primary: true,
            },
            brightness_char_id: esp_bt_uuid_t {
                len: ESP_UUID_LEN_128 as u16,
                uuid: esp_bt_uuid_t__bindgen_ty_1 {
                    uuid128: BRIGHTNESS_CHAR_UUID,
                },
            },
            temperature_char_id: esp_bt_uuid_t {
                len: ESP_UUID_LEN_128 as u16,
                uuid: esp_bt_uuid_t__bindgen_ty_1 {
                    uuid128: TEMPERATURE_CHAR_UUID,
                },
            },
        }
    }
}
