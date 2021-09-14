//! Docker Registry API application entry point

use crate::requests::*;
use limiting_factor::kernel::MinimalApplication;
use rocket_codegen::routes;

pub fn run () {

    let routes = routes![
        // Monitoring
        status,

        // /docker/registry
        get_registry_stats,

        // /docker/registry/repository
        get_repository_info,
        get_all_repositories,
        find_repository_by_layer,
        find_repository_by_image,
    ];

    MinimalApplication::start_application(routes);
}
