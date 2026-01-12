// Copyright 2024 Oxide Computer Company

// Test that we can generate code for the Coinbase API which has operations
// with multiple different error response types (heterogeneous errors).
// For example, settleX402Payment has:
// - 400 → x402SettlePaymentRejection schema
// - 402, 500, 502, 503 → Error schema
mod coinbase_client {
    progenitor_middleware::generate_api!(
        spec = "../sample_openapi/api.coinbase.com.json",
        interface = Builder,
        tags = Merged,
    );

    // Verify that the code compiles, which validates that error enums
    // are generated correctly for operations with heterogeneous error types.
    #[test]
    fn test_client_compiles() {
        // This test just needs to compile - the existence of the error enums
        // and correct type signatures is validated at compile time.
    }
}
