#!/bin/sh

contest='317'
task='b'

oj_download() {
    task=$(basename "$(pwd)")
    oj download "https://atcoder.jp/contests/abc${contest}/tasks/abc${contest}_${task}"
}

oj_test() {
    if [ ! -d 'test' ]; then
        oj_download
    fi

    task=$(basename "$(pwd)")
    if cargo build --bin "abc${contest}_${task}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/debug/abc${contest}_${task}"
    fi
}

oj_test_release() {
    if [ ! -d 'test' ]; then
        oj_download
    fi

    task=$(basename "$(pwd)")
    if cargo build -r --bin "abc${contest}_${task}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/release/abc${contest}_${task}"
    fi
}

oj_submit() {
    task=$(basename "$(pwd)")
    oj submit "https://atcoder.jp/contests/abc${contest}/tasks/abc${contest}_${task}" "${task}.rs" -w 1 --no-open
}
