use actix_web::web::Form;
use actix_web::{post, HttpResponse, Responder};
use hex::ToHex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlite::Connection;

#[derive(Deserialize, Clone)]
struct RegisterInfo {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterDatabaseResponse {
    success: bool,
    message: Option<String>,
}

#[post("/api/register")]
async fn register(form: Form<RegisterInfo>) -> impl Responder {
    let response = register_to_db(form.into_inner());
    println!("{}", response.success);
    if response.message.is_some() {
        println!("{}", response.message.unwrap());
    }
    HttpResponse::Ok().body("OK")
}

fn hash(password: String) -> String {
    let mut hasher: Sha256 = Digest::new();
    hasher.update(password);
    (&hasher.finalize()[..]).encode_hex::<String>()
}

fn register_to_db(to_register: RegisterInfo) -> RegisterDatabaseResponse {
    //check if username is already taken
    let connection: Connection = sqlite::open("data.db").unwrap();
    let statement = connection.prepare(format!(
        "SELECT COUNT(*) FROM User WHERE username = '{}'",
        to_register.username
    ));
    if statement.is_err() {
        println!("DB Error: {}", statement.err().unwrap().message.unwrap());
        return RegisterDatabaseResponse {
            success: false,
            message: Some("DBError".to_string()),
        };
    }
    let mut statement = statement.unwrap();
    if let Err(err) = statement.next() {
        return RegisterDatabaseResponse {
            success: false,
            message: Some(err.message.unwrap()),
        };
    }
    let count: i64 = statement.read::<i64>(0).unwrap();
    if count > 0 {
        return RegisterDatabaseResponse {
            success: false,
            message: Some("Username taken".to_string()),
        };
    }
    //hash password
    let hashed_password = hash(to_register.password);

    let output = connection.execute(format!(
        "INSERT INTO User (username, hashed_password) VALUES ('{}', '{}');
    ",
        to_register.username, hashed_password
    ));
    if output.is_err() {
        println!("DB Error: {}", output.err().unwrap().message.unwrap());
        return RegisterDatabaseResponse {
            success: false,
            message: Some("DBError".to_string()),
        };
    }

    RegisterDatabaseResponse {
        success: true,
        message: None,
    }
}
