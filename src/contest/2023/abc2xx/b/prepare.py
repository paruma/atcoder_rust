#!/usr/bin/env python3

# example
# ./contest.py abc206 b c

import os
import shutil
import argparse


def get_args():
    parser = argparse.ArgumentParser(description='ABC2xx B 用のファイル生成')
    parser.add_argument('contest_list', nargs='*')
    parser.add_argument('--only_toml', action='store_true')

    return parser.parse_args()


def get_path(contest_name: str) -> str:
    return f"src/contest/2023/abc2xx/b/{contest_name}.rs"


def make_files(contest_list: list[str]):
    """
    ファイルを作成する
    """

    # ソースコードの準備(ファイルのコピー)
    template_file_path: str = "src/contest/template.rs"
    for contest_name in contest_list:
        dst_file_path = get_path(contest_name)
        if not os.path.exists(dst_file_path):
            shutil.copy(template_file_path, dst_file_path)


def print_toml(contest_list: list[str]):
    """
    例
    [[bin]]
        name = abc2xx_b_200
        path = "src/contest/2023/abc2xx/b/200.rs"
    """
    for contest_name in contest_list:
        print(
            f"""[[bin]]
name = "abc2xx_b_{contest_name}"
path = "{get_path(contest_name)}"
        """
        )


def main():
    args = get_args()
    contest_list: str = args.contest_list
    only_toml: bool = args.only_toml

    if not args.only_toml:
        make_files(contest_list)

    # cargo.tomlコードの出力
    print_toml(contest_list)


if __name__ == '__main__':
    main()
