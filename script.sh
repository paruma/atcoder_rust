#!/bin/bash

# AtCoder 開発補助ツール
#
# このスクリプトを `source script.sh` で読み込むことで、oj test (ojt) や
# oj submit (ojs)、ビルド・実行 (exe) などのコマンドが現在のシェルで利用可能になります。
# ロジックの実体は atcoder_tool.py に集約されています。

_ATCODER_ROOT="$(git rev-parse --show-toplevel)"

PYTHONSTARTUP="${_ATCODER_ROOT}/calc.py"
export PYTHONSTARTUP

export RUST_BACKTRACE=1

ojt() { python3 "${_ATCODER_ROOT}/atcoder_tool.py" test "$@"; }
ojs() { python3 "${_ATCODER_ROOT}/atcoder_tool.py" submit "$@"; }
exe() { python3 "${_ATCODER_ROOT}/atcoder_tool.py" exe "$@"; }

if [ -n "$ZSH_VERSION" ]; then
    source "${_ATCODER_ROOT}/completion.zsh"
fi

# あらかじめライブラリをビルドしておいて、ビルド時間を短縮させる
cargo build -p atcoder_rust_2026 2>/dev/null &
cargo build -p atcoder_rust_2026 --release 2>/dev/null &
