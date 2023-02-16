use actix_files as actix_fs;
use actix_web::{ get, App, Error, HttpRequest, HttpResponse, HttpServer , Responder };
use frontend::App as YewApp;
use std::fs;
use yew::ServerRenderer;

// To run the project use
// cd ssr-with-rust
// cargo run

#[get("/")]
async fn render_yew_app(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html:String = fs::read_to_string("./frontend/dist/index.html").unwrap();

    let renderer:ServerRenderer::<YewApp> = ServerRenderer::<YewApp>::new();

    let content:String = renderer.render().await;

    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(index_html.replace("<body>", &format!("<body>{}", content)))
    )
}

#[get("/testi")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(actix_fs::Files::new("/dist", "./frontend/dist"))
        
        .service(render_yew_app)
        .service(hello)
    })
        .bind(("localhost", 3000))?
        .run()
        .await
}