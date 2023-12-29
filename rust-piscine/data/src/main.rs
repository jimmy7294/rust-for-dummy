mod changes;
use changes::*;
mod test;
fn main() {
    // changes.rs
    println!("\x1b[31mchanges.rs\x1b[0m");
    let mut lights = vec![
        Light::new("living_room"),
        Light::new("bedroom"),
        Light::new("rest_room"),
    ];

    println!("brightness of living room = {}", lights[0].brightness);
    change_brightness(&mut lights, "living_room", 200);
    println!("new brightness of living room = {}", lights[0].brightness);
    println!("\x1b[31mEnd of changes.rs\x1b[0m\n");
}
