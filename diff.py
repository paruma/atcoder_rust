#!/usr/bin/env python3

import requests
import math
import argparse

def convert_difficulty(raw_difficulty):
    if raw_difficulty >= 400:
        return round(raw_difficulty)
    else:
        return round(400 / math.exp(1.0 - raw_difficulty / 400))

def main(contest_id):
    problems_url = "https://kenkoooo.com/atcoder/resources/problems.json"
    models_url = "https://kenkoooo.com/atcoder/resources/problem-models.json"

    problems_response = requests.get(problems_url)
    models_response = requests.get(models_url)

    problems = problems_response.json()
    models = models_response.json()

    contest_problems = [p for p in problems if p['contest_id'] == contest_id]
    
    if not contest_problems:
        print(f"コンテスト {contest_id} に問題が見つかりませんでした。")
        return

    difficulties = []

    for problem in contest_problems:
        model = models.get(problem['id'])
        raw_difficulty = model['difficulty'] if model else None
        if raw_difficulty is not None:
            displayed_difficulty = convert_difficulty(raw_difficulty)
        else:
            displayed_difficulty = "不明"
        
        difficulties.append({
            'problem': problem['title'],
            'difficulty': displayed_difficulty
        })
    
    for difficulty in difficulties:
        print(f"{difficulty['difficulty']}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="AtCoder Contest Difficulty")
    parser.add_argument("contest_id", help="The contest ID to fetch problems for")
    args = parser.parse_args()
    
    main(args.contest_id)
