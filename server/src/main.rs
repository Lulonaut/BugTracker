use actix_web::{App, HttpServer};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::process::exit;
use std::{fs::File, path::Path};
mod endpoints;
mod types;
mod utils;
lazy_static! {
    static ref SECRET: String =
        std::env::var("HASH_SECRET").expect("Please add a HASH_SECRET to the .env");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env Error");
    setup_db();
    HttpServer::new(|| {
        App::new()
            .service(endpoints::list_all_bugs::list_all_bugs)
            .service(endpoints::add_bug::add_bug)
            .service(endpoints::auth::register)
            .service(endpoints::auth::login)
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

    //create user table
    let result = connection.execute(
        "CREATE TABLE User (id INTEGER PRIMARY KEY, username varchar[255], hashed_password varchar, current_token varchar, displayname varchar[255]);"
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
