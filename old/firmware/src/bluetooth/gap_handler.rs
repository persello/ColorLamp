use esp_idf_sys::*;
use log::*;

pub extern "C" fn gap_event_handler(
    event: esp_gap_ble_cb_event_t,
    param: *mut esp_ble_gap_cb_param_t,
) {
    let mut constants = crate::bluetooth::constants::Constants::default();

    #[allow(non_upper_case_globals)]
    match event {
        esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT => {
            info!("[GAP] BLE GAP advertisement data set complete.");
            info!("[GAP] Starting BLE GAP advertisement.");

            unsafe {
                esp_nofail!(esp_ble_gap_start_advertising(
                    &mut constants.advertisement_parameters
                ));
            }
        }

        esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_SET_COMPLETE_EVT => {
            info!("[GAP] BLE GAP scan response data set complete.");
            info!("[GAP] Starting BLE GAP response advertisement.");

            unsafe {
                esp_nofail!(esp_ble_gap_start_advertising(
                    &mut constants.advertisement_parameters
                ));
            }
        }

        esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT => {
            let param: esp_ble_gap_cb_param_t_ble_adv_data_cmpl_evt_param =
                unsafe { (*param).adv_data_cmpl };
            if param.status == esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
                info!("[GAP] BLE GAP advertisement started.");
            } else {
                warn!("[GAP] BLE GAP advertisement start failed.");
            }
        }

        esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_STOP_COMPLETE_EVT => {
            let param: esp_ble_gap_cb_param_t_ble_adv_data_cmpl_evt_param =
                unsafe { (*param).adv_data_cmpl };
            if param.status == esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
                info!("[GAP] BLE GAP advertisement stopped.");
            } else {
                warn!("[GAP] BLE GAP advertisement stop failed.");
            }
        }

        esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_CONN_PARAMS_EVT => {
            let param: esp_ble_gap_cb_param_t_ble_update_conn_params_evt_param =
                unsafe { (*param).update_conn_params };
            info!("[GAP] Connection parameters updated: {:?}", param);
        }

        _ => {
            warn!("[GAP] Unhandled GAP event: {:?}", event);
        }
    }
}
