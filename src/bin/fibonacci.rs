use std::io;

fn main() {
    loop {
        println!("Which Fibonacci number to get?");

        let mut target_number = String::new();

        io::stdin()
            .read_line(&mut target_number)
            .expect("Failed to read line");

        let target_number: u32 = match target_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a positive number!");
                continue;
            }
        };

        println!("Entered number: {}", target_number);
        let mut previous_nth_fibonacci = 0;
        let mut nth_fibonacci = 0;
        for number in 0..target_number {
            println!("fibonacci so far: {}", nth_fibonacci);
            if number == 0 {
                nth_fibonacci = 1;
            } else {
                let temporal_fibonacci = nth_fibonacci;
                nth_fibonacci += previous_nth_fibonacci;
                previous_nth_fibonacci = temporal_fibonacci;
            }
        }
        println!("The nth fibonacci for your number is: {}", nth_fibonacci)
    }
}
