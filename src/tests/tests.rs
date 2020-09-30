use crate::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_health() {
    dotenv::dotenv().ok();
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/api").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("OK".into()));
}