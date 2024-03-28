use std::io;

const WELCOME_STRING: &str = "Welcome!\nBelow are your options:
    \tType \"C\" for Farenheit --> Celcius
    \tType \"F\" for Celcius --> Farenheit";

/* Making a Temp Converter */
fn main() {
    // Getting input for calculation type
    let formatted_input: String = loop {
        println!("{}", WELCOME_STRING);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input: &str = input.trim();

        match input {
            "F" | "C" => break input.to_string(),
            _ => continue,
        }
    };

    // Grabbing Second Input for Calculation
    let calculation: f64 = loop {
        println!("Input the number you would like to convert.");

        let mut temp_string = String::new();
        io::stdin()
            .read_line(&mut temp_string)
            .expect("Failed to read line.");

        let temp: f64 = match temp_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        break if formatted_input == "F" {
            (temp - 32.0) * 5.0 / 9.0 // F to C
        } else {
            (temp * 9.0 / 5.0) + 32.0 // C to F
        };
    };

    println!("Converted Temperature: {}", calculation);
}
