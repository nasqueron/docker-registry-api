use docker_registry_api::registry::Registry;

#[test]
fn test_registry_stats() {
    let registry = Registry::new("tests/registry".to_string());

    assert_eq!(3, registry.count_repositories()); // Lorem, ipsum, dolor
}

#[test]
fn test_registry_stats_when_registry_is_empty() {
    let registry = Registry::new("tests/void-registry".to_string());

    assert_eq!(0, registry.count_repositories());
}

#[test]
fn test_registry_stats_when_registry_is_a_file() {
    let registry = Registry::new("tests/broken-registry".to_string());

    assert_eq!(0, registry.count_repositories());
}
