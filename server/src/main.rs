use std::process::exit;
use std::{fs::File, path::Path};

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod endpoints;
mod types;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_db();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(endpoints::list_all_bugs::list_all_bugs)
            .service(endpoints::add_bug::add_bug)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn setup_db() {
    let path = Path::new("data.db");

    if !path.exists() {
        File::create(path).expect("Unable to create the db file");
    }

    let connection = sqlite::open("data.db").unwrap();

    //create bug table
    let result = connection.execute(
        "CREATE TABLE Bug (id INTEGER PRIMARY KEY, author varchar[255], title varchar[255], content varchar[20000]);
        ",
    );
    if result.is_err() {
        let err = result.err().unwrap().message.unwrap();
        if !&err.contains("already exists") {
            println!(
                "Some error occured while setting up the database: {:?}",
                &err
            );
            exit(1);
        }
    }
}
