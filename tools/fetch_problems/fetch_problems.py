# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "requests",
# ]
# ///

import argparse
import csv
import math
import re
import sys
from dataclasses import dataclass
from pathlib import Path

import requests


@dataclass(frozen=True)
class ProblemInfo:
    """Basic problem information from AtCoder Problems API."""

    id: str
    contest_id: str
    title: str


@dataclass(frozen=True)
class ProblemDifficulty:
    """Difficulty information from AtCoder Problems API."""

    difficulty: float | None


def convert_difficulty(raw_difficulty: float) -> int:
    """Converts internal difficulty value to the display value (positive number)."""
    if raw_difficulty >= 400:
        return round(raw_difficulty)
    return round(400 / math.exp(1.0 - raw_difficulty / 400))


def fetch_atcoder_data(timeout: int = 10) -> tuple[list[ProblemInfo], dict[str, ProblemDifficulty]]:
    """
    Fetches problems and models data from AtCoder Problems API.
    """
    problems_url = "https://kenkoooo.com/atcoder/resources/problems.json"
    models_url = "https://kenkoooo.com/atcoder/resources/problem-models.json"

    p_res = requests.get(problems_url, timeout=timeout)
    p_res.raise_for_status()
    m_res = requests.get(models_url, timeout=timeout)
    m_res.raise_for_status()

    problems = [
        ProblemInfo(id=p["id"], contest_id=p["contest_id"], title=p["title"])
        for p in p_res.json()
    ]

    models_raw = m_res.json()
    models = {
        pid: ProblemDifficulty(difficulty=m.get("difficulty"))
        for pid, m in models_raw.items()
    }

    return problems, models


def parse_title(title: str) -> tuple[str, str]:
    """
    Splits the title into index and clean title.
    Example: 'C. Drop Blocks' -> ('C', 'Drop Blocks')
    """
    match = re.match(r"^([a-zA-Z0-9]+)\.\s*(.*)$", title)
    if match:
        return match.group(1), match.group(2)
    return "", title


def get_problem_index(problem_id: str, contest_id: str, title_index: str) -> str:
    """
    Determines the best problem index.
    Prioritizes title_index if available, otherwise uses problem_id suffix.
    """
    if title_index:
        return title_index.upper()

    if problem_id.startswith(contest_id + "_"):
        return problem_id[len(contest_id) + 1 :].upper()
    return problem_id.upper()


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Fetch all AtCoder problems and output as TSV including difficulty."
    )
    parser.add_argument(
        "--output",
        "-o",
        type=Path,
        default=Path("atcoder_problems.tsv"),
        help="Path to the output TSV file (default: atcoder_problems.tsv)",
    )
    parser.add_argument(
        "--prefix",
        "-p",
        type=str,
        help="Filter contests by prefix (case-insensitive, e.g., 'abc4')",
    )

    args = parser.parse_args()

    print("Fetching data from AtCoder Problems API...", file=sys.stderr)
    try:
        problems, models = fetch_atcoder_data()
    except Exception as e:
        print(f"Error fetching data: {e}", file=sys.stderr)
        sys.exit(1)

    prefix = args.prefix.lower() if args.prefix else None

    count = 0
    with open(args.output, "w", encoding="utf-8", newline="") as f:
        writer = csv.writer(f, delimiter="\t")
        writer.writerow(["Contest", "Index", "Title", "Difficulty", "URL"])

        for p in problems:
            if prefix and not p.contest_id.lower().startswith(prefix):
                continue

            diff_info = models.get(p.id)
            if diff_info and diff_info.difficulty is not None:
                difficulty = str(convert_difficulty(diff_info.difficulty))
            else:
                difficulty = "-"

            contest_display = p.contest_id.upper()
            title_index, clean_title = parse_title(p.title)
            index = get_problem_index(p.id, p.contest_id, title_index)
            url = f"https://atcoder.jp/contests/{p.contest_id}/tasks/{p.id}"

            writer.writerow([contest_display, index, clean_title, difficulty, url])
            count += 1

    print(f"Successfully saved {count} problems to {args.output}", file=sys.stderr)


if __name__ == "__main__":
    main()
