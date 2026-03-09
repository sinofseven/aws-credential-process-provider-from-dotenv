use crate::models::configure::RedisConfig;
use crate::variables::{DEFAULT_CACHE_SECOND, PRODUCT_NAME};
use redis::{Client, Connection, TypedCommands};

pub fn connect(config: &RedisConfig) -> Result<Connection, String> {
    let client = Client::open(config.connection_string.clone())
        .map_err(|e| format!("failed to create client: {}", e))?;
    client
        .get_connection()
        .map_err(|e| format!("failed to connect redis: {}", e))
}

fn create_key(env_file_path: &str) -> String {
    let path = shellexpand::tilde(env_file_path);
    format!("{}:{}", PRODUCT_NAME, path)
}

pub fn get_value(env_file_path: &str, con: &mut Connection) -> Result<Option<String>, String> {
    let key = create_key(env_file_path);

    con.get(&key)
        .map_err(|e| format!("failed to get value (key: {}): {}", key, e))
}

pub fn set_value(
    env_file_path: &str,
    value: &str,
    config: &RedisConfig,
    con: &mut Connection,
) -> Result<(), String> {
    let key = create_key(env_file_path);
    let sec = if let Some(sec) = config.cache_second {
        sec
    } else {
        DEFAULT_CACHE_SECOND
    };

    con.set_ex(&key, value, sec)
        .map_err(|e| format!("failed to set value: {}", e))
}
