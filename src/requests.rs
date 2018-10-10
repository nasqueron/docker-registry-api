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

#[get("/repository/getAll")]
pub fn get_all_repositories() -> ApiJsonResponse<Vec<Repository>> {
    Registry::with_default_location()
        .get_all_repositories()
        .into_json_response()
}

#[get("/repository/findByLayer/<hash>")]
pub fn find_repository_by_layer(hash: String) -> ApiJsonResponse<Vec<Repository>> {
    if !Repository::is_valid_hash(&hash) {
        return Err(build_bad_request_response())
    }

    Registry::with_default_location()
        .get_repositories_by_layer_hash(&hash)
        .into_json_response()
}

#[get("/repository/findByImage/<hash>")]
pub fn find_repository_by_image(hash: String) -> ApiJsonResponse<Vec<Repository>> {
    if !Repository::is_valid_hash(&hash) {
        return Err(build_bad_request_response())
    }

    Registry::with_default_location()
        .get_repositories_by_image_hash(&hash)
        .into_json_response()
}
