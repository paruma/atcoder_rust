#!/bin/bash

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
    if [ ! -d 'test' ]; then
        oj_download
    fi

    if cargo build --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/debug/${contest}_${task}"
    fi
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
        oj test -c "$(git rev-parse --show-toplevel)/target/release/${contest}_${task}"  -e 1e-6
    fi
}

oj_submit() {
    contest="$(get_contest)"
    task="$(get_task)"
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
