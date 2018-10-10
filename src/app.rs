//! Docker Registry API application entry point

use crate::requests::*;
use limiting_factor::kernel::MinimalApplication;

pub fn run () {

    let routes = routes![
        status,
        favicon,
        get_registry_stats,
        get_repository_info,
        get_all_repositories,
        find_repository_by_layer,
        find_repository_by_image,
    ];

    MinimalApplication::start_application(routes);
}
