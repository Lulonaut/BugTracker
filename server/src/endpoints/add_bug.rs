use crate::types::bug::Bug;
use crate::utils::check_auth_header;
use actix_web::{post, HttpRequest, HttpResponse, Responder};
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
    title: String,
    content: String,
}

#[post("/api/add_bug")]
async fn add_bug(form: actix_web::web::Form<FormData>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization");
    let author = check_auth_header(token);
    if author.is_none() {
        return HttpResponse::Unauthorized().json(ResponseError {
            success: false,
            cause: "Invalid Session".to_string(),
        });
    }
    if form.title.is_empty() || form.content.is_empty() {
        return HttpResponse::BadRequest().json(ResponseError {
            success: false,
            cause: "One or more fields empty".to_string(),
        });
    }
    let result = add_bug_to_db(Bug {
        id: 0,
        author: author.unwrap(),
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
