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
HOME=/home/node UV_CACHE_DIR=/home/node/.cache/uv PATH="/home/node/.local/bin:$PATH" uv run .gemini/skills/atcoder-fetcher/scripts/fetch_problem.py <URL>
```

### Fetching an Editorial

```bash
HOME=/home/node UV_CACHE_DIR=/home/node/.cache/uv PATH="/home/node/.local/bin:$PATH" uv run .gemini/skills/atcoder-fetcher/scripts/fetch_editorial.py <URL>
```

### Listing Editorials for a Problem

```bash
HOME=/home/node UV_CACHE_DIR=/home/node/.cache/uv PATH="/home/node/.local/bin:$PATH" uv run .gemini/skills/atcoder-fetcher/scripts/list_editorials.py <URL>
```

## URL Construction Rules

If the user provides a contest name and problem letter instead of a full URL, construct the URL using these rules:

### 1. Problem Statement URL
- **Pattern:** `https://atcoder.jp/contests/<contest_id>/tasks/<contest_id>_<problem_letter>`
- **Example:** "ABC432 D" -> `https://atcoder.jp/contests/abc432/tasks/abc432_d`

### 2. Editorial List URL (Problem-specific)
- **Pattern:** `https://atcoder.jp/contests/<contest_id>/tasks/<contest_id>_<problem_letter>/editorial`
- **Example:** "ABC432 D Editorials" -> `https://atcoder.jp/contests/abc432/tasks/abc432_d/editorial`

### 3. Individual Editorial URL
- **Pattern:** `https://atcoder.jp/contests/<contest_id>/editorial/<editorial_id>`
- **Example:** `https://atcoder.jp/contests/abc432/editorial/12345`

### Rules for Identifiers
- `<contest_id>`: Lowercase contest name (e.g., `abc432`, `arc150`, `agc001`).
- `<problem_letter>`: Lowercase problem label (e.g., `a`, `b`, `c`, `d`).
- `<editorial_id>`: A unique numeric identifier for a specific editorial (found in the editorial list).

## Environment Variables

The following variables are required in the sandbox:
- `HOME=/home/node`
- `UV_CACHE_DIR=/home/node/.cache/uv`
- `PATH="/home/node/.local/bin:$PATH"`