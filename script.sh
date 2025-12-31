#!/bin/bash

PYTHONSTARTUP="$(git rev-parse --show-toplevel)/calc.py"
export PYTHONSTARTUP

# カレントディレクトリ名からタスク名を取得します。
#
# 前提: カレントディレクトリがタスク名のディレクトリであること。
# (例: /path/to/contest/a の場合、 "a" を返す)
#
# 引数:
#   なし
# 出力:
#   タスク名を標準出力に書き出します。
_get_task() {
    basename "$(pwd)"
}

# カレントディレクトリの親ディレクトリ名からコンテスト名を取得します。
#
# 前提: カレントディレクトリがタスクのディレクトリであること。
# (例: /path/to/contest/a の場合、 "contest" を返す)
#
# 引数:
#   なし
# 出力:
#   コンテスト名を標準出力に書き出します。
_get_contest() {
    contest_path=$(dirname "$(pwd)")
    basename "$contest_path"
}

# AtCoder から問題のテストケースをダウンロードします。
#
# oj (online-judge-tools) を使用して、現在のコンテストとタスクに対応する
# テストケースをダウンロードし、'test' ディレクトリに保存します。
#
# 引数:
#   なし
# 出力:
#   'test' ディレクトリにテストケースファイルが生成されます。
oj_download() {
    contest="$(_get_contest)"
    task="$(_get_task)"
    oj download "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}"
}

# デバッグビルドで生成された実行ファイルのパスを取得します。
#
# 引数:
#   なし
# 出力:
#   実行ファイルのフルパスを標準出力に書き出します。
_get_executable_path() {
    local contest=$(_get_contest)
    local task=$(_get_task)
    echo "$(git rev-parse --show-toplevel)/target/debug/${contest}_${task}"
}

# リリースビルドで生成された実行ファイルのパスを取得します。
#
# 引数:
#   なし
# 出力:
#   実行ファイルのフルパスを標準出力に書き出します。
_get_release_path() {
    local contest=$(_get_contest)
    local task=$(_get_task)
    echo "$(git rev-parse --show-toplevel)/target/release/${contest}_${task}"
}

# テスト実行後に、デバッグ出力が残っていないかチェックします。
#
# _check_dbg_output を呼び出し、結果に応じて警告メッセージを表示します。
#
# 引数:
#   なし
# 出力:
#   デバッグ出力が残っている場合に警告メッセージを標準出力に表示します。
_on_after_test() {
    if ! _check_dbg_output; then
        echo
        printf '\033[33m%s\033[m\n' 'WARNING: デバッグ出力が残ってる'
    fi
}

# oj test を実行するための共通関数です。
#
# 'test' ディレクトリが存在しない場合は、先にテストケースをダウンロードします。
#
# 引数:
#   $1 (必須): 実行ファイルのパス。
#   $2 (任意): oj test に渡す追加の引数 (例: "-e 1e-6")。
# 出力:
#   oj test の実行結果を標準出力に表示します。
_oj_test_common() {
    local contest=$(_get_contest)
    local task=$(_get_task)

    if [ ! -d 'test' ]; then
        oj_download
    fi

    local bin_path=$1
    if [ -n "$2" ]; then
        oj test -c "$bin_path" --ignore-spaces-and-newline "$2"
    else
        oj test -c "$bin_path" --ignore-spaces-and-newline
    fi

    _on_after_test
}

# Rust コードをビルドし、oj を使ってテストを実行します。
#
# 引数:
#   -f: 浮動小数点数向けの許容誤差 (1e-6) を設定してテストを実行します。
#   -r: リリースビルドでテストを実行します。指定しない場合はデバッグビルドになります。
#
# 出力:
#   ビルドとテストの実行結果を標準出力に表示します。
oj_test() {
    local extra_args=""
    local bin_path
    local is_release=false
    local contest=$(_get_contest)
    local task=$(_get_task)
    
    while getopts "fr" opt; do
        case $opt in
            f)
                extra_args="-e 1e-6"
                ;;
            r)
                is_release=true
                ;;
            *)
                ;;
        esac
    done

    if [ "$is_release" = true ]; then
        if ! cargo build --release --bin "${contest}_${task}"; then
            return 1
        fi
        bin_path="$(_get_release_path)"
    else
        if ! cargo build --bin "${contest}_${task}"; then
            return 1
        fi
        bin_path="$(_get_executable_path)"
    fi

    _oj_test_common "$bin_path" "$extra_args"
}

# コードをビルドして実行するための共通ロジックです。
#
# 引数:
#   $1 (必須): 実行ファイルのパス。
# 出力:
#   cargo build の実行結果と、プログラムの実行結果を標準出力に表示します。
_exe_common() {
    local bin_path=$1
    local contest=$(_get_contest)
    local task=$(_get_task)
    if cargo build --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        "$bin_path"
    fi
}

# ソースコードに `dbg!` や `lg!` といったデバッグマクロが残っていないかチェックします。
#
# コメントアウトされている (`//`) デバッグマクロは無視します。
#
# 引数:
#   なし
# 返り値:
#   デバッグマクロが見つからなかった場合: 終了コード 0
#   デバッグマクロが見つかった場合: 終了コード 1
_check_dbg_output() {
    local task=$(_get_task)
    if grep -Pq '^(?!.*//.*(dbg|lg)!).*(dbg|[^:]lg)!' "${task}.rs"; then
        return 1
    fi
}

# oj を使って AtCoder にソースコードを提出します。
#
# 引数:
#   なし
# 出力:
#   oj submit の実行結果を標準出力に表示します。
oj_submit() {
    local contest=$(_get_contest)
    local task=$(_get_task)
    oj submit "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}" "${task}.rs" -w 1 --no-open
}

# Rust コードをビルドして実行します。
#
# 注意: リリースビルドで作成した実行ファイルにリダイレクトなしで
#       標準入力をすると、proconio との相性問題でうまく動作しない可能性があります。
#
# 引数:
#   -r (任意): リリースビルドで実行します。指定しない場合はデバッグビルドになります。
#
# 出力:
#   ビルドとプログラムの実行結果を標準出力に表示します。
exe() {
    # リリースビルドで作成した実行ファイルに対してリダイレクト無しで標準入力をした場合
    # proconio との相性が悪くてうまくいかない可能性がある。
    local bin_path
    local is_release=false
    local contest=$(_get_contest)
    local task=$(_get_task)

    if [ "$1" = "-r" ]; then
        is_release=true
    fi

    if [ "$is_release" = true ]; then
        if ! cargo build --release --bin "${contest}_${task}"; then
            return 1
        fi
        bin_path="$(_get_release_path)"
    else
        if ! cargo build --bin "${contest}_${task}"; then
            return 1
        fi
        bin_path="$(_get_executable_path)"
    fi

    "$bin_path"
}

# 新しいテストケースファイル (.in, .out) を作成します。
#
# 'test' ディレクトリが存在しない場合は作成します。
#
# 引数:
#   $1 (必須): 作成するテストケース名 (例: "sample1")。
# 出力:
#   'test' ディレクトリに <name>.in と <name>.out ファイルが作成されます。
make_test() {
    mkdir -p 'test'

    local name=$1
    touch "test/${name}.in" "test/${name}.out"
}

alias ojs='oj_submit'
alias ojt='oj_test'

# あらかじめライブラリをビルドしておいて、ビルド時間を短縮させる
cargo build -p atcoder_rust_2026 2> /dev/null &
cargo build -p atcoder_rust_2026 --release 2> /dev/null &
