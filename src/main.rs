extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse};

fn index(req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    server::new(|| vec![
        App::new()
            .prefix("/app1")
            .resource("/", |r| r.f(|r| HttpResponse::Ok())),
        App::new()
            .prefix("/app2")
            .resource("/", |r| r.f(|r| HttpResponse::Ok())),
        App::new().resource("/", |r| r.f(index))])
        .bind("127.0.0.1:8088")
        .expect("Can not bind to 127.0.0.1:8088")
        .run();
}
