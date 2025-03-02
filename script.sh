#!/bin/bash

PYTHONSTARTUP="$(git rev-parse --show-toplevel)/calc.py"
export PYTHONSTARTUP

_get_task() {
    basename "$(pwd)"
}

_get_contest() {
    contest_path=$(dirname "$(pwd)")
    basename "$contest_path"
}

oj_download() {
    contest="$(_get_contest)"
    task="$(_get_task)"
    oj download "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}"
}

_get_executable_path() {
    local contest=$(_get_contest)
    local task=$(_get_task)
    echo "$(git rev-parse --show-toplevel)/target/debug/${contest}_${task}"
}

_get_release_path() {
    local contest=$(_get_contest)
    local task=$(_get_task)
    echo "$(git rev-parse --show-toplevel)/target/release/${contest}_${task}"
}

_on_after_test() {
    if ! _check_dbg_output; then
        echo
        printf '\033[33m%s\033[m\n' 'WARNING: デバッグ出力が残ってる'
    fi
}

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

_exe_common() {
    local bin_path=$1
    local contest=$(_get_contest)
    local task=$(_get_task)
    if cargo build --bin "${contest}_${task}"; then
        export RUST_BACKTRACE=1
        "$bin_path"
    fi
}

_check_dbg_output() {
    local task=$(_get_task)
    if grep -Pq '^(?!.*//.*(dbg|lg)!).*(dbg|[^:]lg)!' "${task}.rs"; then
        return 1
    fi
}

oj_submit() {
    local contest=$(_get_contest)
    local task=$(_get_task)
    oj submit "https://atcoder.jp/contests/${contest}/tasks/${contest}_${task}" "${task}.rs" -w 1 --no-open
}

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

make_test() {
    mkdir -p 'test'

    local name=$1
    touch "test/${name}.in" "test/${name}.out"
}

alias ojs='oj_submit'
alias ojt='oj_test'
