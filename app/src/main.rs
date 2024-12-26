mod adapters;
mod domain;

use adapters::rest::server;
use chrono::Local;
use openapi::models;
use svix_ksuid::{Ksuid, KsuidLike};

#[tokio::main()]
async fn main() {
    let author_model = models::Author::new(
        Ksuid::new(None, None).to_string(),
        String::from("Johann"),
        String::from("Goethe"),
        Local::now().date_naive(),
    );
    println!(
        "{} {} was born on {}, right?",
        author_model.first_name, author_model.last_name, author_model.date_of_birth
    );

    println!("Starting web server");
    server::start_server("0.0.0.0:8443").await;
}
