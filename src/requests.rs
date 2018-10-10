//!
//! HTTP requests handled by the microservice.
//!

use crate::registry::{Registry, Repository};
use limiting_factor::api::replies::*;
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

//  -------------------------------------------------------------
//  /repository
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[get("/repository/<repository_name>")]
pub fn get_repository_info(repository_name: String) -> ApiJsonResponse<Repository> {
    let repository = Registry::with_default_location()
        .get_repository(&repository_name);

    match repository {
        None => Err(build_bad_request_response()),
        Some(repo) => {
            if repo.exists() {
                Ok(repo.into_json_response()?)
            } else {
                Err(build_not_found_response())
            }
        }
    }
}
