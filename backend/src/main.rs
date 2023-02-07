use actix_files as actix_fs;
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer};
use frontend::App as YewApp;
use std::fs;
use yew::ServerRenderer;

// To run the project use
// cd ssr-with-rust
// cargo run


#[get("/")]
async fn render_yew_app(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html = fs::read_to_string("./frontend/dist/index.html").unwrap();
    
    let renderer = ServerRenderer::<YewApp>::new();

    let content = renderer.render().await;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html.replace("<body>", &format!("<body>{}", content))))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(
                actix_fs::Files::new("/dist", "./dist")
            )

            .service(render_yew_app)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

