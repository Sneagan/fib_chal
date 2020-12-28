use actix_session::CookieSession;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use std::sync::Mutex;
mod fibonacci;
use fibonacci::Fibonacci;

lazy_static! {
    static ref FIB: Mutex<Fibonacci> = Mutex::new(Fibonacci::new());
}

/// Get the next value for the shared sequence.
#[get("/next")]
async fn fib_next() -> impl Responder {
    let mut fib = FIB.lock().unwrap();
    let result = match fib.next() {
        Some(value) => value.to_string(),
        None => String::from("0")
    };
    HttpResponse::Ok().body(result.to_string())
}

/// Get the previous value for the shared sequence.
#[get("/previous")]
async fn fib_previous() -> impl Responder {
    let mut fib = FIB.lock().unwrap();
    let result = match fib.previous() {
        Some(value) => value.to_string(),
        None => String::from("0")
    };
    HttpResponse::Ok().body(result.to_string())
}

/// Get the current value of the shared sequence. It's not cleaar from the requirements what
/// happens in the event that the first call is to /current. Therefore, calling /current before
/// calling /next will yield 0 even though the sequence has not been formall progressed in the 0
/// position. This can be changed trivially if needed.
#[get("/current")]
async fn fib_current() -> impl Responder {
    let fib = FIB.lock().unwrap();
    HttpResponse::Ok().body(fib.current().unwrap().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                // This session is not currently being used because the implementation and testing
                // are outside the written scope of this project. However, the API could be
                // extended to have session-based `current` numbers to serve as the input for
                // pregression instead of the universal mechanism being used now.
                CookieSession::signed(&[0; 32])
                    .domain("www.somefibonaccisite.pro")
                    .name("fib_chal")
                    .path("/")
                    .secure(true)
            )
            .service(fib_next)
            .service(fib_previous)
            .service(fib_current)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
