use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Hello {
  name: String
}

#[derive(Serialize)]
struct Greeting {
    message: String
}

async fn hello_get() -> impl Responder {
  "Hello World 🐙🐙🐙"
}

async fn hello_post(info: web::Json<Hello>) -> impl Responder {
  let message = format!("Hello {}", &info.name);
  HttpResponse::Ok().json(Greeting{message})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/api/hello", web::get().to(hello_get))
      .route("/api/hello", web::post().to(hello_post))
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
