# WRKFLW

[![Crates.io](https://img.shields.io/crates/v/wrkflw)](https://crates.io/crates/wrkflw)
[![Rust Version](https://img.shields.io/badge/rust-1.67%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/wrkflw)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/bahdotsh/wrkflw/build.yml?branch=main)](https://github.com/bahdotsh/wrkflw/actions/workflows/build.yml)

GitHub ActionsワークフローやGitLab CI/CDパイプラインを、ローカル環境で検証・実行するCLIツール。

![WRKFLW Demo](demo.gif)

## 概要

WRKFLWは、GitHubやGitLabにプッシュする前に、CI/CDワークフローをローカルで開発・テストできるようにします。ワークフローファイルを解析し、構文を検証し、Dockerコンテナまたはネイティブエミュレーションで依存関係順にジョブを実行します。

**主なメリット:**

- フィードバックループを高速化—リモートへのコミットなしでワークフローをテスト可能
- CI/CDに到達する前に設定エラーを検出
- コンテナ検査でインタラクティブにデバッグ
- GitHub ActionsとGitLab CI/CDの両方をサポート

## インストール

### Cargo経由（推奨）

```bash
cargo install wrkflw
```

### ソースからビルド

```bash
git clone https://github.com/bahdotsh/wrkflw.git
cd wrkflw
cargo build --release
# バイナリは target/release/wrkflw にあります
```

## クイックスタート

```bash
# プロジェクトディレクトリへ移動
cd your-project

# TUI起動（.github/workflowsを自動検出）
wrkflw

# またはワークフローを検証
wrkflw validate

# または特定のワークフローを実行
wrkflw run .github/workflows/ci.yml
```

## 使い方

### 検証

ワークフローの構文と構造を検証します:

```bash
# .github/workflows内の全ワークフローを検証
wrkflw validate

# 特定のファイルを検証
wrkflw validate .github/workflows/ci.yml

# 詳細出力で検証
wrkflw validate --verbose path/to/workflow.yml

# GitLab CIパイプラインを検証
wrkflw validate .gitlab-ci.yml --gitlab
```

**終了コード:** `0`（成功）、`1`（検証失敗）、`2`（使用法エラー）

### 実行

ワークフローをローカルで実行します:

```bash
# Dockerで実行（デフォルト）
wrkflw run .github/workflows/ci.yml

# セキュアエミュレーションモードで実行（サンドボックス済み、コンテナ不要）
wrkflw run --runtime secure-emulation .github/workflows/ci.yml

# 詳細出力で実行
wrkflw run --verbose .github/workflows/ci.yml

# デバッグ用に失敗したコンテナを保持
wrkflw run --preserve-containers-on-failure .github/workflows/ci.yml
```

### TUIインターフェース

```bash
# TUI起動（デフォルト: .github/workflows）
wrkflw tui

# カスタムパスを指定
wrkflw tui path/to/workflows

# 特定のランタイムモードで起動
wrkflw tui --runtime emulation
```

**TUIキーバインド:**

| キー | アクション |
|------|-----------|
| `Tab` / `1-4` | タブ切り替え |
| `↑↓` / `j/k` | 移動 |
| `Space` | 選択切り替え |
| `Enter` | 実行 / 詳細表示 |
| `r` | 選択項目を実行 |
| `e` | ランタイムモード切替 |
| `q` | 終了 |

### リモートトリガー

GitHub/GitLabでワークフローをトリガーします:

```bash
# GitHubワークフロー（GITHUB_TOKENが必要）
export GITHUB_TOKEN=ghp_your_token
wrkflw trigger workflow-name --branch main --input key=value

# GitLabパイプライン（GITLAB_TOKENが必要）
export GITLAB_TOKEN=glpat_your_token
wrkflw trigger-gitlab --branch main --variable key=value
```

## ランタイムモード

| モード | 分離レベル | 用途 |
|--------|-----------|------|
| **Docker** | コンテナ | CI環境に最も近い；全アクションタイプをサポート |
| **Secure Emulation** | サンドボックス化プロセス | ローカル開発；信頼できないワークフローも安全 |
| **Emulation** | なし（⚠️ 安全でない） | レガシー；非推奨 |

### Dockerモード

```bash
wrkflw run --runtime docker .github/workflows/ci.yml
```

- GitHub Actionsとの完全な互換性
- Dockerコンテナアクションをサポート
- サービスコンテナをサポート

### セキュアエミュレーションモード

```bash
wrkflw run --runtime secure-emulation .github/workflows/ci.yml
```

- コンテナランタイム不要
- 危険な操作をブロック（`rm -rf /`、`sudo` など）
- リソース制限（CPU、メモリ、実行時間）
- ローカル開発に最適

### エミュレーションモード（レガシー）

```bash
wrkflw run --runtime emulation .github/workflows/ci.yml
```

- コンテナランタイム不要
- セキュリティ保護なし—**信頼できるワークフローのみ使用**
- Dockerコンテナアクションをサポートしない

## 機能

### サポートされている機能

- ✅ ワークフロー検証（適切な終了コード付き）
- ✅ ジョブ依存関係解決（`needs` キーワード）
- ✅ 並列ジョブ実行
- ✅ マトリックスビルド
- ✅ 環境変数とGitHubコンテキスト
- ✅ Dockerコンテナアクション（Dockerモードのみ）
- ✅ JavaScriptアクション
- ✅ コンポジットアクション（ネスト含む）
- ✅ ローカルアクション
- ✅ 再利用可能なワークフロー（`jobs.<id>.uses`経由の呼び出しジョブ）
- ✅ `actions/checkout` のネイティブ処理
- ✅ 環境ファイル（`GITHUB_OUTPUT`、`GITHUB_ENV`、`GITHUB_PATH`、`GITHUB_STEP_SUMMARY`）
- ✅ リモートワークフロートリガー
- ✅ GitLab CI/CDパイプラインの検証とトリガー

### サポートされていない機能

- ❌ GitHubシークレット（環境変数を使用してください）
- ❌ Actionsキャッシュ（`actions/cache`）
- ❌ アーティファクトのアップロード/ダウンロード
- ❌ Windows/macOSランナー（Linuxのみ）
- ❌ エミュレーションモードでのサービスコンテナ
- ❌ ジョブ/ステップのタイムアウト強制
- ❌ 同時実行制限
- ❌ `workflow_dispatch`以外のイベントトリガー

## システム要件

- **Rust**: 1.67以上（ソースからビルドする場合）
- **Docker**: オプションだが推奨（Dockerモードの場合）

## 使用例

### ワークフローの検証

```bash
$ wrkflw validate .github/workflows/ci.yml
Validating 1 workflow file(s)...
✅ Valid: .github/workflows/ci.yml

Summary: 1 valid, 0 invalid
```

### ワークフローの実行

```bash
$ wrkflw run .github/workflows/ci.yml

Executing workflow: .github/workflows/ci.yml
============================================================
Runtime: Docker
------------------------------------------------------------

✅ Job succeeded: build

------------------------------------------------------------
  ✅ Checkout code
  ✅ Set up Rust
  ✅ Build
  ✅ Run tests

✅ Workflow completed successfully!
```

### 再利用可能なワークフロー

```yaml
jobs:
  call-shared:
    uses: ./.github/workflows/shared.yml
    with:
      config: production
    secrets:
      token: ${{ secrets.MY_TOKEN }}
```

## 環境ファイル

WRKFLWはGitHubの特殊な環境ファイルをサポートしています:

```bash
# ステップ出力
echo "result=value" >> "$GITHUB_OUTPUT"

# 環境変数
echo "VAR=value" >> "$GITHUB_ENV"

# PATH変更
echo "/path/to/bin" >> "$GITHUB_PATH"

# ステップサマリー（Markdown）
echo "## Summary" >> "$GITHUB_STEP_SUMMARY"
```

## 失敗したコンテナのデバッグ

検査用に失敗したコンテナを保持します:

```bash
wrkflw run --preserve-containers-on-failure .github/workflows/build.yml
```

ジョブが失敗すると、WRKFLWはコンテナを実行したままにします:

```
Preserving container abc123 for debugging (exit code: 1).
Use 'docker exec -it abc123 bash' to inspect.
```

## コントリビューション

コントリビューションをお待ちしています！[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。

## ライセンス

[MIT License](LICENSE)
