use crate::utils::check_auth;
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State, Statement};

use crate::types::bug::{self, Bug};

#[derive(Serialize)]
struct Error {
    success: bool,
    cause: String,
}

#[derive(Deserialize, Clone)]
struct FormData {
    auth_token: String,
}

#[derive(Serialize)]
struct ResponseError {
    success: bool,
    cause: String,
}

#[get("/api/buglist")]
async fn list_all_bugs(form: actix_web::web::Form<FormData>) -> impl Responder {
    let username = check_auth(form.clone().auth_token);
    if username.is_none() {
        return HttpResponse::Unauthorized().json(ResponseError {
            success: false,
            cause: "Invalid Session".to_string(),
        });
    }

    let bug_list: Vec<Bug> = get_bugs_from_db();
    let as_string: Result<String, serde_json::Error> = serde_json::to_string(&bug_list);
    match as_string {
        Ok(_) => {
            let response: String =
                format!("{{\"success\": true, \"data\": {}}}", as_string.unwrap());
            HttpResponse::Ok()
                .header("content-type", "application/json")
                .body(response)
        }
        Err(err) => HttpResponse::InternalServerError().json(Error {
            success: false,
            cause: err.to_string(),
        }),
    }
}

fn get_bugs_from_db() -> Vec<bug::Bug> {
    let mut bugs: Vec<bug::Bug> = Vec::new();
    let connection: Connection = sqlite::open("data.db").unwrap();
    let mut result: Statement = connection.prepare("SELECT * FROM Bug").unwrap();

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
