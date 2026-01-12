// Test with minimal spec that has heterogeneous error types
mod minimal_client {
    progenitor_middleware::generate_api!(
        spec = "/tmp/minimal_heterogeneous_errors.json",
        interface = Builder,
        tags = Merged,
    );

    #[test]
    fn test_compiles() {
        // Just needs to compile
    }
}
