pub mod scraper;

#[derive(Debug, Eq, PartialEq)]
pub struct Contest {
    id: String,
    start_epoch_second: i64,
    duration_second: i64,
    title: String,
    rate_change: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    id: String,
    contest_id: String,
    title: String,
}

#[derive(Debug)]
pub struct Submission {
    id: u64,
    epoch_second: i64,
    problem_id: String,
    contest_id: String,
    user_id: String,
    language: String,
    point: f64,
    length: i32,
    result: String,
    execution_time: Option<i32>,
}
