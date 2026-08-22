# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "requests",
# ]
# ///

import unittest

from fetch_problems import (
    ContestProblem,
    ProblemDifficulty,
    ProblemInfo,
    ProblemRow,
    build_problem_rows,
)


class BuildProblemRowsTest(unittest.TestCase):
    """Tests for joining problem metadata with contest associations."""

    def test_restores_original_contest_after_adt_reuse(self) -> None:
        """An ADT reuse must not hide the problem from its original ABC."""
        problems = [
            ProblemInfo(id="abc400_a", title="B. ABC400 Party"),
            ProblemInfo(id="abc400_g", title="G. Patisserie ABC 3"),
        ]
        contest_problems = [
            ContestProblem(
                contest_id="abc400",
                problem_id="abc400_a",
                problem_index="A",
            ),
            ContestProblem(
                contest_id="adt_easy_20260806_2",
                problem_id="abc400_a",
                problem_index="B",
            ),
            ContestProblem(
                contest_id="abc400",
                problem_id="abc400_g",
                problem_index="G",
            ),
        ]
        models = {
            "abc400_a": ProblemDifficulty(difficulty=800),
            "abc400_g": ProblemDifficulty(difficulty=None),
        }

        rows = build_problem_rows(problems, contest_problems, models, "ABC4")

        self.assertEqual(
            rows,
            [
                ProblemRow(
                    contest="ABC400",
                    index="A",
                    title="ABC400 Party",
                    difficulty="800",
                    url="https://atcoder.jp/contests/abc400/tasks/abc400_a",
                ),
                ProblemRow(
                    contest="ABC400",
                    index="G",
                    title="Patisserie ABC 3",
                    difficulty="-",
                    url="https://atcoder.jp/contests/abc400/tasks/abc400_g",
                ),
            ],
        )


if __name__ == "__main__":
    unittest.main()
