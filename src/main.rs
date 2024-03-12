use rand::Rng;
use std::io::stdin;
fn main() {
    let random: u8 = rand::thread_rng().gen_range(0..=100);
    println!("Please type a number:");
    loop {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let input: u8 = input.trim().parse().unwrap();
        if input < random {
            println!("Too small");
        } else if input > random {
            println!("Too big");
        } else if input == random {
            break;
        }
    }
    println!("Correct!")
}
