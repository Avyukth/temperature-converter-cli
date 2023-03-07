use std::io;

fn main() {
    println!("Temperature Conversion Tool");

    // Get temperature input from user
    println!("Enter temperature:");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    // Parse input as a f32 float
    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature input");
            return;
        }
    };

    // Get input temperature unit from user
    println!("Enter temperature unit (C or F):");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read temperature unit");

    // Convert temperature to Celsius or Fahrenheit
    let converted_temperature: f32;
    match unit.trim().to_lowercase().as_str() {
        "c" => {
            converted_temperature = (temperature * 9.0 / 5.0) + 32.0;
            println!("{} C = {} F", temperature, converted_temperature);
        }
        "f" => {
            converted_temperature = (temperature - 32.0) * 5.0 / 9.0;
            println!("{} F = {} C", temperature, converted_temperature);
        }
        _ => {
            println!("Invalid temperature unit");
            return;
        }
    }
}
