//!
//! Requests handled by the microservice.
//!

use rocket::response::NamedFile;

#[get("/status")]
pub fn status() -> &'static str {
    "ALIVE"
}

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("assets/favicon.ico").ok()
}
