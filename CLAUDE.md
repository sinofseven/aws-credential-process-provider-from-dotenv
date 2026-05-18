# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 概要

`.env` ファイルからAWSクレデンシャルを読み取り、AWS CLIの `credential_process` 向けJSON形式で出力するRust CLIツール。Redisキャッシュのオプションと、カスタムキー名に対応する `--prefix`/`--suffix` フラグをサポートしている。

## コマンド

```sh
cargo build              # デバッグビルド
cargo build --release    # リリースビルド
cargo run -- /path/to/credentials.env [--prefix myapp] [--suffix dev]
cargo test               # テスト実行
cargo clippy             # lint
cargo fmt                # フォーマット
cargo deny check         # ライセンスチェック（about.toml で許可ライセンスを管理）
```

## アーキテクチャ

```
src/
  main.rs            # CLIエントリポイント: 引数解析、Redisキャッシュ確認、.env読み取り
  variables.rs       # キー名定数と prefix/suffix ロジックの build_keys()
  redis.rs           # Redis 接続・get/set（TTL付き）
  models/
    configure.rs     # ~/.config/aws-credential-process-provider-from-dotenv/config.toml の読み込み
    output.rs        # Output 構造体（PascalCase の credential_process JSON にシリアライズ）
```

### 重要な設計上のポイント

**`variables.rs` — `build_keys()`**: ベースのキー配列（`ALL_KEYS_*`）は常に `[PascalCase, SCREAMING_SNAKE, snake_case, AwsPascalCase, AWS_SCREAMING_SNAKE, aws_snake_case]` のサイクルで並んでいる。`build_keys()` は `i % 3` でケース種別を判定して prefix/suffix を適用する。新しいベースキー配列を追加するときはこの3要素サイクル順を守ること。順序が崩れると prefix/suffix のケース変換が誤動作する。

**`dotenvy` 依存**: crates.io ではなく GitHub の特定 rev を直接参照している。crates.io バージョンへの変更はAPI互換性を確認してから行うこと。

**Rust edition 2024**: `main.rs` で `let`-chain 構文（`if let … && let …`）を使用しており、edition 2024 が必要。

**Redisキャッシュキーの形式**: `aws-credential-process-provider-from-dotenv:<チルダ展開済みパス>[:prefix=<P>][:suffix=<S>]`

## 開発ワークフロー（kanban-kit）

このプロジェクトでは `kanban-kit` プラグインを使って開発作業を管理する。

### タスクの追加と実行

```
/add-kanban   # kanban/ ディレクトリに新規タスクファイルを作成
/kanban       # 未完了タスクのうち番号最大のものを実行（引数でタスク番号指定も可）
```

### ファイル構成

```
kanban/
  {xxxx}_{title}/
    {xxxx}_{title}.md   # タスクファイル（ユーザーが作成）
    log.md              # 作業ログ（Claude が記録）
```

- `xxxx` は4桁の0パディング連番（例: `0001`）
- 未完了タスク = `## 完了サマリー` セクションを含まないタスクファイル

### タスクファイルの構造

ユーザーが以下の2セクションを記述する。

```markdown
## 目的
（なぜこの作業が必要か — 背景・動機・ゴール）

## 要望
（具体的に何をどうしてほしいか）
```

### 実行フロー

1. **プランモード**: 実装計画を立ててユーザーの承認を得る
2. **実装**: 承認済みプランに沿って実装し、作業中も随時 `log.md` に追記する
3. **完了**: `log.md` の完了日時を更新し、タスクファイルに `## 完了サマリー` を追記する

### ログ記録の原則

- ログはインタラクティブセッションで得た情報・判断・経緯の**完全な記録**であり、要約・省略をしない
- 調べたファイルごとに発見した事実を具体的に書く
- タイムスタンプは JST の ISO 8601 形式: `TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"`
