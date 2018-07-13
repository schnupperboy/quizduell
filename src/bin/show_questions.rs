extern crate quizduell;
extern crate diesel;

use self::quizduell::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use quizduell::schema::questions::dsl::*;

    let connection = establish_connection();
    let results = questions
        .limit(5)
        .load::<Question>(&connection)
        .expect("Error loading questions");

    println!("Displaying {} questions", results.len());
    for question in results {
        println!("{}", question.question_text);
        println!("----------\n");
        println!("{}", question.option_a);
        println!("{}", question.option_b);
        println!("{}", question.option_c);
        println!("{}", question.option_d);
    }
}