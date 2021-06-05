use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlite::State;

use crate::types::bug::{self, Bug};

#[derive(Serialize, Deserialize)]
struct Error {
    success: bool,
    cause: String,
}

#[get("/api/buglist")]
async fn list_all_bugs() -> impl Responder {
    let bug_list = get_bugs_from_db();
    let as_string = serde_json::to_string(&bug_list).unwrap();

    HttpResponse::Ok()
        .header("content-type", "application/json")
        .body(as_string)
}

fn get_bugs_from_db() -> Vec<bug::Bug> {
    let mut bugs: Vec<bug::Bug> = Vec::new();
    let connection = sqlite::open("data.db").unwrap();
    let mut result = connection.prepare("SELECT * FROM bugs").unwrap();

    while let State::Row = result.next().unwrap() {
        bugs.push(Bug {
            id: bugs.len() as isize + 1,
            author: result.read::<String>(1).unwrap(),
            title: result.read::<String>(2).unwrap(),
            content: result.read::<String>(3).unwrap(),
        })
    }
    bugs
}
