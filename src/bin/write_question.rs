extern crate diesel;
extern crate quizduell;

use quizduell::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your question to be?");
    let mut question_text = String::new();
    stdin().read_line(&mut question_text).unwrap();
    let question_text = &question_text[..(question_text.len() - 1)]; // Drop the newline character

    let mut option_a = String::new();
    let mut option_b = String::new();
    let mut option_c = String::new();
    let mut option_d = String::new();

    println!("\nOk! Let's add option a for that question write {} (Press {} when finished)\n", question_text, EOF);
    stdin().read_to_string(&mut option_a).unwrap();
    println!("\nOk! Let's add option b for that question write {} (Press {} when finished)\n", question_text, EOF);
    stdin().read_to_string(&mut option_b).unwrap();
    println!("\nOk! Let's add option c for that question write {} (Press {} when finished)\n", question_text, EOF);
    stdin().read_to_string(&mut option_c).unwrap();
    println!("\nOk! Let's add option d for that question write {} (Press {} when finished)\n", question_text, EOF);
    stdin().read_to_string(&mut option_d).unwrap();

    let question = create_question(&connection, &question_text, &option_a, &option_b, &option_c, &option_d);
    println!("\nSaved draft {} with id {}", question_text, question.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";