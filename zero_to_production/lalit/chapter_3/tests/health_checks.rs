//! tests/health_check.rs
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
  // Arrange
  let address = spawn_app();
  let client = reqwest::Client::new();
  // Act
  let response = client
  // Use the returned application address
  .get(&format!("{}/health_check", &address))
  .send()
  .await
  .expect("Failed to execute request.");
  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

  
fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0")
  .expect("Failed to bind random port");
  // We retrieve the port assigned to us by the OS
  let port = listener.local_addr().unwrap().port();
  let server = zero2prod::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);
  // We return the application address to the caller!
  format!("http://127.0.0.1:{}", port)
}

/*
// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

 async fn health_check_works() {
  // Arrange
  spawn_app();
  //spawn_app().await.expect("Failed to spawn our app.");
  // We need to bring in `reqwest`
  // to perform HTTP requests against our application.
  //
  // Use `cargo add reqwest --dev --vers 0.11` to add
  // it under `[dev-dependencies]` in Cargo.toml
  let client = reqwest::Client::new();
  // Act
  let response = client
  .get("http://127.0.0.1:8000/health_check")
  .send()
  .await
  .expect("Failed to execute request.");
  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}
 */
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
/*fn spawn_app() {
  let server = zero2prod::run("127.0.0.1:0").expect("Failed to bind address");
  let _ = tokio::spawn(server);
  //let server = zero2prod::run().expect("Failed to bind address");
  // Launch the server as a background task
  // tokio::spawn returns a handle to the spawned future,
  // but we have no use for it here, hence the non-binding let
  //
  // New dev dependency - let's add tokio to the party with
  // `cargo add tokio --dev --vers 1`
  //let _ = tokio::spawn(server);
}*/
