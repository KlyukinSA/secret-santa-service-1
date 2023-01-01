// Sample REST API for the Rust programming language

// Import the required libraries
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// Define the data structure for the REST API
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

// Define the function to handle the REST API
async fn index(info: web::Json<Person>) -> impl Responder {
    HttpResponse::Ok().json(info.0)
}

// Define the main function
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("127.0.0.0:8080")? // Bind the server to the IP address and port
        .run() // Run the server
        .await // Wait for the server to start
}

// End of file
