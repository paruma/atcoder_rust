#!/bin/bash

PYTHONSTARTUP="$(git rev-parse --show-toplevel)/calc.py"
export PYTHONSTARTUP

get_task(){
    basename "$(pwd)"
}

get_contest(){
    wd=$(pwd)
    contest_path=${wd%/*}
    basename "$contest_path"
}


oj_download() {
    contest="$(get_contest)"
    task="$(get_task)"
    oj download "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}"
}

oj_test() {
    contest="$(get_contest)"
    task="$(get_task)"
    echo $contest
    echo $task
    if [ ! -d 'test' ]; then
        oj_download
    fi

    if cargo build --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/debug/${contest}_${task}"
    fi

    on_after_test
}

oj_test_f() {
    contest="$(get_contest)"
    task="$(get_task)"
    if [ ! -d 'test' ]; then
        oj_download
    fi

    if cargo build --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/debug/${contest}_${task}" -e 1e-6
    fi

    on_after_test
}

ojt() {
    oj_test
}

oj_test_release() {
    contest="$(get_contest)"
    task="$(get_task)"
    if [ ! -d 'test' ]; then
        oj_download
    fi

    if cargo build -r --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/release/${contest}_${task}"
    fi

    on_after_test
}

on_after_test() {
    if ! check_mod; then
        echo 
        printf '\033[33m%s\033[m\n' 'WARNING: 問題文には mod があるけど、ソースコードには mod がない'
    fi

    if ! check_dbg_output; then
        echo 
        printf '\033[33m%s\033[m\n' 'WARNING: デバッグ出力が残ってる'
    fi
}

check_mod() {
    contest="$(get_contest)"
    task="$(get_task)"
    
    if curl -s "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}" | grep -q -e "で割ったあまりを求めてください" -e "で割った余りを求めてください"; then
        if ! grep -q -e "ModInt" "${task}.rs"; then
            return 1
        fi
    fi
}

check_dbg_output() {
    task="$(get_task)"
    if grep -Pq '^(?!.*//.*(dbg|lg)!).*(dbg|[^:]lg)!' "${task}.rs"; then
        return 1
    fi
}


oj_submit() {
    contest="$(get_contest)"
    task="$(get_task)"
    if ! check_mod; then
        echo "問題文には mod があるけど、ソースコードには mod がない"
        return 1
    fi
    oj submit "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}" "${task}.rs" -w 1 --no-open
}

ojs() {
    oj_submit
}

exe() {
    contest="$(get_contest)"
    task="$(get_task)"

    if cargo build --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        "$(git rev-parse --show-toplevel)/target/debug/${contest}_${task}"
    fi
}
