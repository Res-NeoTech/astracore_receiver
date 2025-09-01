use dotenv::dotenv;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

pub fn insert_custom_env(env_name: &str, env_val: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(".env")?;

    writeln!(file, "{}={}", env_name, env_val)?;

    return Ok(());
}

pub fn get_env(env: &str, err_message: Option<String>) -> String {
    dotenv().ok();

    let msg = err_message.unwrap_or_else(|| "Target env is not set.".to_string());

    return env::var(env).expect(&msg);
}