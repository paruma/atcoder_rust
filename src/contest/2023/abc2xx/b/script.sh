#!/bin/sh

task='b'

oj_download() {
    contest=$(basename "$(pwd)")
    oj download "https://atcoder.jp/contests/abc${contest}/tasks/abc${contest}_${task}"
}

oj_test() {
    contest=$(basename "$(pwd)")
    cargo build --bin "abc2xx_${task}_${contest}"
    oj test -c "$(git rev-parse --show-toplevel)/target/debug/abc2xx_${task}_${contest}"
}

oj_submit() {
    contest=$(basename "$(pwd)")
    oj submit "https://atcoder.jp/contests/abc${contest}/tasks/abc${contest}_${task}" "${contest}.rs"
}
