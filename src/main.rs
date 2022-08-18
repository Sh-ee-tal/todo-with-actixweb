

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]

pub struct Customer {
    pub username:String,
    pub password: String,
    pub todo: Vec<String>,
}

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";


/// Adds a new user to the "users" collection in the database.
#[get("/add_user")]
async fn add_user(client: web::Data<Client>, info: web::Json<Customer>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(info, None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("User added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("mongodb+srv://heer:goodtosee@cluster0.f22pwo8.mongodb.net/myApp?retryWrites=true&w=majority").unwrap();

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            //.service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}