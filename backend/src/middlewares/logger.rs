use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response, Data};
use std::time::Instant;

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let start_time = Instant::now();
        request.local_cache(|| start_time);
        println!("[REQUEST] {} {}", request.method(), request.uri());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let start_time: &Instant = request.local_cache(|| Instant::now());
        let duration = start_time.elapsed();
        println!(
            "[RESPONSE] {} {} - Status: {} - Duration: {:?}",
            request.method(),
            request.uri(),
            response.status(),
            duration
        );
    }
}