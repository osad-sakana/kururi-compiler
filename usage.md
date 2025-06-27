# Kururi Compiler 使用ガイド

## 概要

Kururi Compilerは、Kururi言語のコードをPythonに変換するプログラミング言語コンパイラです。
統合されたHTTPサービスと、簡単なコンパイルワークフローのためのPythonオーケストレーターの両方を提供します。

## 前提条件

### システム要件

- **Docker & Docker Compose** - コンテナ化されたコンパイルサービス用
- **Python 3.13+** - オーケストレーターとパイプライン実行用
- **Rust 1.83.0+** - ローカル開発用（オプション）

### インストール

1. リポジトリをクローンする
2. Dockerが動作していることを確認する
3. オーケストレーターを直接使用する場合はPython依存関係をインストールする

## クイックスタート

### 方法1: Docker Compose + Orchestrator（推奨）

```bash
# 1. 統合コンパイラサービスを開始
docker-compose up --build -d

# 2. KururiファイルをPythonにコンパイル
cd orchestrator
python run_pipeline.py ../example.kururi output.py

# 3. 生成されたPythonファイルを確認
cat output.py
```

### 方法2: ローカル開発

```bash
# 1. コンパイラをローカルでビルド
cd compiler
cargo build

# 2. HTTPサービスを開始
cargo run

# 3. 別のターミナルでオーケストレーターを使用
cd orchestrator
python run_pipeline.py ../example.kururi output.py
```

### 方法3: 直接HTTP API

```bash
# 1. サービスを開始
docker-compose up -d

# 2. curlを使用して直接コンパイル
curl -X POST http://localhost:8080/compile \
  -H "Content-Type: application/json" \
  -d '{"code": "function main(): void{ const msg: string = \"Hello\" output(msg) }"}'
```

## Kururi言語の構文

### 基本構造

```kururi
function main(): void{
    const variable_name: string = "Hello World by Kururi!"
    output(variable_name)
}
```

### サポートされている機能

- **関数**: `function name(): return_type { ... }`
- **変数**: `const name: type = value`
- **型**: `string`, `void`
- **組み込み関数**: `output()`
- **文字列リテラル**: `"text"`

### サンプルプログラム

#### Hello World

```kururi
function main(): void{
    const greeting: string = "Hello, Kururi!"
    output(greeting)
}
```

生成されるPython：

```python
def main():
    greeting = "Hello, Kururi!"
    print(greeting)

if __name__ == "__main__":
    main()
```

## HTTP API リファレンス

コンパイラサービスは `http://localhost:8080` で動作し、以下のエンドポイントを提供します。

### 完全コンパイレーション

```bash
POST /compile
Content-Type: application/json

{
  "code": "function main(): void{ const msg: string = \"Hello\" output(msg) }"
}
```

**レスポンス：**

```json
{
  "code": "def main():\n    msg = \"Hello\"\n    print(msg)\n\nif __name__ == \"__main__\":\n    main()",
  "tokens": [],
  "ast": {"Program": []},
  "checked_ast": {"Program": []}
}
```

### 個別ステップ（デバッグ用）

#### 字句解析

```bash
POST /lex
{"code": "const x: string = \"hello\""}
```

#### 構文解析

```bash
POST /parse
{"tokens": ["const", "x", ":", "string", "=", "\"hello\""]}
```

#### 意味解析

```bash
POST /semantic
{"ast": [...]}
```

#### コード生成

```bash
POST /codegen
{"checked_ast": [...]}
```

## エラーハンドリング

コンパイラは提案付きの詳細なエラーメッセージを提供します。

```json
{
  "error": "Semantic analysis error: Undefined variable: x",
  "error_type": "semantic_error",
  "details": "Error occurred during semantic analysis",
  "suggestions": [
    "Make sure the variable is declared before use"
  ]
}
```

### 一般的なエラータイプ

- **lexical_error**: 無効な文字や構文
- **parse_error**: 文法違反
- **semantic_error**: 型の不一致、未定義変数/関数
- **codegen_error**: 内部コード生成の問題

## 開発コマンド

### ビルドとテスト

