use std::io::stdin;
use rand::{Rng};


fn main() {

    let num:i32 = rand::rng().random_range(1..10);


    println!("Enter Number to Guess: \n");
    let mut number = String::new();


    stdin().read_line(&mut number).expect("No Input");

    let nums: i32 =     number.trim().parse::<i32>().expect("TODO: panic message");

    if nums == num {

        println!("Correct")

    }

    else {
        println!("Number is: {}", num);
    }

}

