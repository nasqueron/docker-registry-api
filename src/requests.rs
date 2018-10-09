//!
//! Requests handled by the microservice.
//!

use crate::registry::Registry;
use limiting_factor::api::replies::{ApiJsonResponse, ApiResponse};
use rocket::response::NamedFile;

#[get("/status")]
pub fn status() -> &'static str {
    "ALIVE"
}

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("assets/favicon.ico").ok()
}

//  -------------------------------------------------------------
//  /registry
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[get("/registry/stats")]
pub fn get_registry_stats() -> ApiJsonResponse<Registry> {
    Registry::with_default_location()
        .into_json_response()
}
