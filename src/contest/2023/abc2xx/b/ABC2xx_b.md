# ABC 2xx B

## 手順

```sh
# top level で
pipenv shell
# cd src/contest/2023/abc2xx/b
python prepare.py {230..239}
source script.sh

# ABC230 の場合
cd 230
# 実装をする
oj_download
oj_test
oj_submit
```

## メモ
* 初回配信の編集をする
* ミスした内容を記録していきたい

`oj_test` のときに、backtrack があまり出てこない
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
