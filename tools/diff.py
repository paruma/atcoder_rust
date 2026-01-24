# /// script
# requires-python = ">=3.13"
# dependencies = [
#   "requests",
# ]
# ///

import sys
import math
import requests
import argparse
from dataclasses import dataclass

# --- 0. モデル定義 ---

@dataclass(frozen=True)
class Problem:
    """AtCoder Problems から取得した問題の基本情報"""
    id: str
    contest_id: str
    title: str

@dataclass(frozen=True)
class ProblemDifficulty:
    """AtCoder Problems から取得した難易度情報"""
    difficulty: int | None

# --- 1. 計算レイヤー ---

def convert_difficulty(raw_difficulty: float) -> int:
    """内部的な難易度数値を、サイト上の表示数値（正の数）に変換する"""
    if raw_difficulty >= 400:
        return round(raw_difficulty)
    return round(400 / math.exp(1.0 - raw_difficulty / 400))

# --- 2. 取得レイヤー (Network) ---

def fetch_atcoder_problems_data() -> tuple[list[Problem], dict[str, ProblemDifficulty]]:
    """AtCoder Problems API から全問題と全難易度モデルを構造化して取得する"""
    problems_url = "https://kenkoooo.com/atcoder/resources/problems.json"
    models_url = "https://kenkoooo.com/atcoder/resources/problem-models.json"

    try:
        p_res = requests.get(problems_url)
        p_res.raise_for_status()
        m_res = requests.get(models_url)
        m_res.raise_for_status()
        
        # 必要なデータのみをモデルに詰め替える
        problems = [
            Problem(id=p["id"], contest_id=p["contest_id"], title=p["title"])
            for p in p_res.json()
        ]
        
        models_raw = m_res.json()
        models = {
            pid: ProblemDifficulty(difficulty=m.get("difficulty"))
            for pid, m in models_raw.items()
        }
        
        return problems, models
        
    except requests.exceptions.RequestException as e:
        print(f"データの取得に失敗しました: {e}", file=sys.stderr)
        sys.exit(1)

# --- 3. 加工レイヤー (Logic) ---

def extract_contest_difficulties(
    contest_id: str, 
    problems: list[Problem], 
    models: dict[str, ProblemDifficulty]
) -> list[int | str]:
    """指定コンテストの問題を抽出し、各問題の難易度をリストで返す"""
    
    contest_problems = [
        p for p in problems 
        if p.contest_id == contest_id.lower()
    ]
    
    if not contest_problems:
        return []

    results: list[int | str] = []
    for p in contest_problems:
        diff_info = models.get(p.id)
        if diff_info and diff_info.difficulty is not None:
            results.append(convert_difficulty(diff_info.difficulty))
        else:
            results.append("不明")
    
    return results

# --- 4. 実行制御レイヤー (App) ---

def parse_args():
    """コマンドライン引数の解析"""
    parser = argparse.ArgumentParser(
        description="AtCoder の指定のコンテストの各問題の推定難易度（Difficulty）を取得します。",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
使い方例:
  python tools/diff.py abc390
"""
    )
    parser.add_argument("contest_id", help="コンテストID (例: abc390)")
    return parser.parse_args()

def main():
    args = parse_args()
    
    # データの取得
    problems, models = fetch_atcoder_problems_data()
    
    # データの加工
    difficulties = extract_contest_difficulties(args.contest_id, problems, models)
    
    # 出力
    if not difficulties:
        print(f"コンテスト {args.contest_id} に問題が見つかりませんでした。", file=sys.stderr)
        sys.exit(1)

    for diff in difficulties:
        print(diff)

if __name__ == "__main__":
    main()
