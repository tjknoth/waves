use std::io::Read;
use csv::ReaderBuilder;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        Parse(csv::Error);
    }
}

// Buoy readings are discrete or continuous
enum ReadingVal {
    Discrete(i32),
    Continuous(f32),
}

// Buoy readings are a 3-tuple of a label, units, and an actual value
pub struct Reading {
    name : String,
    units : String,
    val : Option<ReadingVal>,
}

pub enum BuoyKind {
    Swell,
    Tide,
}

// A swell buoy
pub struct Buoy {
    buoy_name: String,
    buoy_id: i32,
    buoy_kind : BuoyKind,
}

impl Buoy {
    pub fn new(buoy_name: String, buoy_id: i32, buoy_kind: BuoyKind) -> Self {
        Buoy {
            buoy_name,
            buoy_id,
            buoy_kind
        }
    }

    pub fn to_string(&self) -> String {
        format!("Buoy {} ({})", &self.buoy_id, &self.buoy_name)
    }

    pub fn to_url(&self) -> String {
        format!("https://www.ndbc.noaa.gov/data/realtime2/{}.txt", &self.buoy_id)
    }

    pub fn read(&self) -> Result<()> {
        match &self.buoy_kind {
            BuoyKind::Swell => self.read_swell(),
            BuoyKind::Tide => self.read_tide(),
        }
    }

    fn read_swell(&self) -> Result<()> {
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
        for _ in 0..10 {
            let record = rdr.records().next();
            println!("{:?}", record);
        }
        Ok(())
    }

    fn read_tide(&self) -> Result<()> {
        Ok(())
    }
}