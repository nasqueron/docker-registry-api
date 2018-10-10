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
    ];

    MinimalApplication::start_application(routes);
}
