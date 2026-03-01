# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
# ]
# ///

import argparse
from collections import defaultdict
from collections.abc import Generator
from dataclasses import dataclass
from urllib.parse import urljoin, urlparse

import requests
from bs4 import BeautifulSoup, Tag


@dataclass(frozen=True)
class Editorial:
    """最終的に出力される解説の情報を保持する。"""

    type_label: str  # 解説の種別（"公式", "User", "English" 等）
    title: str
    url: str
    author: str
    problem_labels: tuple[
        str, ...
    ]  # 関連する問題のリスト。不変性を保つため tuple を使用。


@dataclass(frozen=True)
class _EditorialMetadata:
    """HTMLの各要素から抽出された個々の解説のメタデータ。"""

    type_label: str
    title: str
    url: str
    author: str


def fetch_html(url: str) -> str:
    """指定されたURLからHTMLテキストを取得する。

    Args:
        url: 取得対象のURL。

    Returns:
        取得したHTMLテキスト。
    """
    # 10秒のタイムアウトを設定
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    return response.text


def _extract_problem_label(text: str) -> str | None:
    """h3要素のテキストから問題ラベル（A, B, Overall等）を抽出する。
    問題見出しではないと判断される場合は None を返す。

    Args:
        text: h3要素から取得したテキスト内容。

    Returns:
        抽出された問題ラベル、または None。

    Examples:
        - "A - Seats 2" -> "A"
        - "解説" -> "Overall"
        - "Overall Editorial" -> "Overall"
        - "B" -> "B"
        - "Comments" -> None
    """
    s = text.strip()
    if " - " in s:
        # "A - Problem" 形式
        return s.split(" - ")[0]

    # キーワードによる判定
    if any(kw in s for kw in ["解説", "Editorial", "Overall"]):
        return "Overall"

    # 単一の英文字（A, B, C...）や数字の場合も許容
    if len(s) <= 2:
        return s

    return None


def _parse_editorial_item(li: Tag, base_url: str) -> _EditorialMetadata | None:
    """li要素から解説のメタデータを抽出する。

    Args:
        li: 解析対象の li 要素。
        base_url: HTML内の相対パス（例: /contests/...）を絶対URLに変換するための起点となるURL。

    Returns:
        抽出されたメタデータ。解説リンクがない、または不正な場合は None。

    Examples:
        Input:
            li:
                <span class="label">公式</span>
                <a href="/contests/abcXXX/editorial/123">解説</a>
                by <a class="username" href="/users/user_name">user_name</a>
            base_url: "https://atcoder.jp/contests/abcXXX/editorial"
        Output:
            _EditorialMetadata(
                type_label="公式",
                title="解説",
                url="https://atcoder.jp/contests/abcXXX/editorial/123",
                author="user_name"
            )
    """
    # /editorial/ID という形式のリンクを探す（末尾に数字があるものに限定）
    link = li.find(
        "a",
        href=lambda x: (
            x and "/editorial/" in x and x.rstrip("/").split("/")[-1].isdigit()
        ),
    )
    if not link:
        return None

    href = link.get("href")
    if not href:
        return None
    full_url = urljoin(base_url, href)

    # 種別ラベル（公式/User等）
    label_span = li.find("span", class_="label")
    type_label = label_span.get_text().strip() if label_span else "User"

    # 著者名の抽出
    author = ""
    author_link = li.find("a", class_="username")
    if author_link:
        author = author_link.get_text().strip()
    else:
        # リンクがない場合のみテキストから抽出を試みる
        text_content = li.get_text()
        if "by" in text_content:
            parts = text_content.split("by")
            potential_author = parts[-1].strip()
            # 著者名として妥当な長さ（1〜50文字）の場合のみ採用
            if 0 < len(potential_author) < 50:
                author = potential_author

    return _EditorialMetadata(
        type_label=type_label,
        title=link.get_text().strip(),
        url=full_url,
        author=author,
    )


