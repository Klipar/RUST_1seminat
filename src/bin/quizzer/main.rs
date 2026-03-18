//! Quizzer binary - main entry point

use quizzer::{get_greeting};

fn main() {
    println!("Hello, world from quizzer binary!");

    let message = get_greeting();
    println!("{}", message);
}