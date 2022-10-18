// A buoy (data source)
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

// A swell reading
#[derive(Default)]
pub struct Swell {
    wind_dir: Option<i32>,
    wind_speed: Option<f32>,
    wind_gusts: Option<f32>,
    wave_height: Option<f32>,
    wave_dpd: Option<f32>,
    wave_apd: Option<f32>,
    wave_dir: Option<i32>,
}

impl Swell {
    pub fn new(wind_dir: Option<i32>, wind_speed: Option<f32>, wind_gusts: Option<f32>, wave_height: Option<f32>, wave_dpd: Option<f32>, wave_apd: Option<f32>, wave_dir: Option<i32>) -> Self {
        Swell {
            wind_dir,
            wind_speed,
            wind_gusts,
            wave_height,
            wave_dpd,
            wave_apd,
            wave_dir,
        }
    }
}

// A tide reading
#[derive(Default)]
pub struct Tide {
    water_depth: Option<f32>
}

impl Tide {
    pub fn new(water_depth: Option<f32>) -> Self {
        Tide {
            water_depth,
        }
    }
}