def _group_by_sections(
    elements: list[Tag],
) -> Generator[tuple[str, list[Tag]], None, None]:
    """問題見出し(h3)とその下に続く解説リスト(li)を、問題ラベルごとにグループ化する。

    Args:
        elements: BeautifulSoup で抽出された h3（問題見出し）または li（解説リンク）要素のリスト。

    Yields:
        (問題ラベル, その問題に関連する li 要素のリスト) のタプル。
    """
    current_problem_label: str | None = "Overall"
    current_items: list[Tag] = []

    for el in elements:
        if el.name == "h3":
            # 新しい見出しが現れたら、それまでのセクションを出力
            if current_items and current_problem_label:
                yield current_problem_label, current_items

            # 問題ラベルとして認識できる場合のみ、次のグループを開始する
            current_problem_label = _extract_problem_label(el.get_text())
            current_items = []
        elif el.name == "li":
            current_items.append(el)

    # 最後のセクションを出力
    if current_items and current_problem_label:
        yield current_problem_label, current_items


def extract_editorials(html: str, base_url: str) -> list[Editorial]:
    """HTMLから解説データのリストを抽出する。

    Args:
        html: AtCoderの解説一覧ページのHTMLテキスト。
        base_url: HTML内の相対パス（例: /contests/...）を絶対URLに変換するための起点となるURL。

    Returns:
        抽出・集約された Editorial オブジェクトのリスト。
    """
    soup = BeautifulSoup(html, "html.parser")
    main_container = soup.find("div", id="main-container")
    if not main_container:
        return []

    # URL をキーにして、(メタデータ, 問題ラベルのリスト) を管理する
    url_to_meta: dict[str, _EditorialMetadata] = {}
    url_to_problems: dict[str, list[str]] = defaultdict(list)
    ordered_urls: list[str] = []

    elements = main_container.find_all(["h3", "li"])
    for problem_label, items in _group_by_sections(elements):
        for li in items:
            meta = _parse_editorial_item(li, base_url)
            if not meta:
                continue

            url = meta.url
            if url not in url_to_meta:
                url_to_meta[url] = meta
                ordered_urls.append(url)

            if problem_label not in url_to_problems[url]:
                url_to_problems[url].append(problem_label)

    return [
        Editorial(
            type_label=url_to_meta[url].type_label,
            title=url_to_meta[url].title,
            url=url,
            author=url_to_meta[url].author,
            problem_labels=tuple(url_to_problems[url]),
        )
        for url in ordered_urls
    ]


def format_editorials_as_markdown(editorials: list[Editorial], base_url: str) -> str:
    """解説リストを表示用のMarkdown形式に整形する。

    Args:
        editorials: 整形対象の解説リスト。
        base_url: 対象ページのURL（ヘッダー表示用）。

    Returns:
        Markdown形式のテキスト。
    """
    lines = [f"# Editorials for {base_url}\n"]
    if not editorials:
        lines.append("No editorials found on this page.")
        return "\n".join(lines)

    for data in editorials:
        problems_str = (
            f"[{', '.join(data.problem_labels)}] " if data.problem_labels else ""
        )
        author_str = f" (by {data.author})" if data.author else ""
        lines.append(f"- {problems_str}[{data.type_label}] {data.title}{author_str}")
        lines.append(f"  URL: {data.url}")

    return "\n".join(lines)


def run_list_editorials(url: str) -> None:
    """各ステップを組み合わせて解説リストを表示する。

    Args:
        url: AtCoderの解説一覧ページのURL。
    """
    parsed_url = urlparse(url)
    base_url = f"{parsed_url.scheme}://{parsed_url.netloc}{parsed_url.path}"
    # 日本語ページを明示的に指定
    target_url = base_url + "?lang=ja"

    html = fetch_html(target_url)
    editorials = extract_editorials(html, base_url)
    output = format_editorials_as_markdown(editorials, base_url)

    print(output)


def main() -> None:
    """CLIエントリポイント。"""
    parser = argparse.ArgumentParser(
        description="List editorials for an AtCoder problem."
    )
    parser.add_argument("url", help="URL of the problem's editorial list page")
    args = parser.parse_args()

    run_list_editorials(args.url)


if __name__ == "__main__":
    main()
