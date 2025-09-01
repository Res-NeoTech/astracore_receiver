mod env;

use actix_web::{App, HttpRequest, HttpServer, Responder, get, post};
use env::{get_env, insert_custom_env};
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::process::exit;

const DEFAULT_APP_PORT: u16 = 6769;

#[get("/")]
async fn hello() -> impl Responder {
    return "AstraCore receiver script is healthy and running.";
}

#[post("/execute")]
async fn execute(req: HttpRequest) -> impl Responder {
    let token: String = get_env(&"ASTRA_TOKEN".to_string(), None);
    if let Some(astra_token) = req.headers().get("x-astra-token") {
        if let Ok(token_str) = astra_token.to_str() {
            if token == token_str {
                return format!("Executed. Token: {}", token);
            } else {
                return "Unauthorized".to_string();
            }
        } else {
            return "Unauthorized".to_string();
        }
    } else {
        return "Unauthorized".to_string();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !std::path::Path::new(".env").exists() {
        let token: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(32)
            .map(char::from)
            .collect(); // Unique token generation. Later, it will be used to verify x-astra-token header.

        println!(".env file not found. Initiating install process...");
        insert_custom_env("ASTRA-TOKEN", &token)?;

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
