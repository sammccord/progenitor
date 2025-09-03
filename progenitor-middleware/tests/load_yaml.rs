mod load_yaml {
    progenitor_middleware::generate_api!("../sample_openapi/param-overrides.yaml");

    fn _ignore() {
        let _ = Client::new("").key_get(None, None);
    }
}
