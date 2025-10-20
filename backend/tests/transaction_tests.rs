use actix_web::{App, test};
use fraud_detection::models::CreateTransactionRequest;

///  /health endpoint returns success
#[actix_web::test]
async fn test_health_check() {
    // Create a test app with just the health route
    let app = test::init_service(App::new().configure(fraud_detection::configure_app)).await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;

    // Assert it's successful
    assert!(resp.status().is_success());
    println!("Health check test passed!");
}

/// Test 2: Transaction Request Deserialization
/// CreateTransactionRequest model works
#[test]
async fn test_transaction_deserializes_from_json() {
    let json_data = r#"{
        "user_id": "test_user_123",
        "amount": 99.99,
        "merchant": "Test Coffee Shop",
        "country": "USA",
        "card_last_4": "5678"
    }"#;

    // Try to deserialize JSON to CreateTransactionRequest
    let result: Result<CreateTransactionRequest, _> = serde_json::from_str(json_data);

    // Assert it worked
    assert!(result.is_ok());

    let tx_req = result.unwrap();
    assert_eq!(tx_req.user_id, "test_user_123");
    assert_eq!(tx_req.amount, 99.99);
    assert_eq!(tx_req.merchant, "Test Coffee Shop");
    assert_eq!(tx_req.country, "USA");
    assert_eq!(tx_req.card_last_4, "5678");

    println!("Transaction deserialization test passed!");
}

/// Test 3: Transaction Serialization
/// Convert CreateTransactionRequest back to JSON
#[test]
async fn test_transaction_serializes_to_json() {
    let tx_req = CreateTransactionRequest {
        user_id: "user456".to_string(),
        amount: 150.50,
        merchant: "Gas Station".to_string(),
        country: "Canada".to_string(),
        card_last_4: "9999".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&tx_req);

    assert!(json.is_ok());
    let json_str = json.unwrap();

    // Check it contains expected values
    assert!(json_str.contains("user456"));
    assert!(json_str.contains("150.5"));
    assert!(json_str.contains("Gas Station"));

    println!("Transaction serialization test passed!");
}
