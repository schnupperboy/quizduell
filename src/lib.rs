#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewQuestion, Question};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_question<'a>(
            conn: &PgConnection,
            question_text: &'a str,
            option_a: &'a str,
            option_b: &'a str,
            option_c: &'a str,
            option_d: &'a str
        ) -> Question {
    use schema::questions;

    let new_question = NewQuestion {
        question_text: question_text,
        option_a: option_a,
        option_b: option_b,
        option_c: option_c,
        option_d: option_d,
    };

    diesel::insert_into(questions::table)
        .values(&new_question)
        .get_result(conn)
        .expect("Error saving new question")
}