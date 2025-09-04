//! # Environment Utilities
//!
//! This module provides helper functions for managing environment variables
//! in the AstraCore receiver application.
//!
//! ## Features
//! - Insert key-value pairs into a `.env` file.
/// - Load environment variables from `.env`.
use dotenv::dotenv;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

/// Inserts a custom environment variable into the `.env` file.
///
/// - If the `.env` file does not exist, it will be created.
/// - The key-value pair will be appended to the end of the file.
///
/// # Arguments
/// * `env_name` - The name of the environment variable (e.g., `"ASTRA_TOKEN"`).
/// * `env_val` - The value to assign to the environment variable.
///
/// # Errors
/// Returns an [`std::io::Error`] if the file cannot be opened or written.
///
/// # Example
/// ```
/// use your_crate::env::insert_custom_env;
///
/// insert_custom_env("ASTRA_TOKEN", "12345").unwrap();
/// ```
pub fn insert_custom_env(env_name: &str, env_val: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(".env")?;

    writeln!(file, "{}={}", env_name, env_val)?;

    return Ok(());
}

/// Retrieves an environment variable.
///
/// Loads variables from a `.env` file (using the [`dotenv`](https://crates.io/crates/dotenv) crate),
/// then attempts to fetch the requested environment variable.  ss
/// If the variable is missing, this function will panic with a custom error message if provided,
/// or with `"Target env is not set."` by default.
///
/// # Arguments
/// * `env` - The name of the environment variable to retrieve.
/// * `err_message` - Optional custom error message to display if the variable is not set.
///
/// # Panics
/// Panics if the environment variable is not set and no fallback exists.
///
/// # Example
/// ```
/// use your_crate::env::get_env;
///
/// let token = get_env("ASTRA_TOKEN", Some("ASTRA_TOKEN must be set!".to_string()));
/// ```
pub fn get_env(env: &str, err_message: Option<String>) -> String {
    dotenv().ok();

    let msg = err_message.unwrap_or_else(|| "Target env is not set.".to_string());

    return env::var(env).expect(&msg);
}