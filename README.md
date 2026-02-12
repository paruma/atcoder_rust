# AtCoder Rust (paruma184)

https://atcoder.jp/users/paruma184

## スニペットの更新


```sh
sh snippet.sh
```

## oj の使い方

```sh
uv sync
source .venv/bin/activate
```

## oj の補助ツールの使い方

以下を実行すると、ojt, ojs などの oj 補助ツールが使える。

```sh
source script.sh
```

## oj でのログイン方法

aclogin を使う。詳細は以下のツイートを参照

https://x.com/kymn_/status/1901924409662685347

## gemini-cli 用イメージ

cargo が使えるイメージ。

1. イメージビルド

    ```sh
    docker build -t gemini-rust .gemini/
    ```

1. イメージを指定して gemini-cli を起動する

    ```sh
    GEMINI_SANDBOX_IMAGE="gemini-rust" gemini -s --yolo
    ```

## テストカバレッジ

ブラウザで見る

```sh
cargo llvm-cov --open --package mylib --lib -- --include-ignored
```


VSCode で見る

```sh
cargo llvm-cov --lcov --output-path lcov.info --package mylib --lib -- --include-ignored 
```
