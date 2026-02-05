---
name: atcoder-fetcher
description: Fetch and format AtCoder problem statements or editorials from a URL. Use this skill when the user provides an AtCoder problem URL (e.g., .../tasks/abcXXX_x), an editorial URL (e.g., .../editorial/XXXX), or refers to a problem by name (e.g., "ABC432 D").
---

# AtCoder Fetcher

This skill retrieves problem statements and editorials from AtCoder URLs and formats them as Markdown.

## Usage

Execute the corresponding Python script using `uv run`.
**Important:** You must set the correct environment variables for the sandbox environment.

### Fetching a Problem Statement

```bash
export HOME=/home/node && export UV_CACHE_DIR=/home/node/.cache/uv && export PATH="/home/node/.local/bin:$PATH" && uv run .gemini/skills/atcoder-fetcher/scripts/fetch_problem.py <URL>
```

### Fetching an Editorial

```bash
export HOME=/home/node && export UV_CACHE_DIR=/home/node/.cache/uv && export PATH="/home/node/.local/bin:$PATH" && uv run .gemini/skills/atcoder-fetcher/scripts/fetch_editorial.py <URL>
```

## URL Construction Rules

If the user provides a contest name and problem letter instead of a full URL, construct the URL using these rules:

- **Pattern:** `https://atcoder.jp/contests/<contest_id>/tasks/<contest_id>_<problem_letter>`
- **Rules:**
    - `<contest_id>`: Lowercase contest name (e.g., `abc432`, `arc150`, `agc001`).
    - `<problem_letter>`: Lowercase problem label (e.g., `a`, `b`, `c`, `d`).
- **Examples:**
    - "ABC432 D" -> `https://atcoder.jp/contests/abc432/tasks/abc432_d`
    - "ARC100 A" -> `https://atcoder.jp/contests/arc100/tasks/arc100_a`

## Environment Variables

The following variables are required in the sandbox:
- `HOME=/home/node`
- `UV_CACHE_DIR=/home/node/.cache/uv`
- `PATH="/home/node/.local/bin:$PATH"`