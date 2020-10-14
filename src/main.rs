extern crate coral_env;

fn main() {
    println!("Temperature: {} °C", coral_env::temperature());
    println!("Humidity: {} %", coral_env::humidity());
    println!("Ambient Light: {} lux", coral_env::light());
    println!("Pressure: {} kPa", coral_env::pressure());
}
