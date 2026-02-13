# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
# ]
# ///

import argparse
from dataclasses import dataclass
from urllib.parse import urljoin, urlparse

import requests
from bs4 import BeautifulSoup


@dataclass(frozen=True)
class Editorial:
    """解説の情報を保持する。"""

    label: str
    title: str
    url: str
    author: str


def fetch_html(url: str) -> str:
    """指定されたURLからHTMLテキストを取得する。"""
    # 10秒のタイムアウトを設定
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    return response.text


def extract_editorials(html: str, base_url: str) -> list[Editorial]:
    """HTMLから解説データのリストを抽出する。"""
    soup = BeautifulSoup(html, "html.parser")
    main_container = soup.find("div", id="main-container")
    if not main_container:
        return []

    editorials: list[Editorial] = []
    for li in main_container.find_all("li"):
        links = li.find_all("a", href=lambda x: x and "/editorial/" in x)
        if not links:
            continue

        label_span = li.find("span", class_="label")
        label = label_span.get_text().strip() if label_span else "User"

        editorial_link = links[0]
        title = editorial_link.get_text().strip()
        href = editorial_link.get("href")
        if not href:
            continue
        full_url = urljoin(base_url, href)

        author = ""
        author_link = li.find("a", class_="username")
        if author_link:
            author = author_link.get_text().strip()
        else:
            text = li.get_text()
            if "by" in text:
                author = text.split("by")[-1].strip()

        editorials.append(
            Editorial(label=label, title=title, url=full_url, author=author)
        )
    return editorials


def format_editorials_as_markdown(editorials: list[Editorial], base_url: str) -> str:
    """解説リストを表示用のMarkdown形式に整形する。"""
    lines = [f"# Editorials for {base_url}\n"]
    if not editorials:
        lines.append("No editorials found on this page.")
        return "\n".join(lines)

    for data in editorials:
        lines.append(f"- [{data.label}] {data.title} (by {data.author})")
        lines.append(f"  URL: {data.url}")

    return "\n".join(lines)


def run_list_editorials(url: str) -> None:
    """各ステップを組み合わせて解説リストを表示する。"""
    parsed_url = urlparse(url)
    base_url = f"{parsed_url.scheme}://{parsed_url.netloc}{parsed_url.path}"
    # 日本語ページを明示的に指定して取得
    target_url = base_url + "?lang=ja"

    html = fetch_html(target_url)
    editorials = extract_editorials(html, base_url)
    output = format_editorials_as_markdown(editorials, base_url)

    print(output)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="List editorials for an AtCoder problem."
    )
    parser.add_argument("url", help="URL of the problem's editorial list page")
    args = parser.parse_args()

    run_list_editorials(args.url)


if __name__ == "__main__":
    main()
