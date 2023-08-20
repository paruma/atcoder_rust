#!/usr/bin/env python3

import os
import shutil
import argparse


def get_args():
    parser = argparse.ArgumentParser(description='ABC2xx B 用のファイル生成')
    parser.add_argument('contest_list', nargs='*')
    parser.add_argument('--only_toml', action='store_true')

    return parser.parse_args()


def make_files(contest_name: str):
    """
    ファイルを作成する
    """

    # ソースコードの準備(ファイルのコピー)
    template_file_path: str = "../../../template.rs"
    dst_directory_path = contest_name
    dst_file_path = f'{contest_name}/{contest_name}.rs'
    if not os.path.exists(dst_directory_path):
        os.mkdir(dst_directory_path)

    shutil.copy(template_file_path, dst_file_path)


def print_toml(contest_name: str):
    """
    出力例
    [[bin]]
        name = abc2xx_b_221
        path = "src/contest/2023/abc2xx/b/221/221.rs"
    """
    print(
        f"""[[bin]]
name = "abc2xx_b_{contest_name}"
path = "src/contest/2023/abc2xx/b/{contest_name}/{contest_name}.rs"
    """
    )


def main():
    args = get_args()
    contest_list: list[str] = args.contest_list
    only_toml: bool = args.only_toml

    for contest_name in contest_list:
        if not only_toml:
            make_files(contest_name)

        # cargo.tomlコードの出力
        print_toml(contest_name)


if __name__ == '__main__':
    main()
