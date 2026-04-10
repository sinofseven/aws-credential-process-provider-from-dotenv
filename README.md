[日本語](./README_ja.md)

# aws-credential-process-provider-from-dotenv

A Rust CLI tool that reads AWS credentials from `.env` files and outputs them in the JSON format required by AWS CLI's [`credential_process`](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sourcing-external.html). Parsed credentials can optionally be cached in Redis.

This tool was created to use `.env` files mounted by [1Password Environments](https://developer.1password.com/docs/environment) as AWS credentials via `credential_process`.

## Features

- Reads AWS credentials (`AccessKeyId`, `SecretAccessKey`, `SessionToken`, `Expiration`) from `.env` files
- Outputs `credential_process`-compatible JSON (Version 1)
- Supports 6 naming conventions per credential field (PascalCase, SCREAMING_SNAKE_CASE, snake_case, and their `Aws`/`AWS_`/`aws_` prefixed variants)
- Optional Redis caching with configurable TTL (default: 60 seconds)
- No AWS SDK dependency — pure credential file parsing

## Usage

### Basic Usage

```sh
aws-credential-process-provider-from-dotenv /path/to/credentials.env
```

This reads the `.env` file and prints credential JSON to stdout.

### AWS CLI Integration

Add the following to your `~/.aws/config`:

```ini
[profile my-profile]
credential_process = /path/to/aws-credential-process-provider-from-dotenv /path/to/credentials.env
```

You can then verify the integration:

```sh
aws sts get-caller-identity --profile my-profile
```

### .env File Format

```env
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
AWS_SESSION_TOKEN=FwoGZX...
EXPIRATION=2024-01-01T00:00:00Z
```

`SessionToken` and `Expiration` are optional. If not present in the `.env` file, they are omitted from the JSON output.

## Configuration

### Redis Caching

Create a config file at `~/.config/aws-credential-process-provider-from-dotenv/config.toml`:

```toml
[redis]
connection_string = "redis://127.0.0.1/"
cache_second = 120
```

| Field | Required | Description |
|---|---|---|
| `connection_string` | Yes | Redis connection URL |
| `cache_second` | No | Cache TTL in seconds (default: 60) |

If the config file does not exist, Redis caching is disabled and the tool runs without it.

The cache key format is: `aws-credential-process-provider-from-dotenv:<expanded-path>`

## Output Format

```json
{
  "Version": 1,
  "AccessKeyId": "AKIAIOSFODNN7EXAMPLE",
  "SecretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
  "SessionToken": "FwoGZX...",
  "Expiration": "2024-01-01T00:00:00Z"
}
```

Fields with no value in the `.env` file are omitted from the output (not set to `null`).

## Supported .env Key Names

The tool tries each key name in the order listed below and uses the first match found.

| Field | Accepted Key Names |
|---|---|
| Access Key ID | `AccessKeyId`, `ACCESS_KEY_ID`, `access_key_id`, `AwsAccessKeyId`, `AWS_ACCESS_KEY_ID`, `aws_access_key_id` |
| Secret Access Key | `SecretAccessKey`, `SECRET_ACCESS_KEY`, `secret_access_key`, `AwsSecretAccessKey`, `AWS_SECRET_ACCESS_KEY`, `aws_secret_access_key` |
| Session Token | `SessionToken`, `SESSION_TOKEN`, `session_token`, `AwsSessionToken`, `AWS_SESSION_TOKEN`, `aws_session_token` |
| Expiration | `Expiration`, `EXPIRATION`, `expiration` |

> **Note:** `Expiration` only supports 3 variants (no `Aws`/`AWS_`/`aws_` prefixed forms).

## Third-Party Licenses

Third-party license information is available in [third_party_licenses.html](./third_party_licenses.html).

## License

MIT
