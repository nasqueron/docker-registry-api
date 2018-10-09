//! Docker Registry API application entry point

use crate::requests::*;
use limiting_factor::kernel::MinimalApplication;

pub fn run () {

    let routes = routes![
        status,
        favicon,
        get_registry_stats,
    ];

    MinimalApplication::start_application(routes);
}
