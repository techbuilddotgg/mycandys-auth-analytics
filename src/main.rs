use actix_web::{web, App, HttpServer, Responder};

// Handler funkcija za korenno pot
async fn index() -> impl Responder {
    format!("Hello, world!")
}

// Glavna funkcija, ki zažene strežnik
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ustvarimo Http strežnik in nastavimo aplikacijo
    HttpServer::new(|| {
        App::new()
            // Dodamo pot za korenno pot
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?  // Povežemo na naslov in vrata
    .run()
    .await // Poženemo strežnik
}
