#!/usr/bin/env python3

# example
# ./contest.py abc206 b c

import sys
import os
import shutil


contest_name: str = sys.argv[1]
problems: list[str] = sys.argv[2:]


# ソースコードの準備(ディレクトリの生成)
dir_path: str = f"src/contest/{contest_name}/"
if not os.path.exists(dir_path):
    os.mkdir(dir_path)

# ソースコードの準備(ファイルのコピー)
templete_file_path: str = "src/contest/template.rs"
for problem in problems:
    dst_file_path = f"src/contest/{contest_name}/main{problem}.rs"
    if not os.path.exists(dst_file_path):
        shutil.copy(templete_file_path, dst_file_path)


# cargo.tomlコードの出力
for problem in problems:
    print(
        f"""[[bin]]
name = "{contest_name}_{problem}"
path = "src/contest/{contest_name}/main{problem}.rs"
    """
    )
