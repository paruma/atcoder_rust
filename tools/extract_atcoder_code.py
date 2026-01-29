# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "requests",
#     "beautifulsoup4",
# ]
# ///

import argparse
import json
import sys
import re
from dataclasses import asdict, dataclass
from pathlib import Path

import requests
from bs4 import BeautifulSoup

@dataclass(frozen=True)
class Submission:
    """AtCoder 提出データを保持するデータモデル"""
    id: str
    url: str
    problem: str
    user: str
    execution_time: str
    memory: str
    code: str

def parse_submission_html(html: str, url: str) -> Submission:
    """HTML をパースして Submission オブジェクトを生成する"""
    soup = BeautifulSoup(html, "html.parser")
    
    # 正規表現で URL から提出 ID (/submissions/直後の数字) を抽出する
    submission_id = ""
    if match := re.search(r"/submissions/(\d+)", url):
        submission_id = match.group(1)
    
    problem = user = execution_time = memory = code = ""

    # テーブルから各項目を抽出
    for table in soup.find_all("table"):
        for tr in table.find_all("tr"):
            th, td = tr.find("th"), tr.find("td")
            if not th or not td:
                continue
            
            label = th.get_text(strip=True).lower()
            value = td.get_text(strip=True)
            
            match label:
                case "task":
                    problem = value
                case "user":
                    user = value
                case "exec time":
                    execution_time = value
                case "memory":
                    memory = value

    # ソースコードを抽出
    if code_block := soup.find("pre", id="submission-code"):
        code = code_block.get_text()
            
    return Submission(
        id=submission_id,
        url=url,
        problem=problem,
        user=user,
        execution_time=execution_time,
        memory=memory,
        code=code
    )

def download_submission(url: str) -> Submission:
    """指定された URL から提出データをダウンロードする"""
    # 英語のラベルを取得するために lang=en を付加
    target_url = url if "lang=" in url else f"{url}{'&' if '?' in url else '?'}lang=en"

    print(f"Fetching: {target_url}")
    response = requests.get(target_url)
    response.raise_for_status()
    return parse_submission_html(response.text, url)

def save_submission(sub: Submission, out_dir: Path):
    """提出コードとメタデータをファイルに保存する"""
    out_dir.mkdir(exist_ok=True, parents=True)
    
    # ソースコードの保存
    code_path = out_dir / f"submission_{sub.id}.rs"
    code_path.write_text(sub.code, encoding="utf-8")
    
    # メタデータの保存
    meta_path = out_dir / f"submission_{sub.id}_meta.json"
    meta_path.write_text(json.dumps(asdict(sub), indent=2, ensure_ascii=False), encoding="utf-8")
    
    print(f"--- Submission {sub.id} ---")
    print(f"Problem: {sub.problem}")
    print(f"User:    {sub.user}")
    print(f"Time:    {sub.execution_time}")
    print(f"Memory:  {sub.memory}")
    print(f"Saved:   {code_path}")
    print()

def main():
    parser = argparse.ArgumentParser(description="AtCoder 提出情報を取得します。")
    parser.add_argument("urls", nargs="+", help="取得する提出ページの URL")
    parser.add_argument("-o", "--outdir", default="submissions", help="保存先ディレクトリ")
    
    args = parser.parse_args()
    out_dir = Path(args.outdir)
    
    for url in args.urls:
        try:
            sub = download_submission(url)
            save_submission(sub, out_dir)
        except Exception as e:
            print(f"Error processing {url}: {e}", file=sys.stderr)

if __name__ == "__main__":
    main()