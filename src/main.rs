mod buoy;

use crate::buoy::Buoy;
use crate::buoy::BuoyKind;

fn main() {
    let ptloma = Buoy::new(String::from("Pt. Loma South"), 46232, BuoyKind::Swell);
    println!("{}", ptloma.to_string());
    match ptloma.read() {
        Ok(()) => (),
        Err(error) => (),
    }
}
