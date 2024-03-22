use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <html>
            <head>
                <title>Rust Web App</title>
            </head>
            <body style="font-family: Arial, sans-serif; text-align: center;">
                <h1>Add Two Integers</h1>
                <form action="/sum" method="post" style="margin: auto; width: 50%;">
                    <label for="num1" style="display: block; margin-bottom: 10px;">Number 1:</label>
                    <input type="number" id="num1" name="num1" style="width: 100%; padding: 5px; margin-bottom: 10px;"><br>
                    <label for="num2" style="display: block; margin-bottom: 10px;">Number 2:</label>
                    <input type="number" id="num2" name="num2" style="width: 100%; padding: 5px; margin-bottom: 10px;"><br>
                    <input type="submit" value="Calculate Sum" style="padding: 10px 20px; background-color: #007bff; color: #fff; border: none; cursor: pointer;">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn sum(params: web::Form<FormData>) -> impl Responder {
    let sum = params.num1 + params.num2;
    HttpResponse::Ok().body(format!("Sum: {}", sum))
}

#[derive(serde::Deserialize)]
struct FormData {
    num1: i32,
    num2: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/sum").route(web::post().to(sum)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
