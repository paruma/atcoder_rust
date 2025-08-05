# AtCoder Rust (paruma184)

https://atcoder.jp/users/paruma184

## スニペットの更新


```sh
sh snippet.sh
```

## oj でのログイン方法

aclogin を使う。詳細は以下のツイートを参照

https://x.com/kymn_/status/1901924409662685347

## gemini-cli 用イメージ

cargo が使えるイメージ。

1. イメージビルド

    ```sh
    docker build -t gemini-rust .gemini/Dockerfile
    ```

1. イメージを指定して gemini-cli を起動する

    ```sh
    gemini -s --sandbox-image gemini-rust
    ```
