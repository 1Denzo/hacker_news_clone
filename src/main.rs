extern crate diesel;
pub mod schema;
pub mod models;

use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{User, NewUser};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use schema::users;

    let mut connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&mut connection)
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged in: {}", data.username))
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("This is the first link"),
            link: String::from("https://example.com"),
            author: String::from("Bob")
        },
        Post {
            title: String::from("The Second Link"),
            link: String::from("https://example.com"),
            author: String::from("Alice")
        },
    ];

    data.insert("title", "Hacker Clone");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
    })
    .bind("127.0.0.1:8060")?
    .run()
    .await
}