use crate::types::bug::Bug;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, Statement};

#[derive(Serialize)]
struct ResponseError {
    success: bool,
    cause: String,
}

#[derive(Serialize)]
struct ResponseSuccess {
    success: bool,
    id: i64,
}

#[derive(Deserialize, Clone)]
struct FormData {
    author: String,
    title: String,
    content: String,
}

#[post("/api/add_bug")]
async fn add_bug(form: actix_web::web::Form<FormData>) -> impl Responder {
    if form.author.is_empty() || form.title.is_empty() || form.content.is_empty() {
        return HttpResponse::BadRequest().json(ResponseError {
            success: false,
            cause: "One or more fields empty".to_string(),
        });
    }
    let result = add_bug_to_db(Bug {
        id: 0,
        author: form.clone().author,
        title: form.clone().title,
        content: form.clone().content,
    });
    match result {
        Ok(id) => HttpResponse::Ok().json(ResponseSuccess { success: true, id }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseError {
            success: false,
            cause: err.message.unwrap(),
        }),
    }
}
fn add_bug_to_db(bug: Bug) -> Result<i64, sqlite::Error> {
    let connection: Connection = sqlite::open("data.db").unwrap();
    let result = connection.execute(format!(
        "
        INSERT INTO Bug (author, title, content) VALUES (\"{}\", \"{}\", \"{}\")
    ",
        bug.author, bug.title, bug.content
    ));

    let mut statement: Statement = connection
        .prepare(format!(
            "SELECT id FROM Bug WHERE author = '{}' AND title = '{}' AND content = '{}' ORDER BY id DESC LIMIT 1",
            bug.author, bug.title, bug.content
        ))
        .unwrap();
    if let Err(err) = statement.next() {
        return Err(err);
    }
    let id: i64 = statement.read::<i64>(0).unwrap();

    match result {
        Ok(_) => Ok(id),
        Err(err) => Err(err),
    }
}
