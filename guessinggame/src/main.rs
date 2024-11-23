// use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    //now we generate a random number
    let rnd_value = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess:");

        //use "let" to create a variable
        //use "mut" to make the variable mutable
        //String::new creates a new instance of a String
        //:: is used to call a function of a type
        //new() is a common function for many types
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //the & indicates that it is a reference.
            .expect("Failed to read value");

        //it seems this way is also ok, for now, still don't know why
        // println!("value:{}", &guess);

        //now we change the type of guess from String to int
        //the way below will cause the program to crash if the user input an
        //invalid value
        // let guess: i32 = guess.trim().parse().expect("Please input a number");
        //so we change it like the code below, please note there is a `match` keyword
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num, //Ok, the "k" is in lowercase!!!
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&rnd_value) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
