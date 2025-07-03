use std::io::stdin;
use rand::{Rng};

pub trait Number {

    fn get_guessing_number(&self) -> i32;

    fn get_number(&self) -> i32;
}

struct Numbers {
    pub guess_answer: i32,
    pub number: i32

}

impl Number for Numbers {

    fn get_guessing_number(&self) -> i32 {
        self.guess_answer
    }

    fn get_number(&self) -> i32 {
        self.number
    }

}

fn main() {

    let num:i32 = rand::rng().random_range(1..10);

        let nums = Numbers {
            number: 0,
            guess_answer: 0
        };


    println!("Enter Number to Guess: \n");
    let mut number = String::new();

    stdin().read_line(&mut number).expect("No Input");
    number.trim().parse::<i32>().expect("TODO: panic message");
    if nums.get_guessing_number() == num {

        println!("Correct")

    }

    else {
        println!("Number is: {}", num);
    }

}

