# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "requests",
# ]
# ///

from __future__ import annotations

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
    title: str


@dataclass(frozen=True)
class ContestProblem:
    """Association between a contest and a problem."""

    contest_id: str
    problem_id: str
    problem_index: str


@dataclass(frozen=True)
class ProblemDifficulty:
    """Difficulty information from AtCoder Problems API."""

    difficulty: float | None


@dataclass(frozen=True)
class ProblemRow:
    """One row written to the output TSV file."""

    contest: str
    index: str
    title: str
    difficulty: str
    url: str


def convert_difficulty(raw_difficulty: float) -> int:
    """Convert an internal difficulty value to the displayed value.

    Args:
        raw_difficulty: Difficulty value returned by AtCoder Problems API.

    Returns:
        Difficulty value displayed by AtCoder Problems.
    """
    if raw_difficulty >= 400:
        return round(raw_difficulty)
    return round(400 / math.exp(1.0 - raw_difficulty / 400))


def fetch_atcoder_data(
    timeout: int = 10,
) -> tuple[list[ProblemInfo], list[ContestProblem], dict[str, ProblemDifficulty]]:
    """Fetch problem metadata, contest associations, and difficulty models.

    Args:
        timeout: Timeout in seconds for each HTTP request.

    Returns:
        Problem metadata, contest-problem associations, and difficulty models.
    """
    problems_url = "https://kenkoooo.com/atcoder/resources/problems.json"
    contest_problems_url = "https://kenkoooo.com/atcoder/resources/contest-problem.json"
    models_url = "https://kenkoooo.com/atcoder/resources/problem-models.json"

    p_res = requests.get(problems_url, timeout=timeout)
    p_res.raise_for_status()
    cp_res = requests.get(contest_problems_url, timeout=timeout)
    cp_res.raise_for_status()
    m_res = requests.get(models_url, timeout=timeout)
    m_res.raise_for_status()

    problems = [ProblemInfo(id=p["id"], title=p["title"]) for p in p_res.json()]

    contest_problems = [
        ContestProblem(
            contest_id=cp["contest_id"],
            problem_id=cp["problem_id"],
            problem_index=cp["problem_index"],
        )
        for cp in cp_res.json()
    ]

    models_raw = m_res.json()
    models = {
        pid: ProblemDifficulty(difficulty=m.get("difficulty"))
        for pid, m in models_raw.items()
    }

    return problems, contest_problems, models


def parse_title(title: str) -> tuple[str, str]:
    """Split a title into its index and clean title.

    Args:
        title: Problem title such as ``C. Drop Blocks``.

    Returns:
        Title index and the title without that index.
    """
    match = re.match(r"^([a-zA-Z0-9]+)\.\s*(.*)$", title)
    if match:
        return match.group(1), match.group(2)
    return "", title


def format_difficulty(problem_id: str, models: dict[str, ProblemDifficulty]) -> str:
    """Format the displayed difficulty of one problem.

    Args:
        problem_id: Problem identifier used as the model key.
        models: Difficulty models keyed by problem identifier.

    Returns:
        Displayed difficulty, or ``-`` when no estimate is available.
    """
    diff_info = models.get(problem_id)
    if diff_info is None or diff_info.difficulty is None:
        return "-"
    return str(convert_difficulty(diff_info.difficulty))


def build_problem_rows(
    problems: list[ProblemInfo],
    contest_problems: list[ContestProblem],
    models: dict[str, ProblemDifficulty],
    prefix: str | None,
) -> list[ProblemRow]:
    """Build TSV rows using contest-problem associations.

    Args:
        problems: Problem metadata returned by AtCoder Problems API.
        contest_problems: Associations between contests and problems.
        models: Difficulty models keyed by problem identifier.
        prefix: Optional case-insensitive contest ID prefix.

    Returns:
        Rows matching the requested contest prefix.
    """
    normalized_prefix = prefix.lower() if prefix else None
    problem_by_id = {problem.id: problem for problem in problems}

    return [
        ProblemRow(
            contest=contest_problem.contest_id.upper(),
            index=contest_problem.problem_index,
            title=parse_title(problem.title)[1],
            difficulty=format_difficulty(contest_problem.problem_id, models),
            url=(
                f"https://atcoder.jp/contests/{contest_problem.contest_id}"
                f"/tasks/{contest_problem.problem_id}"
            ),
        )
        for contest_problem in contest_problems
        if normalized_prefix is None
        or contest_problem.contest_id.lower().startswith(normalized_prefix)
        if (problem := problem_by_id.get(contest_problem.problem_id)) is not None
    ]


def main() -> None:
    """Fetch AtCoder problem data and write the selected contests as TSV."""
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
        problems, contest_problems, models = fetch_atcoder_data()
    except requests.RequestException as error:
        print(f"Error fetching data: {error}", file=sys.stderr)
        sys.exit(1)

    rows = build_problem_rows(problems, contest_problems, models, args.prefix)

    with open(args.output, "w", encoding="utf-8", newline="") as f:
        writer = csv.writer(f, delimiter="\t")
        writer.writerow(["Contest", "Index", "Title", "Difficulty", "URL"])

        for row in rows:
            writer.writerow(
                [row.contest, row.index, row.title, row.difficulty, row.url]
            )

    print(
        f"Successfully saved {len(rows)} problems to {args.output}",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
