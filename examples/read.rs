extern crate coralenv;

fn main() {
    println!("Temperature: {} °C", coralenv::temperature());
    println!("Humidity: {} %", coralenv::humidity());
    println!("Ambient Light: {} lux", coralenv::light());
    println!("Pressure: {} kPa", coralenv::pressure());
}
