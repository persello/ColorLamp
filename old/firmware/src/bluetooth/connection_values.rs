use esp_idf_sys::*;

#[derive(Clone, Debug)]
pub struct ConnectionValues {
    gatts_if: u8,
    conn_id: u16,
    brightness_handle: u16,
    temperature_handle: u16,
}

impl ConnectionValues {
    pub fn new() -> Self {
        Self {
            gatts_if: ESP_GATT_IF_NONE as u8,
            conn_id: u16::MAX,
            brightness_handle: u16::MAX,
            temperature_handle: u16::MAX,
        }
    }

    pub fn set_gatts_if(&mut self, gatts_if: u8) {
        self.gatts_if = gatts_if;
    }

    pub fn set_conn_id(&mut self, conn_id: u16) {
        self.conn_id = conn_id;
    }

    pub fn set_brightness_handle(&mut self, brightness_handle: u16) {
        self.brightness_handle = brightness_handle;
    }

    pub fn set_temperature_handle(&mut self, temperature_handle: u16) {
        self.temperature_handle = temperature_handle;
    }

    pub fn get_gatts_if(&self) -> u8 {
        self.gatts_if
    }

    pub fn get_conn_id(&self) -> u16 {
        self.conn_id
    }

    pub fn get_brightness_handle(&self) -> u16 {
        self.brightness_handle
    }

    pub fn get_temperature_handle(&self) -> u16 {
        self.temperature_handle
    }

    pub fn is_connected(&self) -> bool {
        self.gatts_if != ESP_GATT_IF_NONE as u8 && self.conn_id != u16::MAX
    }
}

unsafe impl Sync for ConnectionValues {}
