use actix_web::{web, App, HttpServer, Responder};

struct Appstore {
    app_name: String,
}

async fn index(data: web::Data<Appstore>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name} !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new( Appstore {
            app_name: String::from("Actix web")
        }))
        .service(
            web::scope("/app")
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await    
}