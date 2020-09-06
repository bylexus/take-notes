use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    const MIN: u32 = 1;
    const MAX: u32 = 100;
    println!(
        "I guess a number between {} and {}, and you will guess it!",
        MIN, MAX
    );

    let my_number: u32 = rand::thread_rng().gen_range(MIN, MAX + 1);
    println!("My number: {}", my_number);

    let mut counter = 0;
    loop {
        counter += 1;
        let mut user_input = String::new();
        println!("Your guess: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Oops, something's wrong!");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match my_number.cmp(&user_input) {
            Ordering::Equal => break,
            Ordering::Less => println!(" ====> That's too big"),
            Ordering::Greater => println!(" ====> That's too small"),
        };
    }

    println!("{:-<80}", "");
    println!("{:^80}", "Yeah, you won!");
    println!("{:^80}", format!("Tries: {}", counter));
    println!("{:-<80}", "");
}
