---
name: notion-viewer
description: Notion ページの内容を検索・閲覧し、人間が読みやすい形式（Markdown）で表示するためのスキル。Notion API の冗長な JSON レスポンスを整形して表示したい場合に使用する。
---

# Notion Viewer

This skill provides utilities for searching and viewing Notion pages with clean, human-readable formatting. It abstracts away the complexity of raw JSON-RPC calls to the Notion MCP server.

## When to Use

- When the user asks to "read", "view", "show", or "list" Notion pages.
- When the raw JSON output from `notion:API-retrieve-a-page` or `notion:API-post-search` is too verbose or difficult to read.
- To quickly search for a page ID by title.

## Usage

### 1. Search for a Page

To find a page's ID by its title:

```bash
HOME=/home/node UV_CACHE_DIR=/home/node/.cache/uv PATH="/home/node/.local/bin:$PATH" uv run .gemini/skills/notion-viewer/scripts/notion_viewer.py search "Query String"
```

### 2. View Page Content

To display the content of a page (blocks) in Markdown format:

```bash
HOME=/home/node UV_CACHE_DIR=/home/node/.cache/uv PATH="/home/node/.local/bin:$PATH" uv run .gemini/skills/notion-viewer/scripts/notion_viewer.py view "Page-ID-or-Block-ID"
```

## Requirements

- **Environment Variables**: 
  - `NOTION_TOKEN` must be set.
  - `HOME=/home/node`, `UV_CACHE_DIR=/home/node/.cache/uv`, and `PATH` update are required for `uv run` in the sandbox.
- **Runtime**: Python 3.13+ and `uv`.

## Troubleshooting

- **401 Unauthorized**: Check if `NOTION_TOKEN` in `.env` is correct.
- **Empty Output**: Ensure the page ID is correct and the bot user has access to that page (via "Add connections" in Notion).