use std::sync::{OnceLock, RwLock};

use esp_idf_sys::*;

use connection_values::ConnectionValues;

mod connection_values;
mod constants;
mod gap_handler;
mod gatt_handler;
mod indicate;

pub use indicate::indicate_lamp_changes;

static CONNECTION_VALUES: OnceLock<RwLock<ConnectionValues>> = OnceLock::new();

pub unsafe fn initialise_bluetooth() {
    // Initialise the connection value store.
    if CONNECTION_VALUES
        .set(RwLock::new(ConnectionValues::new()))
        .is_err()
    {
        panic!("Failed to initialise the connection value store.");
    }

    // Initialisation of the Non-Volatile Storage for reading RF calibration data.
    esp_nofail!(nvs_flash_init());

    // Release memory used by the "classic" Bluetooth controller, as we are only using BLE.
    esp_nofail!(esp_bt_controller_mem_release(
        esp_bt_mode_t_ESP_BT_MODE_CLASSIC_BT
    ));

    // Initialise the Bluetooth controller with the default configuration.
    #[cfg(esp32)]
    let mut default_controller_configuration = esp_bt_controller_config_t {
        controller_task_stack_size: ESP_TASK_BT_CONTROLLER_STACK as _,
        controller_task_prio: ESP_TASK_BT_CONTROLLER_PRIO as _,
        hci_uart_no: BT_HCI_UART_NO_DEFAULT as _,
        hci_uart_baudrate: BT_HCI_UART_BAUDRATE_DEFAULT,
        scan_duplicate_mode: SCAN_DUPLICATE_MODE as _,
        scan_duplicate_type: SCAN_DUPLICATE_TYPE_VALUE as _,
        normal_adv_size: NORMAL_SCAN_DUPLICATE_CACHE_SIZE as _,
        mesh_adv_size: MESH_DUPLICATE_SCAN_CACHE_SIZE as _,
        send_adv_reserved_size: SCAN_SEND_ADV_RESERVED_SIZE as _,
        controller_debug_flag: CONTROLLER_ADV_LOST_DEBUG_BIT,
        mode: esp_bt_mode_t_ESP_BT_MODE_BLE as _,
        ble_max_conn: CONFIG_BTDM_CTRL_BLE_MAX_CONN_EFF as _,
        bt_max_acl_conn: CONFIG_BTDM_CTRL_BR_EDR_MAX_ACL_CONN_EFF as _,
        bt_sco_datapath: CONFIG_BTDM_CTRL_BR_EDR_SCO_DATA_PATH_EFF as _,
        auto_latency: BTDM_CTRL_AUTO_LATENCY_EFF != 0,
        bt_legacy_auth_vs_evt: BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF != 0,
        bt_max_sync_conn: CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF as _,
        ble_sca: CONFIG_BTDM_BLE_SLEEP_CLOCK_ACCURACY_INDEX_EFF as _,
        pcm_role: CONFIG_BTDM_CTRL_PCM_ROLE_EFF as _,
        pcm_polar: CONFIG_BTDM_CTRL_PCM_POLAR_EFF as _,
        hli: BTDM_CTRL_HLI != 0,
        magic: ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL,
        #[cfg(any(esp_idf_version = "5.0", esp_idf_version = "5.1"))]
        dup_list_refresh_period: SCAN_DUPL_CACHE_REFRESH_PERIOD as u16,
    };

    // Initialise and enable the controller.
    esp_nofail!(esp_bt_controller_init(
        &mut default_controller_configuration
    ));

    esp_nofail!(esp_bt_controller_enable(esp_bt_mode_t_ESP_BT_MODE_BLE));

    // Initialise and enable the Bluedroid stack.
    esp_nofail!(esp_bluedroid_init());
    esp_nofail!(esp_bluedroid_enable());

    // Attach GAP and GATT event handlers.
    esp_nofail!(esp_ble_gap_register_callback(Some(
        gap_handler::gap_event_handler
    )));

    esp_nofail!(esp_ble_gatts_register_callback(Some(
        gatt_handler::gatts_event_handler
    )));

    let mut constants = constants::Constants::default();

    // Set device name.
    esp_nofail!(esp_ble_gap_set_device_name(
        constants.device_name.as_ptr().cast::<i8>()
    ));

    // Configure advertisement data.
    esp_nofail!(esp_ble_gap_config_adv_data(
        &mut constants.advertisement_data
    ));

    esp_nofail!(esp_ble_gap_config_adv_data(
        &mut constants.scan_response_data
    ));

    // Register the main profile.
    esp_nofail!(esp_ble_gatts_app_register(0));
}

pub fn register_lamp_service(gatts_if: esp_gatt_if_t) {
    let mut service_id = constants::Constants::default().lamp_service_id;

    // Register the "Color Lamp" service with 5 free handles.
    unsafe {
        esp_nofail!(esp_ble_gatts_create_service(gatts_if, &mut service_id, 5));
    }
}

pub fn start_lamp_service(service_handle: u16) {
    unsafe {
        esp_nofail!(esp_ble_gatts_start_service(service_handle));
    }
}

pub fn register_brightness_characteristic(service_handle: u16) {
    let mut constants = constants::Constants::default();

    unsafe {
        esp_nofail!(esp_ble_gatts_add_char(
            service_handle,
            &mut constants.brightness_char_id,
            (ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE) as u16,
            (ESP_GATT_CHAR_PROP_BIT_READ
                | ESP_GATT_CHAR_PROP_BIT_WRITE
                | ESP_GATT_CHAR_PROP_BIT_NOTIFY) as u8,
            &mut esp_attr_value_t {
                attr_len: 1,
                attr_max_len: 1,
                attr_value: Box::into_raw(Box::new([0x00])).cast::<u8>(),
            },
            &mut esp_attr_control_t {
                auto_rsp: ESP_GATT_RSP_BY_APP as u8,
            }
        ));
    }
}

pub fn register_temperature_characteristic(service_handle: u16) {
    let mut constants = constants::Constants::default();

    unsafe {
        esp_nofail!(esp_ble_gatts_add_char(
            service_handle,
            &mut constants.color_char_id,
            (ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE) as u16,
            (ESP_GATT_CHAR_PROP_BIT_READ
                | ESP_GATT_CHAR_PROP_BIT_WRITE
                | ESP_GATT_CHAR_PROP_BIT_NOTIFY) as u8,
            &mut esp_attr_value_t {
                attr_len: 2,
                attr_max_len: 2,
                attr_value: Box::into_raw(Box::new([0x00, 0x00])).cast::<u8>(),
            },
            &mut esp_attr_control_t {
                auto_rsp: ESP_GATT_RSP_BY_APP as u8,
            }
        ));
    }
}
