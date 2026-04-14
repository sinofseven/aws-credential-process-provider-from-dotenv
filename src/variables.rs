pub const PRODUCT_NAME: &str = "aws-credential-process-provider-from-dotenv";
pub const DEFAULT_CACHE_SECOND: u64 = 60;

pub const ALL_KEYS_ACCESS_KEY_ID: [&str; 6] = [
    "AccessKeyId",
    "ACCESS_KEY_ID",
    "access_key_id",
    "AwsAccessKeyId",
    "AWS_ACCESS_KEY_ID",
    "aws_access_key_id",
];

pub const ALL_KEYS_SECRET_ACCESS_KEY: [&str; 6] = [
    "SecretAccessKey",
    "SECRET_ACCESS_KEY",
    "secret_access_key",
    "AwsSecretAccessKey",
    "AWS_SECRET_ACCESS_KEY",
    "aws_secret_access_key",
];

pub const ALL_KEYS_SESSION_TOKEN: [&str; 6] = [
    "SessionToken",
    "SESSION_TOKEN",
    "session_token",
    "AwsSessionToken",
    "AWS_SESSION_TOKEN",
    "aws_session_token",
];

pub const ALL_KEYS_EXPIRATION: [&str; 3] = ["Expiration", "EXPIRATION", "expiration"];

fn to_pascal(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

pub fn build_keys(base: &[&str], prefix: Option<&str>, suffix: Option<&str>) -> Vec<String> {
    if prefix.is_none() && suffix.is_none() {
        return base.iter().map(|k| k.to_string()).collect();
    }

    base.iter()
        .enumerate()
        .map(|(i, key)| {
            let case_type = i % 3; // 0=PascalCase, 1=SCREAMING_SNAKE, 2=snake_case
            let mut result = String::new();

            if let Some(p) = prefix {
                match case_type {
                    0 => result.push_str(&to_pascal(p)),
                    1 => {
                        result.push_str(&p.to_uppercase());
                        result.push('_');
                    }
                    _ => {
                        result.push_str(&p.to_lowercase());
                        result.push('_');
                    }
                }
            }

            result.push_str(key);

            if let Some(s) = suffix {
                match case_type {
                    0 => result.push_str(&to_pascal(s)),
                    1 => {
                        result.push('_');
                        result.push_str(&s.to_uppercase());
                    }
                    _ => {
                        result.push('_');
                        result.push_str(&s.to_lowercase());
                    }
                }
            }

            result
        })
        .collect()
}
