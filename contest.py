#!/usr/bin/env python3

# example
# ./contest.py abc206 b c

import sys
import os
import shutil
import argparse


def get_args():
    parser = argparse.ArgumentParser(description='コンテスト用のファイル生成')
    parser.add_argument('contest_name', type=str)
    parser.add_argument('task_name_list', nargs='*')
    parser.add_argument('--year', type=str, default='2023')
    parser.add_argument('--only_toml', action='store_true')

    return parser.parse_args()


def make_files(dir_path: str, problems: list[str]):
    """
    ディレクトリ（なければ）とファイルを作成する
    """
    if not os.path.exists(dir_path):
        os.mkdir(dir_path)

    # ソースコードの準備(ファイルのコピー)
    template_file_path: str = "src/contest/template.rs"
    for problem in problems:
        dst_file_path = f"{dir_path}{problem}.rs"
        if not os.path.exists(dst_file_path):
            shutil.copy(template_file_path, dst_file_path)


def print_toml(dir_path: str, contest_name: str, problems: list[str]):
    for problem in problems:
        print(
            f"""[[bin]]
    name = "{contest_name}_{problem}"
    path = "{dir_path}{problem}.rs"
        """
        )


def main():
    args = get_args()

    contest_name: str = args.contest_name
    problems: list[str] = args.task_name_list
    year: str = args.year

    dir_path: str = f"src/contest/{year}/{contest_name}/"
    if not args.only_toml:
        make_files(dir_path, problems)

    # cargo.tomlコードの出力
    print_toml(dir_path, contest_name, problems)


if __name__ == '__main__':
    main()
