use std::io::{self, Write};

fn far_to_deg(x: i32) -> f64 {
    (x as f64 -32.0) * (5.0 / 9.0)
}

fn main() {
    let mut guess = String::new();
    let fah :i32;

    loop {
        print!("Enter Fahrenheit to convert: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        fah = match guess.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("You must enter a number");
                continue;
            }
        };

        break;
    }

    println!("{}F = {}°C", fah, far_to_deg(fah));
}

