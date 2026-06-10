# READMEにインストール方法を追加

## 目的
今含まれていないから追加したい。ReleaseからのダウンロードとHomebrewを想定しています。Tapは `sinofseven/luciferous-tap`です。

## 要望
READMEにインストール方法を追加して欲しい。

## プラン
Features セクションと Usage/使い方 セクションの間に Installation/インストール方法 セクションを追加。

### Installation セクション（英語版）
- From GitHub Releases: バイナリダウンロード方法と対応プラットフォーム情報
- Using Homebrew: tap + install コマンド

### インストール方法 セクション（日本語版）
- GitHub Releases からのダウンロード: バイナリダウンロード方法と対応プラットフォーム情報
- Homebrew でのインストール: tap + install コマンド

## 完了サマリー

**完了日時:** 2026-06-10T14:52:00+09:00

README.md と README_ja.md にインストール方法セクションを追加しました。

### 実施内容

1. **README.md**: Installation セクションを Features と Usage の間に追加
   - From GitHub Releases: GitHub Releases ページへのリンク、対応プラットフォーム一覧、セットアップ手順
   - Using Homebrew: `brew tap sinofseven/luciferous-tap` と `brew install aws-credential-process-provider-from-dotenv` コマンド

2. **README_ja.md**: インストール方法セクションを 機能 と 使い方 の間に追加
   - GitHub Releases からのダウンロード: GitHub Releases ページへのリンク、対応プラットフォーム一覧（日本語）、セットアップ手順
   - Homebrew でのインストール: 同じコマンド（日本語説明付き）

### 成果物

- `/Users/yuta/space/tools/aws-credential-process-provider-from-dotenv/README.md`: Installation セクション追加
- `/Users/yuta/space/tools/aws-credential-process-provider-from-dotenv/README_ja.md`: インストール方法セクション追加
- `/Users/yuta/space/tools/aws-credential-process-provider-from-dotenv/kanban/0002_add_readme_installation/log.md`: 作業ログ記録
