use rocket::local::blocking::Client;
use std::fs;

pub fn setup_test_client() -> Client {
    // Use a unique test database for each test run
    let test_db = format!("test_auth_{}.db", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros());
    
    // Clean up any existing test database
    let _ = fs::remove_file(&test_db);
    
    // Build the rocket instance
    let rocket = auth_app::rocket();
    
    Client::tracked(rocket).expect("valid rocket instance")
}
