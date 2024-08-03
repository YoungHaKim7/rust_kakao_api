use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use kakao_rs::prelude::*;
use serde_json::Value;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/kakao")]
async fn kakao(kakao: web::Json<Value>) -> impl Responder {
    let mut result = Template::new();

    result.add_output(SimpleText::new("안녕하세요~~").build());

    let body = serde_json::to_string(&result).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

// [ source: http://rust-study.ajousw.kr/ ]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(kakao).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
