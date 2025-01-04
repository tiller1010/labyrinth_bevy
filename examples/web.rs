use actix_files::{NamedFile, Files};
use actix_web::{web, App, HttpServer};

async fn game() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(game))
            .service(Files::new("/wasm_out", "./wasm_out").show_files_listing())
            .service(Files::new("/assets", "./assets").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

