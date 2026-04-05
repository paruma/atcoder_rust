# completion.zsh
# zsh 補完定義。source script.sh 経由で読み込まれる。

_atcoder_rs_suffixes() {
    local -a rs_files suffixes
    rs_files=( *.rs(N) )
    suffixes=( ${rs_files%.rs} )
    _describe 'rs suffix' suffixes
}

_ojt() {
    _arguments \
        '-f[浮動小数点許容誤差 1e-6 を設定]' \
        '-r[リリースビルドを使用する]' \
        '-b[バイナリ名サフィックス（振り返り実装用）]:suffix:_atcoder_rs_suffixes'
}

_exe_completion() {
    _arguments \
        '-r[リリースビルドを使用する]' \
        '-b[バイナリ名サフィックス（振り返り実装用）]:suffix:_atcoder_rs_suffixes'
}

compdef _ojt ojt
compdef _exe_completion exe
