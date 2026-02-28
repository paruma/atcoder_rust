# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
# ]
# ///

import argparse
from collections.abc import Generator
from dataclasses import dataclass
from urllib.parse import urljoin, urlparse

import requests
from bs4 import BeautifulSoup, Tag


@dataclass(frozen=True)
class Editorial:
    """解説の情報を保持する。"""

    label: str
    title: str
    url: str
    author: str
    problems: list[str]  # 関連する問題のリスト（例：["A", "B"]）


@dataclass(frozen=True)
class _EditorialMetadata:
    """li要素から抽出された個々の解説のメタデータ。"""

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


def _extract_problem_label(text: str) -> str:
    """h3要素のテキストから問題ラベル（A, B, Overall等）を抽出する。"""
    s = text.strip()
    if " - " in s:
        return s.split(" - ")[0]
    # 日本語・英語両方の「解説」や「Overall」のキーワードに対応
    if any(kw in s for kw in ["解説", "Editorial", "Overall"]):
        return "Overall"
    return s


def _parse_editorial_item(li: Tag, base_url: str) -> _EditorialMetadata | None:
    """li要素から解説のメタデータを抽出する。解説リンクがない場合は None を返す。"""
    link = li.find("a", href=lambda x: x and "/editorial/" in x)
    if not link:
        return None

    href = link.get("href")
    if not href:
        return None

    label_span = li.find("span", class_="label")
    label = label_span.get_text().strip() if label_span else "User"

    # 著者名の抽出
    author = ""
    author_link = li.find("a", class_="username")
    if author_link:
        author = author_link.get_text().strip()
    else:
        # "by author_name" 形式のフォールバック
        full_text = li.get_text()
        if "by" in full_text:
            author = full_text.split("by")[-1].strip()

    return _EditorialMetadata(
        label=label,
        title=link.get_text().strip(),
        url=urljoin(base_url, href),
        author=author,
    )


def _group_by_sections(
    elements: list[Tag],
) -> Generator[tuple[str, list[Tag]], None, None]:
    """h3とliの混在リストを、問題ラベルごとのセクションにグループ化して返す。"""
    current_label = "Overall"
    current_items: list[Tag] = []

    for el in elements:
        if el.name == "h3":
            # 新しい見出しが現れたら、それまでのセクションを出力
            if current_items:
                yield current_label, current_items
            current_label = _extract_problem_label(el.get_text())
            current_items = []
        elif el.name == "li":
            current_items.append(el)

    # 最後のセクションを出力
    if current_items:
        yield current_label, current_items


def extract_editorials(html: str, base_url: str) -> list[Editorial]:
    """HTMLから解説データのリストを抽出する。"""
    soup = BeautifulSoup(html, "html.parser")
    main_container = soup.find("div", id="main-container")
    if not main_container:
        return []

    # 1. HTMLの構造をセクションごとにパースし、フラットな (問題ラベル, メタデータ) のリストにする
    elements = main_container.find_all(["h3", "li"])
    raw_items: list[tuple[str, _EditorialMetadata]] = []

    for problem_label, items in _group_by_sections(elements):
        for li in items:
            meta = _parse_editorial_item(li, base_url)
            if meta:
                raw_items.append((problem_label, meta))

    # 2. URLをキーにして集約する (登場順を維持)
    metadata_map: dict[str, _EditorialMetadata] = {}
    problems_map: dict[str, list[str]] = {}
    urls_in_order: list[str] = []

    for label, meta in raw_items:
        if meta.url not in metadata_map:
            metadata_map[meta.url] = meta
            problems_map[meta.url] = [label]
            urls_in_order.append(meta.url)
        else:
            if label not in problems_map[meta.url]:
                problems_map[meta.url].append(label)

    # 3. 最終的な Editorial オブジェクトのリストを生成
    return [
        Editorial(
            label=metadata_map[url].label,
            title=metadata_map[url].title,
            url=url,
            author=metadata_map[url].author,
            problems=problems_map[url],
        )
        for url in urls_in_order
    ]


def format_editorials_as_markdown(editorials: list[Editorial], base_url: str) -> str:
    """解説リストを表示用のMarkdown形式に整形する。"""
    lines = [f"# Editorials for {base_url}\n"]
    if not editorials:
        lines.append("No editorials found on this page.")
        return "\n".join(lines)

    for data in editorials:
        problems_str = f"[{', '.join(data.problems)}] " if data.problems else ""
        lines.append(f"- {problems_str}[{data.label}] {data.title} (by {data.author})")
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
