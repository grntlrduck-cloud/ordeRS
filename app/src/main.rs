mod adapters;

use adapters::rest::book_handler;
use chrono::Local;
use openapi::models;
use svix_ksuid::{Ksuid, KsuidLike};

fn main() {
    let book_h = book_handler::BookHandler {
        name: String::from("BookHandler"),
    };
    println!("Hello {}!", book_h.name);

    let author_model = models::Author::new(
        Ksuid::new(None, None).to_string(),
        String::from("Johann"),
        String::from("Goethe"),
        Local::now().date_naive(),
    );
    println!(
        "{} {} was born on {}, right?",
        author_model.first_name,
        author_model.last_name,
        author_model.date_of_birth.to_string()
    );
}
