// Regression test for OpenAPI 3.1-style `type: null` branches and mixed
// successful responses with and without a body.
mod openapi_null_and_mixed_client {
    progenitor_middleware::generate_api!(
        spec = "../sample_openapi/openapi-null-and-mixed.json",
        interface = Builder,
        tags = Merged,
    );

    #[test]
    fn test_client_compiles() {
        // Successful responses must be represented by a generated response enum,
        // and the schema's null branch must remain valid Rust output.
    }
}
