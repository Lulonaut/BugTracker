use crate::utils::{generate_token, hash};
use actix_web::web::Form;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlite::Connection;

#[derive(Deserialize, Clone)]
struct Info {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterDatabaseResponse {
    success: bool,
    message: Option<String>,
}

#[derive(Serialize)]
struct LoginDatabaseResponse {
    success: bool,
    message: String,
    auth_token: Option<String>,
}

#[post("/api/register")]
async fn register(form: Form<Info>) -> impl Responder {
    if form.username.contains(' ') {
        return HttpResponse::Ok().json(RegisterDatabaseResponse {
            success: false,
            message: Some("Invalid Username".to_string()),
        });
    }

    let response = register_to_db(form.into_inner());

    if response.success {
        HttpResponse::Ok().json(RegisterDatabaseResponse {
            success: true,
            message: Some("success".to_string()),
        })
    } else {
        HttpResponse::Ok().json(RegisterDatabaseResponse {
            success: false,
            message: Some(response.message.unwrap()),
        })
    }
}

#[post("/api/login")]
async fn login(form: Form<Info>) -> impl Responder {
    let response = check_login(form.into_inner());
    if response.success {
        HttpResponse::Ok().json(LoginDatabaseResponse {
            success: true,
            message: response.message,
            auth_token: Some(response.auth_token.unwrap()),
        })
    } else {
        HttpResponse::Ok().json(LoginDatabaseResponse {
            success: false,
            message: response.message,
            auth_token: None,
        })
    }
}

fn check_login(to_login: Info) -> LoginDatabaseResponse {
    let connection: Connection = sqlite::open("data.db").unwrap();
    let count: i64 = check_username_usage(to_login.clone().username, &connection);
    if count == 0 {
        return LoginDatabaseResponse {
            success: false,
            message: "Invalid Username".to_string(),
            auth_token: None,
        };
    }
    //check password
    let statement = connection.prepare(format!(
        "SELECT hashed_password FROM User WHERE username = '{}'",
        to_login.username,
    ));
    if statement.is_err() {
        println!("DBError: {}", statement.err().unwrap().message.unwrap());
        return LoginDatabaseResponse {
            success: false,
            message: "Database Error".to_string(),
            auth_token: None,
        };
    }
    let mut statement = statement.unwrap();
    if statement.next().is_err() {
        return LoginDatabaseResponse {
            success: false,
            message: "Database Error".to_string(),
            auth_token: None,
        };
    }

    let hashed_password_database = statement.read::<String>(0).unwrap();
    let hashed_password_given = hash(to_login.clone().password);
    if hashed_password_database != hashed_password_given {
        LoginDatabaseResponse {
            success: false,
            message: "Invalid Password".to_string(),
            auth_token: None,
        }
    } else {
        LoginDatabaseResponse {
            success: true,
            message: "".to_string(),
            auth_token: Some(generate_token(to_login.username)),
        }
    }
}

fn register_to_db(to_register: Info) -> RegisterDatabaseResponse {
    //check if username is already taken
    let connection: Connection = sqlite::open("data.db").unwrap();
    let count: i64 = check_username_usage(to_register.clone().username, &connection);
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

fn check_username_usage(username: String, connection: &Connection) -> i64 {
    let statement = connection.prepare(format!(
        "SELECT COUNT(*) FROM User WHERE username = '{}'",
        username
    ));
    if statement.is_err() {
        println!("DBError: {}", statement.err().unwrap().message.unwrap());
        return -1;
    }
    let mut statement = statement.unwrap();
    if statement.next().is_err() {
        return -1;
    }
    statement.read::<i64>(0).unwrap()
}
