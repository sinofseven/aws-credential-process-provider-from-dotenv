[English](./README.md)

# aws-credential-process-provider-from-dotenv

`.env` ファイルから AWS 認証情報を読み取り、AWS CLI の [`credential_process`](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sourcing-external.html) が要求する JSON 形式で出力する Rust 製 CLI ツールです。オプションで Redis によるキャッシュに対応しています。

[1Password Environments](https://developer.1password.com/docs/environment) でマウントされる `.env` ファイルを `credential_process` 経由で AWS 認証情報として利用するために作成しました。

## 機能

- `.env` ファイルから AWS 認証情報 (`AccessKeyId`、`SecretAccessKey`、`SessionToken`、`Expiration`) を読み取る
- `credential_process` 互換の JSON (Version 1) を出力する
- 各認証情報フィールドにつき 6 種類の命名規則に対応 (PascalCase、SCREAMING_SNAKE_CASE、snake_case、およびそれぞれの `Aws`/`AWS_`/`aws_` プレフィックス付き)
- オプションの Redis キャッシュ (TTL は設定可能、デフォルト: 60 秒)
- AWS SDK への依存なし — 純粋な認証情報ファイルの解析

## 使い方

### 基本的な使い方

```sh
aws-credential-process-provider-from-dotenv /path/to/credentials.env
```

`.env` ファイルを読み取り、認証情報の JSON を標準出力に出力します。

### AWS CLI との連携

`~/.aws/config` に以下を追記します。

```ini
[profile my-profile]
credential_process = /path/to/aws-credential-process-provider-from-dotenv /path/to/credentials.env
```

動作確認は以下のコマンドで行えます。

```sh
aws sts get-caller-identity --profile my-profile
```

### .env ファイルの形式

```env
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
AWS_SESSION_TOKEN=FwoGZX...
EXPIRATION=2024-01-01T00:00:00Z
```

`SessionToken` と `Expiration` はオプションです。`.env` ファイルに存在しない場合、JSON 出力から省略されます。

## 設定

### Redis キャッシュ

`~/.config/aws-credential-process-provider-from-dotenv/config.toml` を作成します。

```toml
[redis]
connection_string = "redis://127.0.0.1/"
cache_second = 120
```

| フィールド | 必須 | 説明 |
|---|---|---|
| `connection_string` | 必須 | Redis の接続 URL |
| `cache_second` | 任意 | キャッシュの TTL (秒)。デフォルト: 60 |

設定ファイルが存在しない場合、Redis キャッシュは無効になり、ツールはそのまま動作します。

キャッシュキーの形式: `aws-credential-process-provider-from-dotenv:<展開済みパス>`

## 出力形式

```json
{
  "Version": 1,
  "AccessKeyId": "AKIAIOSFODNN7EXAMPLE",
  "SecretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
  "SessionToken": "FwoGZX...",
  "Expiration": "2024-01-01T00:00:00Z"
}
```

`.env` ファイルに値がないフィールドは JSON 出力から省略されます (`null` にはなりません)。

## 対応するキー名一覧

以下の順にキー名を試し、最初に見つかったものを使用します。

| フィールド | 対応するキー名 |
|---|---|
| Access Key ID | `AccessKeyId`、`ACCESS_KEY_ID`、`access_key_id`、`AwsAccessKeyId`、`AWS_ACCESS_KEY_ID`、`aws_access_key_id` |
| Secret Access Key | `SecretAccessKey`、`SECRET_ACCESS_KEY`、`secret_access_key`、`AwsSecretAccessKey`、`AWS_SECRET_ACCESS_KEY`、`aws_secret_access_key` |
| Session Token | `SessionToken`、`SESSION_TOKEN`、`session_token`、`AwsSessionToken`、`AWS_SESSION_TOKEN`、`aws_session_token` |
| Expiration | `Expiration`、`EXPIRATION`、`expiration` |

> **注意:** `Expiration` のみ 3 種類の対応で、`Aws`/`AWS_`/`aws_` プレフィックス付きの形式はありません。

## サードパーティライセンス

サードパーティのライセンス情報は [third_party_licenses.html](./third_party_licenses.html) を参照してください。

## ライセンス

MIT
