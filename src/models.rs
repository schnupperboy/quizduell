#[derive(Queryable)]
pub struct Question {
    pub id: i32,
    pub question_text: String,
    pub option_a: String,
    pub option_b: String,
    pub option_c: String,
    pub option_d: String,
}
