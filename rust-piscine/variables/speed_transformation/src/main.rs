// Create a function to convert speed from km/h to m/s
mod speed_transformation;
mod test;
use speed_transformation::*;
fn main() {
    let km_h = 100.0;
    let m_s = kmh_to_ms(km_h);
    println!("{} km/h is equivalent to {} m/s", km_h, m_s);
}
