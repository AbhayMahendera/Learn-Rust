
/* 

use std::io;

fn main() {
    println!("Guess any number !");
    println!("Input your guess : ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {guess}");
}

*/

/*

std                      -> standard library
io                       -> input / ouput library from the standard library
let mut guess           -> mutable variable named guess ; if it were " let guess " , then it would be immutable
String::new             -> new instance of a String
:: (:: in  ::new)       -> new is an associated function of the String type
                            new function creates a new , empty string
let mut guess = String::new(); -> mutable variable that is currently bound to a new , empty instance of a String.

io::stdin()                 -> read_line method on the standard input handle to get input from the user.
    .read_line(&mut guess)      we are also passing &mut guess as the argument to read_line to tell it what 
                                string to store the input in. 

read_line takes whatever the user types into standard input and append that into a string without overwriting its contents , so therefore we pass that string as an argument. The string argument needs to be mutable so the method can change the string's content.
"&" indicates that it is the reference.

you need to write &mut guess to make it mutable , not &guess.


.expect("Failed to read line");         -> used to handle potential failure with result.


*/


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

// use rand::Rng        -> Rng trait defines methods that random number generator implements , and this trait must be in scope for us to use those methods.
// rand::thread_rng     -> this function gives us the particular random number generator we're going to use : one that is local to the current thread of execution and is seeded by the operating system.
// gen_range            -> this method takes a range as expressions as an argument that generates a random number in the range,
// start..=end          -> this is the kind of range expression we are using here; upper and lower bound are inclusive. So 1 to 100 is specified as 1..=100
// The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
// Line 66 : guess refers to original guess variable. Trim eliminates any leading or trailing whitespace and converts it to u32 (unsigned 32-bit number)
// parse() is a method used to parse a string into a specific type, in this case, a u32 (unsigned 32-bit integer).

// conversion was done so that the number we entered that was initially a string could be compared to the random secret number that we are generating.





/*
// same function but with loop and exiting after a correct guess :

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


*/