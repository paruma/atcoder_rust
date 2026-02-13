# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
#     "markdownify",
# ]
# ///

import argparse
import re
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup, Tag
from markdownify import markdownify as md


@dataclass(frozen=True)
class EditorialMetaData:
    """解説のメタ情報を保持する。"""

    title: str
    url: str


def fetch_html(url: str) -> str:
    """指定されたURLからHTMLテキストを取得する。"""
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    return response.text


def extract_editorial_content(soup: BeautifulSoup) -> Tag:
    """HTMLから解説の本文を含むコンテナを抽出する。"""
    main_container = soup.find("div", id="main-container")
    if not main_container:
        raise ValueError("Could not find main container.")

    content_div = None

    for h2 in main_container.find_all("h2"):
        if "Editorial" in h2.get_text():
            content_div = h2.find_parent("div", class_="col-sm-12")
            break

    if not content_div:
        divs = main_container.find_all("div", class_="col-sm-12")
        for d in divs:
            if d.find("h2"):
                content_div = d
                break

    if not content_div:
        raise ValueError("Could not find editorial content.")

    return content_div


def unescape_math(text: str) -> str:
    r"""
    markdownify によってエスケープされた数式内のアンダースコアを元に戻す。
    \( ... \), \[ ... \], $$ ... $$ の内側を対象とする。
    """

    def replace_math(match: re.Match) -> str:
        # マッチした数式部分の \_ を _ に戻す
        return match.group(0).replace("\\_", "_")

    # インライン数式 \( ... \)
    text = re.sub(r"\\\(.*?\\\)", replace_math, text, flags=re.DOTALL)
    # ディスプレイ数式 \[ ... \]
    text = re.sub(r"\\\[.*?\\\]", replace_math, text, flags=re.DOTALL)
    # ディスプレイ数式 $$ ... $$
    text = re.sub(r"\$\$.*?\$\$", replace_math, text, flags=re.DOTALL)

    return text


def format_editorial_as_markdown(metadata: EditorialMetaData, content_tag: Tag) -> str:
    """markdownify を使用して解説を Markdown 形式に整形し、数式のエスケープを解除する。"""
    markdown_content = md(
        str(content_tag),
        strip=["script", "style", "noscript"],
        heading_style="ATX",
        newline_style="backslash",
    )

    # 数式内の過剰なエスケープを解除
    markdown_content = unescape_math(markdown_content)

    lines = [f"# {metadata.title}\n"]
    lines.append(f"Source: {metadata.url}\n")
    lines.append("---\n")
    lines.append(markdown_content)

    return "\n".join(lines)


def run_fetch_editorial(url: str) -> None:
    """解説の取得と表示を実行する。"""
    html = fetch_html(url)
    soup = BeautifulSoup(html, "html.parser")

    h2 = soup.find("h2")
    title = h2.get_text().strip() if h2 else "Editorial"
    metadata = EditorialMetaData(title=title, url=url)

    content_tag = extract_editorial_content(soup)
    markdown = format_editorial_as_markdown(metadata, content_tag)
    print(markdown)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Fetch AtCoder editorial and format as Markdown."
    )
    parser.add_argument("url", help="URL of the AtCoder editorial")
    args = parser.parse_args()

    run_fetch_editorial(args.url)


if __name__ == "__main__":
    main()
