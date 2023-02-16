use std::io::Read;
use csv::ReaderBuilder;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        Parse(csv::Error);
    }
}

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

    pub fn to_url(&self) -> String {
        format!("https://www.ndbc.noaa.gov/data/realtime2/{}.txt", &self.buoy_id)
    }

    pub fn read(&self) -> Result<()> {
        // Get readings
        let mut res = reqwest::blocking::get(self.to_url())?;
        let mut body = String::new();
        res.read_to_string(&mut body);

        // Read csv: for now just print first 10 entries
        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .flexible(true)
            .has_headers(false)              // Manually handle headers; they might change
            .from_reader(body.as_bytes());
        // Print first ten entries
        for i in 0..10 {
            let record = rdr.records().next();
            println!("{:?}", record);
        }
        Ok(())
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

    pub fn from_lines(str: &String) -> Self {
        Self::default() // TODO
    }
}

// A tide reading
#[derive(Default)]
pub struct Tide {
    tide: Option<f32>
}

impl Tide {
    pub fn new(tide: Option<f32>) -> Self {
        Tide {
            tide,
        }
    }

    pub fn from_lines(str: &String) -> Self {
        Self::default() // TODO
    }
}