```bash
# コンパイラをビルド
cd compiler
cargo build

# 全テストを実行
cargo test

# 出力付きでテストを実行
cargo test -- --nocapture

# 警告をチェック
cargo clippy
```

### Docker操作

```bash
# サービスをビルドして開始
docker-compose up --build -d

# ログを表示
docker-compose logs compiler

# サービスを停止
docker-compose down

# コンパイラのみ再ビルド
docker-compose build compiler
```

### オーケストレーター使用法

```bash
cd orchestrator

# 基本的なコンパイレーション
python run_pipeline.py input.kururi output.py

# オーケストレーターは自動的に以下を実行：
# 1. 入力Kururiファイルを読み取り
# 2. コードをコンパイラサービスに送信
# 3. 生成されたPythonを出力ファイルに書き込み
```

## ファイル構造

```text
kururi-compiler/
├── compiler/              # Rust HTTPコンパイラサービス
│   ├── src/
│   │   ├── main.rs        # HTTPサーバーエントリーポイント
│   │   ├── lexer.rs       # トークン化
│   │   ├── parser_new.rs  # 構文解析
│   │   ├── semantic.rs    # 型チェック
│   │   ├── codegen.rs     # Pythonコード生成
│   │   └── ...
│   ├── Cargo.toml
│   └── Dockerfile
├── orchestrator/          # Python調整サービス
│   ├── run_pipeline.py    # メインパイプラインスクリプト
│   └── pyproject.toml
├── example.kururi         # サンプルKururiプログラム
├── docker-compose.yml     # サービスオーケストレーション
└── usage.md              # このファイル
```

## パフォーマンス

- **コンパイル速度**: 小さなプログラムで約100ms
- **メモリ使用量**: サービスで約10MB RAM
- **コンテナサイズ**: 約50MB（最適化されたマルチステージビルド）

## トラブルシューティング

### よくある問題

#### Dockerサービスが開始しない

```bash
# Dockerが動作しているかチェック
docker info

# ポートの可用性をチェック
lsof -i :8080

# 新しいイメージで再ビルド
docker-compose build --no-cache compiler
```

#### コンパイレーションエラー

```bash
# サービスログをチェック
docker-compose logs compiler

# 最小プログラムでテスト
echo 'function main(): void{ output("test") }' > test.kururi
python run_pipeline.py test.kururi test.py
```

#### Python依存関係

```bash
# requestsが不足している場合はインストール
pip install requests

# または システムパッケージマネージャーを使用
brew install python-requests  # macOS
```

### デバッグモード

詳細なデバッグには、コンパイラをローカルで実行します。

```bash
cd compiler
RUST_LOG=debug cargo run
```

## 貢献

### 言語機能の追加

1. **Lexer**: `token.rs`に新しいトークンを追加
2. **Parser**: `parser_new.rs`で文法を拡張
3. **Semantic**: `semantic.rs`で型チェックを追加
4. **Codegen**: `codegen.rs`でPython生成を追加
5. **Tests**: 新機能のテストケースを追加

### テスト

```bash
# 特定のテストを実行
cargo test test_name

# HTTPエンドポイントをテスト
cargo test test_lex_handler

# 完全なパイプラインをテスト
cargo test test_compile_ast_example_kururi
```

## 例

### 変数宣言

```kururi
function main(): void{
    const name: string = "Kururi"
    const message: string = "Hello from Kururi!"
    output(name)
    output(message)
}
```

### 生成される出力

```python
def main():
    name = "Kururi"
    message = "Hello from Kururi!"
    print(name)
    print(message)

if __name__ == "__main__":
    main()
```

## API統合

他のアプリケーションにコンパイラを統合する場合の例です。

```python
import requests

def compile_kururi(code):
    response = requests.post(
        'http://localhost:8080/compile',
        json={'code': code}
    )
    if response.status_code == 200:
        return response.json()['code']
    else:
        error = response.json()
        print(f"Error: {error['error']}")
        print(f"Suggestions: {error['suggestions']}")
        return None

# 使用方法
python_code = compile_kururi('function main(): void{ output("Hello") }')
print(python_code)
```

## 参考資料

より詳細な技術情報については、プロジェクトルートの `CLAUDE.md` を参照してください。