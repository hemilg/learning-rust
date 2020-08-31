use std::io::{
    stdin,
    Stdin,
};

use rand::{
    thread_rng,
    Rng,
};

use std::cmp::Ordering::{
    Equal,
    Less,
    Greater,
};

use std::num::ParseIntError;

fn main() {
    println!("GUESS THE NUMBER");
    println!("Range: 0 <= n <= 1000");

    let mut rng = thread_rng();
    let secret_number = rng.gen_range(0, 1001);

    loop {
        let mut guess = String::new();  // reallocating is cheap on stack
        println!("Guess:");

        let input: Stdin = stdin();
        let result: std::io::Result<usize> = input.read_line(&mut guess);
        let error_msg: &str = "Failed to read line";
        let _bytes_read: usize = result.expect(error_msg);

        // equivalent to:
        //
        // stdin()
        // .read_line(&mut guess)
        // .expect("Failed to read line");

        let parse_output: std::result::Result<u32, ParseIntError> = guess.trim().parse::<u32>();

        // (exhaustive) pattern matching with match guards
        let guess: u32 = match parse_output {
            Ok(num) if num <= 1000 => num,
            Ok(_) | Err(_) => {
                println!("Guess must be an integer in the range [0, 1000].");
                continue;
            },
        };
        
        // (exhaustive) pattern matching using Orderings
        match guess.cmp(&secret_number) {
            Less => println!("Your guess {} is too low.", guess),
            Equal => {
                println!("Winner winner! Your guess {} is correct.", guess);
                break;
            },
            Greater => println!("Your guess {} is too high.", guess),
        }
    }
}
