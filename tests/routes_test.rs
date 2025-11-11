use rocket::http::{ContentType, Status};

mod common;

#[test]
fn test_index_route() {
    let client = common::setup_test_client();
    let response = client.get("/").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("Hello, world!"));
}

#[test]
fn test_register_page() {
    let client = common::setup_test_client();
    let response = client.get("/register").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("Register"));
}

#[test]
fn test_register_new_user() {
    let client = common::setup_test_client();
    
    let username = format!("testuser_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    let response = client
        .post("/register")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
}

#[test]
fn test_register_duplicate_user() {
    let client = common::setup_test_client();
    
    let username = format!("duplicate_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    // First registration should succeed
    client
        .post("/register")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    // Second registration with same username should fail
    let response = client
        .post("/register")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/register"));
}

#[test]
fn test_login_with_valid_credentials() {
    let client = common::setup_test_client();
    
    let username = format!("loginuser_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    // Register user first
    client
        .post("/register")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    // Try to login
    let response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/dashboard"));
    
    // Check that cookie was set
    let cookies: Vec<_> = response.cookies().iter().collect();
    assert!(!cookies.is_empty());
}

#[test]
fn test_login_with_invalid_credentials() {
    let client = common::setup_test_client();
    
    let response = client
        .post("/login")
        .header(ContentType::Form)
        .body("username=nonexistent&password=wrongpass")
        .dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
}

#[test]
fn test_dashboard_without_auth() {
    let client = common::setup_test_client();
    let response = client.get("/dashboard").dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
}

#[test]
fn test_dashboard_with_auth() {
    let client = common::setup_test_client();
    
    let username = format!("dashuser_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    // Register and login
    client
        .post("/register")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    let _login_response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    // Cookies are automatically stored in the client for blocking Client
    // Try to access dashboard (should succeed with session)
    let response = client.get("/dashboard").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("Dashboard"));
}

#[test]
fn test_logout() {
    let client = common::setup_test_client();
    
    let username = format!("logoutuser_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    // Register and login
    client
        .post("/register")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    let _login_response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!("username={}&password=testpass123", username))
        .dispatch();
    
    // Logout - client automatically maintains cookies
    let response = client.get("/logout").dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
}
