use std::io::Read;
use itertools::izip;

// Buoy readings are discrete or continuous
#[derive(Debug)]
enum ReadingVal {
    Discrete(i32),
    Continuous(f32),
}

type BuoyResult<T> = Result<T, String>;

// Buoy readings are a 3-tuple of a label, units, and an actual value
#[derive(Debug)]
pub struct Reading {
    name : String,
    units : String,
    val : Option<ReadingVal>,
}

impl Reading {
    pub fn new(name: String, units: String, val: String) -> Self {
        Reading {
            name: name,
            units: units,
            val: Self::parse_val(val)
        }
    }

    fn parse_val(v: String) -> Option<ReadingVal> {
        match v.parse::<i32>() {
            Ok(n) => Some(ReadingVal::Discrete(n)),
            Err(_) =>
                match v.parse::<f32>() {
                    Ok(n) => Some(ReadingVal::Continuous(n)),
                    Err(_) => None
                }
        }
    }
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

    pub fn read(&self) -> BuoyResult<()> {
        match &self.buoy_kind {
            BuoyKind::Swell => self.read_swell(),
            BuoyKind::Tide => self.read_tide(),
        }
    }

    fn read_swell(&self) -> BuoyResult<()> {
        // Get readings
        let mut res = match reqwest::blocking::get(self.to_url()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string())
        }?;
        
        let mut body = String::new();

        res.read_to_string(&mut body);

        // For now: first two lines are measurements and units, third is a reading
        // TODO: read swell based on time
        let mut lines = body.lines();
        let measurements = lines
            .next()
            .unwrap_or_else(|| "Parse error: measurements");
        let units = lines
            .next()
            .unwrap_or_else(|| "Parse error: units");
        let firstline = lines
            .next()
            .unwrap_or_else(|| "Parse error: reading");
       
        // Zip into vec of Readings
        let readings: Vec<Reading> = izip!(measurements.split_whitespace(), units.split_whitespace(), firstline.split_whitespace())
            .map(|(m, u, r)| Reading::new(m.to_owned(), u.to_owned(), r.to_owned()))
            .collect();

        println!("{:?}", readings);

        Ok(())
    }

    fn read_tide(&self) -> BuoyResult<()> {
        Ok(()) // TODO
    }
}