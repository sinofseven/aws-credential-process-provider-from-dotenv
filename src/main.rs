mod models;
mod redis;
mod variables;

use clap::{arg, command};
use dotenvy::{EnvLoader, EnvMap};

fn main() -> Result<(), String> {
    let config = crate::models::configure::Config::load()?;

    let matches = command!()
        .arg(arg!(<DOTENV_FILE>).required(true))
        .arg(arg!(--prefix <PREFIX> "Prefix to prepend to key names").required(false))
        .arg(arg!(--suffix <SUFFIX> "Suffix to append to key names").required(false))
        .get_matches();

    let path = matches.get_one::<String>("DOTENV_FILE").unwrap();
    let prefix = matches.get_one::<String>("prefix").map(|s| s.as_str());
    let suffix = matches.get_one::<String>("suffix").map(|s| s.as_str());

    let mut con = if let Some(redis_config) = &config.redis {
        let con = crate::redis::connect(redis_config)?;
        Some(con)
    } else {
        None
    };

    if let Some(con) = &mut con
        && let Some(value) = crate::redis::get_value(path, prefix, suffix, con)?
    {
        println!("{}", value);
        return Ok(());
    }

    let text = get_from_env_file(path, prefix, suffix)?;

    if let Some(con) = &mut con {
        crate::redis::set_value(path, prefix, suffix, &text, &config.redis.unwrap(), con)?;
    }

    println!("{}", text);

    Ok(())
}

fn get_from_env_file(
    env_file_path: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
) -> Result<String, String> {
    let env = EnvLoader::with_path(env_file_path)
        .load()
        .unwrap_or_else(|e| panic!("failed to load .env file (path: {}): {}", env_file_path, e));

    let output = crate::models::output::Output {
        version: 1,
        access_key_id: get_value(
            &env,
            &crate::variables::build_keys(&crate::variables::ALL_KEYS_ACCESS_KEY_ID, prefix, suffix),
        ),
        secret_access_key: get_value(
            &env,
            &crate::variables::build_keys(
                &crate::variables::ALL_KEYS_SECRET_ACCESS_KEY,
                prefix,
                suffix,
            ),
        ),
        session_token: get_value(
            &env,
            &crate::variables::build_keys(&crate::variables::ALL_KEYS_SESSION_TOKEN, prefix, suffix),
        ),
        expiration: get_value(
            &env,
            &crate::variables::build_keys(&crate::variables::ALL_KEYS_EXPIRATION, prefix, suffix),
        ),
    };

    serde_json::to_string(&output).map_err(|e| format!("failed to serialize output: {e}"))
}

fn get_value(env: &EnvMap, all_keys: &[String]) -> Option<String> {
    for key in all_keys {
        let value = env.get(key.as_str());
        if value.is_some() {
            return value.map(|x| x.to_string());
        }
    }

    None
}
