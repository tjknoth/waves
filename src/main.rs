mod buoy;

use crate::buoy::Buoy;

fn main() {
    let ptloma = Buoy::new(String::from("Pt. Loma South"), 46232);
    println!("{}", ptloma.to_string());
}
