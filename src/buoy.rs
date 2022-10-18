pub struct Buoy {
    buoy_name: String,
    buoy_id: i32,
}

impl Buoy {
    pub fn new(buoy_name: String, buoy_id: i32) -> Self {
        Buoy {
            buoy_name,
            buoy_id,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Buoy {} ({})", &self.buoy_id, &self.buoy_name)
    }
}