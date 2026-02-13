# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
# ]
# ///

import argparse
import re
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup, Tag


@dataclass(frozen=True)
class ProblemMetaData:
    """問題のメタ情報を保持する。"""

    title: str
    url: str
    time_limit: str | None
    memory_limit: str | None


@dataclass(frozen=True)
class Section:
    """問題文の各セクション（問題文、制約、入力、出力、サンプル等）を保持する。"""

    title: str
    content_html: list[Tag]


def fetch_html(url: str) -> str:
    """指定されたURLからHTMLテキストを取得する。"""
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    return response.text


def extract_metadata(soup: BeautifulSoup, url: str) -> ProblemMetaData:
    """HTMLから問題のタイトル、時間制限、メモリ制限などのメタデータを抽出する。"""
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


def extract_sections(soup: BeautifulSoup) -> list[Section]:
    """HTMLから問題文の各セクションを抽出する。"""
    # 英語の本文を優先的に探す
    content_root = soup.find("span", class_="lang-en")
    if not content_root:
        content_root = soup.find("div", id="task-statement")

    if not content_root:
        return []

    sections: list[Section] = []
    for section_tag in content_root.find_all("section"):
        h3 = section_tag.find("h3")
        if not h3:
            continue

        title = h3.get_text().strip()
        # h3以外のコンテンツを抽出
        content_html = [
            child
            for child in section_tag.children
            if isinstance(child, Tag) and child.name != "h3"
        ]
        sections.append(Section(title=title, content_html=content_html))

    return sections


def format_problem_as_markdown(
    metadata: ProblemMetaData, sections: list[Section]
) -> str:
    """問題情報をMarkdown形式に整形する。"""
    lines = [f"# {metadata.title}\n"]
    lines.append(f"Source: {metadata.url}\n")

    if metadata.time_limit:
        lines.append(f"- **Time Limit:** {metadata.time_limit}")
    if metadata.memory_limit:
        lines.append(f"- **Memory Limit:** {metadata.memory_limit}")

    lines.append("\n---\n")

    if not sections:
        lines.append(
            "Warning: Could not find a standard task statement. "
            "This problem might be too old or using an unsupported format."
        )
        return "\n".join(lines)

    for section in sections:
        lines.append(f"## {section.title}\n")
        for tag in section.content_html:
            if tag.name == "pre":
                # サンプル入力・出力などのコードブロック
                lines.append(f"```\n{tag.get_text().strip()}\n```\n")
            else:
                # 段落、リスト等のテキスト
                text = tag.get_text().strip()
                if text:
                    lines.append(text + "\n")

    return "\n".join(lines)


def run_fetch_problem(url: str) -> None:
    """各ステップを組み合わせて問題文を取得・表示する。"""
    html = fetch_html(url)
    soup = BeautifulSoup(html, "html.parser")

    metadata = extract_metadata(soup, url)
    sections = extract_sections(soup)
    markdown = format_problem_as_markdown(metadata, sections)

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
