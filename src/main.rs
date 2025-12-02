use clap::{arg, command};
use dotenvy::EnvLoader;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Output {
    version: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<String>,
}

fn main() {
    let matches = command!()
        .arg(arg!(<DOTENV_FILE>).required(true))
        .get_matches();

    let path = matches.get_one::<String>("DOTENV_FILE").unwrap();
    let env = EnvLoader::with_path(path)
        .load()
        .expect(&format!("failed to load .env file (path: {})", path));

    let output = Output {
        version: 1,
        access_key_id: env.get("AccessKeyId").map(|x| x.to_string()),
        secret_access_key: env.get("SecretAccessKey").map(|x| x.to_string()),
        session_token: env.get("SessionToken").map(|x| x.to_string()),
        expiration: env.get("Expiration").map(|x| x.to_string()),
    };

    let text = serde_json::to_string(&output).expect("failed to serialize output");
    println!("{}", text);
}
