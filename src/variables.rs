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
