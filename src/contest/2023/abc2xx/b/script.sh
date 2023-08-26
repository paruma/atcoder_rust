#!/bin/sh

task='b'

oj_download() {
    contest=$(basename "$(pwd)")
    oj download "https://atcoder.jp/contests/abc${contest}/tasks/abc${contest}_${task}"
}

oj_test() {
    if [ ! -d 'test' ]; then
        oj_download
    fi

    contest=$(basename "$(pwd)")
    if cargo build --bin "abc2xx_${task}_${contest}"; then
        export RUST_BACKTRACE=1
        oj test -c "$(git rev-parse --show-toplevel)/target/debug/abc2xx_${task}_${contest}"
    fi
}

oj_submit() {
    contest=$(basename "$(pwd)")
    oj submit "https://atcoder.jp/contests/abc${contest}/tasks/abc${contest}_${task}" "${contest}.rs" -w 1 --no-open
}
