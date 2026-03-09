mod models;
mod redis;
mod variables;

use clap::{arg, command};
use dotenvy::{EnvLoader, EnvMap};

fn main() -> Result<(), String> {
    let config = crate::models::configure::Config::load()?;

    let matches = command!()
        .arg(arg!(<DOTENV_FILE>).required(true))
        .get_matches();

    let path = matches.get_one::<String>("DOTENV_FILE").unwrap();

    let mut con = if let Some(redis_config) = &config.redis {
        let con = crate::redis::connect(redis_config)?;
        Some(con)
    } else {
        None
    };

    if let Some(con) = &mut con
        && let Some(value) = crate::redis::get_value(path, con)?
    {
        println!("{}", value);
        return Ok(());
    }

    let text = get_from_env_file(path)?;

    if let Some(con) = &mut con {
        crate::redis::set_value(path, &text, &config.redis.unwrap(), con)?;
    }

    println!("{}", text);

    Ok(())
}

fn get_from_env_file(env_file_path: &str) -> Result<String, String> {
    let env = EnvLoader::with_path(env_file_path)
        .load()
        .unwrap_or_else(|e| panic!("failed to load .env file (path: {}): {}", env_file_path, e));

    let output = crate::models::output::Output {
        version: 1,
        access_key_id: get_value(&env, &crate::variables::ALL_KEYS_ACCESS_KEY_ID),
        secret_access_key: get_value(&env, &crate::variables::ALL_KEYS_SECRET_ACCESS_KEY),
        session_token: get_value(&env, &crate::variables::ALL_KEYS_SESSION_TOKEN),
        expiration: get_value(&env, &crate::variables::ALL_KEYS_EXPIRATION),
    };

    serde_json::to_string(&output).map_err(|e| format!("failed to serialize output: {e}"))
}

fn get_value(env: &EnvMap, all_keys: &[&str]) -> Option<String> {
    for &key in all_keys {
        let value = env.get(key);
        if value.is_some() {
            return value.map(|x| x.to_string());
        }
    }

    None
}
