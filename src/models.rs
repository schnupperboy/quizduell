use schema::questions;

#[derive(Queryable)]
pub struct Question {
    pub id: i32,
    pub question_text: String,
    pub option_a: String,
    pub option_b: String,
    pub option_c: String,
    pub option_d: String,
}

#[derive(Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub question_text: &'a str,
    pub option_a: &'a str,
    pub option_b: &'a str,
    pub option_c: &'a str,
    pub option_d: &'a str,
}
