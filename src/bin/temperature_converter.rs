use std::io;

fn main() {
    loop {
        println!("Please input a temperature");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("{temperature} Celcius = {} Farenheit", convert_to_farenheit(temperature));
        println!("{temperature} Farenheit = {} Celcius", convert_to_celcius(temperature));
    }
}

fn convert_to_farenheit(celcius: f32) -> f32 {
    ((celcius * 9.0) / 5.0) + 32.0
}

fn convert_to_celcius(farenheit: f32) -> f32 {
    (5.0 / 9.0) * (farenheit - 32.0)
}
