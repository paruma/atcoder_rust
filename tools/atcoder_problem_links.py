# /// script
# requires-python = ">=3.13"
# dependencies = [
#   "requests",
#   "beautifulsoup4",
# ]
# ///

import sys
import requests
from bs4 import BeautifulSoup
import argparse
from dataclasses import dataclass
from abc import ABC, abstractmethod

@dataclass(frozen=True)
class Problem:
    """問題データを保持するデータ構造"""
    symbol: str
    title: str
    url: str

# --- 取得・解析 ---

class AtCoderClient:
    """AtCoder からのデータ取得を担当"""
    def fetch_html(self, contest_id: str) -> str:
        url = f"https://atcoder.jp/contests/{contest_id.lower()}/tasks"
        try:
            response = requests.get(url)
            response.raise_for_status()
            return response.text
        except requests.exceptions.RequestException as e:
            print(f"データの取得に失敗しました ({url}): {e}", file=sys.stderr)
            sys.exit(1)

class AtCoderParser:
    """HTML からの問題データの解析を担当"""
    def parse(self, html: str) -> list[Problem]:
        soup = BeautifulSoup(html, "html.parser")
        table = soup.find("table")
        if not table:
            print("問題テーブルが見つかりませんでした。コンテストIDが正しいか確認してください。", file=sys.stderr)
            sys.exit(1)

        tbody = table.find("tbody")
        if not tbody:
            return []

        problems = []
        for row in tbody.find_all("tr"):
            cols = row.find_all("td")
            if len(cols) < 2:
                continue
            
            symbol_link = cols[0].find("a")
            title_link = cols[1].find("a")
            
            if symbol_link and title_link:
                problems.append(Problem(
                    symbol=symbol_link.text.strip(),
                    title=title_link.text.strip(),
                    url="https://atcoder.jp" + title_link["href"]
                ))
        return problems

# --- 出力形式の定義 ---

class Formatter(ABC):
    """出力形式の基底クラス"""
    @abstractmethod
    def format(self, contest_id: str, problems: list[Problem]) -> str:
        pass

class MarkdownFormatter(Formatter):
    """Markdown 形式: [ABC441 A - タイトル](URL)"""
    def format(self, contest_id: str, problems: list[Problem]) -> str:
        upper_id = contest_id.upper()
        return "\n".join([f"[{upper_id} {p.symbol} - {p.title}]({p.url})" for p in problems])

class SimpleFormatter(Formatter):
    """シンプル形式: ABC441 A - タイトル URL"""
    def format(self, contest_id: str, problems: list[Problem]) -> str:
        upper_id = contest_id.upper()
        return "\n".join([f"{upper_id} {p.symbol} - {p.title} {p.url}" for p in problems])

# --- メインロジック ---

def parse_args():
    parser = argparse.ArgumentParser(description="AtCoder の問題リンクを様々な形式で生成します。")
    parser.add_argument("contest_id", help="AtCoder のコンテストID (例: abc441)")
    parser.add_argument(
        "--format", 
        choices=["markdown", "simple"], 
        default="markdown",
        help="出力形式を指定します (デフォルト: markdown)"
    )
    return parser.parse_args()

def main():
    args = parse_args()

    client = AtCoderClient()
    parser = AtCoderParser()
    
    formatters: dict[str, Formatter] = {
        "markdown": MarkdownFormatter(),
        "simple": SimpleFormatter()
    }
    formatter = formatters[args.format]

    html = client.fetch_html(args.contest_id)
    problems = parser.parse(html)
    
    output = formatter.format(args.contest_id, problems)
    if output:
        print(output)

if __name__ == "__main__":
    main()