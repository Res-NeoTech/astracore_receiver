//! # AstraCore Receiver Script
//!
//! This application is a lightweight HTTP server built with [Actix Web](https://actix.rs/).
//! It provides:
//! - A healthcheck endpoint (`/`).
//! - An execution endpoint (`/execute`) secured with an authentication token.
//!
//! On first run, if a `.env` file is missing, the server will generate a unique
//! authentication token (`ASTRA_TOKEN`) and store it in a `.env` file.
//!
//! ## Authentication
//! Requests to `/execute` must include the `x-astra-token` header with the correct value.
//!
//! ## Default Port
//! The server listens on port `6769` by default.

mod env;
mod command;

use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use env::{get_env, insert_custom_env};
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::process::exit;
use command::execute as command_execute;
use serde::Deserialize;

/// Default TCP port used by the AstraCore receiver.
const DEFAULT_APP_PORT: u16 = 6769;

#[derive(Deserialize)]
struct ExecCommand {
    syntax: String,
}

/// Healthcheck endpoint.
///
/// Returns a static string confirming that the receiver is up and running.
///
/// # Example
/// ```bash
/// curl http://127.0.0.1:6769/
/// ```
#[get("/")]
async fn hello() -> impl Responder {
    return "AstraCore receiver script is healthy and running.";
}

/// Execution endpoint.
///
/// Requires the client to provide the correct `x-astra-token` header.
/// If the token matches the one stored in the `.env` file, the endpoint executes
/// and responds with a confirmation message. Otherwise, it returns an `Unauthorized` error.
///
/// # Authentication
/// ```bash
/// curl -X POST http://127.0.0.1:6769/execute \
///      -H "x-astra-token: <your_token>"
/// ```
///
/// # Errors
/// - Returns `401 Unauthorized` if:
///   - The `x-astra-token` header is missing.
///   - The token cannot be parsed.
///   - The token does not match the expected value.
#[post("/execute")]
async fn execute(req: HttpRequest, command: web::Json<ExecCommand>) -> Result<impl Responder, actix_web::Error> {
    let token: String = get_env(&"ASTRA_TOKEN".to_string(), None);
    if let Some(astra_token) = req.headers().get("x-astra-token") {
        if let Ok(token_str) = astra_token.to_str() {
            if token == token_str {
                if command.syntax.trim().is_empty() {
                    return Err(ErrorBadRequest("Request body is incorrect."));
                } else {
                    let output: String = command_execute(&command.syntax);
                    return Ok(format!("{}", output));
                }
            } else {
                return Err(ErrorUnauthorized("Unauthorized."));
            }
        } else {
            return Err(ErrorUnauthorized("Unauthorized."));
        }
    } else {
        return Err(ErrorUnauthorized("Unauthorized."));
    }
}

/// Main entry point.
///
/// - If the `.env` file is missing:
///   - Generates a random 32-character alphanumeric token.
///   - Saves it as `ASTRA_TOKEN` in a new `.env` file.
///   - Informs the user and exits so they can restart the service.
/// - Otherwise:
///   - Starts an Actix Web server with the `hello` and `execute` services.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !std::path::Path::new(".env").exists() {
        let token: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(32)
            .map(char::from)
            .collect(); // Unique token generation. Later, it will be used to verify x-astra-token header.

        println!(".env file not found. Initiating install process...");
        insert_custom_env("ASTRA_TOKEN", &token)?;
        println!("!IMPORTANT! Insert the folowing token in your AstraCore Panel:\n");
        println!("{}\n", &token);
        println!("This token will also be available in .env");
        println!(
            "The installation process finished. Restart the daemon to apply new settings. Thank you for using AstraCore!"
        );
        exit(0);
    }

    HttpServer::new(|| App::new().service(hello).service(execute))
        .bind(("127.0.0.1", DEFAULT_APP_PORT))?
        .run()
        .await
}