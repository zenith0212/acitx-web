use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct Appstatewithcounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<Appstatewithcounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter =  web::Data::new(Appstatewithcounter {
        counter: Mutex::new(0)
    });
    HttpServer::new(move || {
        App::new()
        .app_data(counter.clone())
        .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await    
}