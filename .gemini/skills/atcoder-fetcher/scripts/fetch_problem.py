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
from typing import Any

import requests
from bs4 import BeautifulSoup, Tag
from markdownify import MarkdownConverter


@dataclass(frozen=True)
class ProblemMetaData:
    """問題のメタ情報を保持する。"""

    title: str
    url: str
    time_limit: str | None
    memory_limit: str | None


class AtCoderConverter(MarkdownConverter):
    """AtCoder の HTML 構造に特化した Markdown 変換器。"""

    def convert_var(
        self, el: Tag, text: str, convert_as_inline: bool = True, **kwargs: Any
    ) -> str:
        """<var> タグを数式デリミタ $ で囲む。"""

        return f"${text}$"


def fetch_html(url: str) -> str:
    """指定されたURLからHTMLテキストを取得する。"""
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    return response.text


def extract_metadata(soup: BeautifulSoup, url: str) -> ProblemMetaData:
    """HTMLから問題のメタデータを抽出する。"""
    title_tag = soup.find("title")
    title = title_tag.text.strip() if title_tag else "Unknown Problem"

    text_content = soup.get_text()
    time_match = re.search(r"Time Limit:\s*(\d+(\.\d+)?\s*sec)", text_content)
    memory_match = re.search(r"Memory Limit:\s*(\d+\s*(?:MB|MiB))", text_content)

    return ProblemMetaData(
        title=title,
        url=url,
        time_limit=time_match.group(1) if time_match else None,
        memory_limit=memory_match.group(1) if memory_match else None,
    )


def unescape_math(text: str) -> str:
    r"""
    markdownify によってエスケープされた数式内のアンダースコアを元に戻す。
    \( ... \), \[ ... \], $$ ... $$, $ ... $ の内側を対象とする。
    """

    def replace_math(match: re.Match) -> str:
        return match.group(0).replace("\\_", "_")

    text = re.sub(r"\\\(.*?\\\)", replace_math, text, flags=re.DOTALL)
    text = re.sub(r"\\\[.*?\\\]", replace_math, text, flags=re.DOTALL)
    text = re.sub(r"\$\$.*?\$\$", replace_math, text, flags=re.DOTALL)
    text = re.sub(r"\$.*?\$", replace_math, text, flags=re.DOTALL)
    return text


def format_problem_as_markdown(metadata: ProblemMetaData, content_tag: Tag) -> str:
    """AtCoderConverter を使用して問題文を Markdown 形式に整形する。"""
    # カスタムコンバータを使用して変換
    markdown_content = AtCoderConverter(
        strip=["script", "style", "noscript"],
        heading_style="ATX",
        newline_style="backslash",
    ).convert_soup(content_tag)

    # 数式のエスケープ解除
    markdown_content = unescape_math(markdown_content)

    lines = [f"# {metadata.title}\n"]
    lines.append(f"Source: {metadata.url}\n")

    if metadata.time_limit:
        lines.append(f"- **Time Limit:** {metadata.time_limit}")
    if metadata.memory_limit:
        lines.append(f"- **Memory Limit:** {metadata.memory_limit}")

    lines.append("\n---\n")
    lines.append(markdown_content)

    return "\n".join(lines)


def run_fetch_problem(url: str) -> None:
    """問題文の取得と表示を実行する。"""
    html = fetch_html(url)
    soup = BeautifulSoup(html, "html.parser")

    # 日本語の本文を最優先で探す
    content_root = soup.find("span", class_="lang-ja")
    if not content_root:
        content_root = soup.find("div", id="task-statement")

    if not content_root:
        print("Warning: Could not find a standard task statement.")
        return

    metadata = extract_metadata(soup, url)
    markdown = format_problem_as_markdown(metadata, content_root)

    print(markdown)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Fetch AtCoder problem statement and format as Markdown."
    )
    parser.add_argument("url", help="URL of the AtCoder problem task")
    args = parser.parse_args()

    run_fetch_problem(args.url)


if __name__ == "__main__":
    main()
