use crate::gatt_server::Profile;
use esp_idf_sys::*;
use log::{info, warn};

impl Profile {
    pub(crate) fn on_reg(&mut self, param: esp_ble_gatts_cb_param_t_gatts_reg_evt_param) {
        // Check status
        if param.status == esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
            info!(
                "{} registered on interface {}.",
                &self,
                self.interface.unwrap()
            );
            self.register_services();
        } else {
            warn!("GATT profile registration failed.");
        }
    }
}
