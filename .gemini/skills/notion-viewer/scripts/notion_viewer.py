# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "httpx",
#   "python-dotenv",
# ]
# ///

import argparse
import asyncio
import os
import sys
from abc import ABC, abstractmethod
from typing import Any

import httpx
from dotenv import load_dotenv

# .env からトークンを読み込み
load_dotenv()
NOTION_TOKEN = os.getenv("NOTION_TOKEN")
NOTION_VERSION = "2022-06-28"

if not NOTION_TOKEN:
    print(
        "Error: NOTION_TOKEN not found in .env. Please set it to your Notion Integration Token.",
        file=sys.stderr,
    )
    sys.exit(1)


# --- Block Handlers ---


class BlockHandler(ABC):
    @abstractmethod
    def can_handle(self, block_type: str) -> bool:
        pass

    @abstractmethod
    def render(self, block: dict[str, Any], indent_level: int) -> str:
        pass

    def rich_text_to_markdown(self, rich_text_list: list[dict[str, Any]]) -> str:
        result = []
        for text in rich_text_list:
            content = text.get("plain_text", "")
            annotations = text.get("annotations", {})
            href = text.get("href")

            if annotations.get("code"):
                content = f"`{content}`"
            if annotations.get("bold"):
                content = f"**{content}**"
            if annotations.get("italic"):
                content = f"*{content}*"
            if annotations.get("strikethrough"):
                content = f"~~{content}~~"

            if href:
                content = f"[{content}]({href})"

            result.append(content)
        return "".join(result)


class SimpleTextBlockHandler(BlockHandler):
    def __init__(self, block_type: str, prefix: str = "", suffix: str = ""):
        self.target_type = block_type
        self.prefix = prefix
        self.suffix = suffix

    def can_handle(self, block_type: str) -> bool:
        return block_type == self.target_type

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        content = self.rich_text_to_markdown(block[self.target_type]["rich_text"])
        return f"{indent}{self.prefix}{content}{self.suffix}"


class HeadingHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type.startswith("heading_")

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        level = int(block["type"].split("_")[1])
        indent = "  " * indent_level
        content = self.rich_text_to_markdown(block[block["type"]]["rich_text"])
        return f"\n{indent}{'#' * level} {content}"


class CodeHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "code"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        code_block = block["code"]
        lang = code_block.get("language", "text")
        content = self.rich_text_to_markdown(code_block["rich_text"])
        indented_content = "\n".join(f"{indent}{line}" for line in content.splitlines())
        return f"\n{indent}```{lang}\n{indented_content}\n{indent}```"


class ToDoHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "to_do"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        to_do = block["to_do"]
        status = "[x]" if to_do["checked"] else "[ ]"
        content = self.rich_text_to_markdown(to_do["rich_text"])
        return f"{indent}{status} {content}"


class EquationHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "equation"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        expr = block["equation"].get("expression", "")
        return f"{indent}$${expr}$$"


class DividerHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "divider"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        return f"{indent}---"


class CalloutHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "callout"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        callout = block["callout"]
        icon = ""
        if callout.get("icon"):
            if callout["icon"]["type"] == "emoji":
                icon = callout["icon"]["emoji"] + " "

        content = self.rich_text_to_markdown(callout["rich_text"])
        return f"{indent}> {icon}{content}"


class TableRowHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "table_row"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        cells = block["table_row"]["cells"]
        row_content = " | ".join([self.rich_text_to_markdown(cell) for cell in cells])
        return f"{indent}| {row_content} |"


class ToggleHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "toggle"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        content = self.rich_text_to_markdown(block["toggle"]["rich_text"])
        return f"{indent}▶ {content}"


class ChildPageHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return block_type == "child_page"

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        indent = "  " * indent_level
        title = block["child_page"].get("title", "Untitled")
        block_id = block["id"]
        # Simplified URL for easier navigation
        return f"{indent}[Page: {title}](https://www.notion.so/{block_id.replace('-', '')})"


class DefaultHandler(BlockHandler):
    def can_handle(self, block_type: str) -> bool:
        return True

    def render(self, block: dict[str, Any], indent_level: int) -> str:
        return ""


# --- Main Components ---


class NotionClient:
    def __init__(self, token: str, version: str):
        self.client = httpx.AsyncClient(
            base_url="https://api.notion.com/v1",
            headers={
                "Authorization": f"Bearer {token}",
                "Notion-Version": version,
                "Content-Type": "application/json",
            },
            timeout=30.0,
        )
        self.semaphore = asyncio.Semaphore(10)

    async def close(self):
        await self.client.aclose()

    async def get_block_children(self, block_id: str) -> list[dict[str, Any]]:
        async with self.semaphore:
            results = []
            cursor = None
            while True:
                params = {"page_size": 100}
                if cursor:
                    params["start_cursor"] = cursor

                # リトライ制限
                for attempt in range(3):
                    try:
                        response = await self.client.get(
                            f"/blocks/{block_id}/children", params=params
                        )
                        if response.status_code == 429:
                            if attempt < 2:
                                await asyncio.sleep(2 * (attempt + 1))
                                continue
                            else:
                                print(
                                    f"Error: Rate limit exceeded after 3 attempts for block {block_id}",
                                    file=sys.stderr,
                                )
                                break
                        if response.status_code == 403:
                            print(
                                f"Error: Permission denied for block {block_id}. Ensure the page is shared with your integration.",
                                file=sys.stderr,
                            )
                            return results
                        response.raise_for_status()
                        data = response.json()
                        results.extend(data.get("results", []))
                        if not data.get("has_more"):
                            return results
                        cursor = data.get("next_cursor")
                        break  # 成功したのでリトライループを抜ける
                    except Exception as e:
                        if attempt < 2:
                            await asyncio.sleep(1)
                            continue
                        print(f"Error fetching block {block_id}: {e}", file=sys.stderr)
                        return results
                else:
                    break  # リトライ失敗
            return results

    async def fetch_full_tree(self, block_id: str) -> list[dict[str, Any]]:
        children = await self.get_block_children(block_id)

        async def process_block(block):
            if block.get("has_children") and block["type"] != "child_page":
                block["children_data"] = await self.fetch_full_tree(block["id"])
            return block

        return await asyncio.gather(*(process_block(b) for b in children))

    async def search(self, query: str) -> list[dict[str, Any]]:
        payload = {
            "query": query,
            "filter": {"value": "page", "property": "object"},
            "sort": {"direction": "descending", "timestamp": "last_edited_time"},
        }
        for attempt in range(3):
            try:
                response = await self.client.post("/search", json=payload)
                if response.status_code == 429:
                    if attempt < 2:
                        await asyncio.sleep(2 * (attempt + 1))
                        continue
                    else:
                        print(
                            "Error: Rate limit exceeded after 3 attempts during search.",
                            file=sys.stderr,
                        )
                        return []
                response.raise_for_status()
                return response.json().get("results", [])
            except Exception as e:
                if attempt < 2:
                    await asyncio.sleep(1)
                    continue
                print(f"Error searching: {e}", file=sys.stderr)
                return []
        return []


class MarkdownRenderer:
    def __init__(self):
        self.handlers: list[BlockHandler] = [
            HeadingHandler(),
            SimpleTextBlockHandler("paragraph"),
            SimpleTextBlockHandler("bulleted_list_item", prefix="- "),
            SimpleTextBlockHandler("numbered_list_item", prefix="1. "),
            SimpleTextBlockHandler("quote", prefix="> "),
            ToDoHandler(),
            CodeHandler(),
            EquationHandler(),
            DividerHandler(),
            CalloutHandler(),
            TableRowHandler(),
            ToggleHandler(),
            ChildPageHandler(),
            DefaultHandler(),
        ]

    def get_handler(self, block_type: str) -> BlockHandler:
        for handler in self.handlers:
            if handler.can_handle(block_type):
                return handler
        return self.handlers[-1]

    def render_tree(self, blocks: list[dict[str, Any]], indent_level: int = 0):
        for block in blocks:
            handler = self.get_handler(block["type"])
            try:
                output = handler.render(block, indent_level)
                if output:
                    print(output)
            except Exception as e:
                print(f"Error rendering block {block['id']}: {e}", file=sys.stderr)

            if "children_data" in block:
                self.render_tree(block["children_data"], indent_level + 1)


async def view_command(client: NotionClient, block_id: str):
    tree = await client.fetch_full_tree(block_id)
    if not tree:
        print(
            f"No content found for ID {block_id}. Check if ID is correct and integration has access.",
            file=sys.stderr,
        )
        return
    renderer = MarkdownRenderer()
    renderer.render_tree(tree)


async def search_command(client: NotionClient, query: str):
    results = await client.search(query)
    if not results:
        print(f"No pages found for query: '{query}'")
        return

    print(f"{'Page ID':<36} | {'Title'}")
    print("-" * 50)

    def extract_title(res: dict[str, Any]) -> str:
        props = res.get("properties", {})
        title_entry = next(
            (
                v
                for v in props.values()
                if isinstance(v, dict) and v.get("type") == "title"
            ),
            {},
        )
        title_list = title_entry.get("title", [])
        return "".join([t.get("plain_text", "") for t in title_list]) or "Untitled"

    for result in results:
        print(f"{result['id']:<36} | {extract_title(result)}")


async def main():
    parser = argparse.ArgumentParser(
        description="Notion Viewer CLI (Parallel Optimized)"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    parser_search = subparsers.add_parser("search", help="Search for pages")
    parser_search.add_argument("query", nargs="?", default="", help="Search query")
    parser_view = subparsers.add_parser("view", help="View page content")
    parser_view.add_argument("block_id", help="Page or Block ID to view")
    args = parser.parse_args()

    client = NotionClient(NOTION_TOKEN, NOTION_VERSION)
    try:
        if args.command == "search":
            await search_command(client, args.query)
        elif args.command == "view":
            await view_command(client, args.block_id)
    finally:
        await client.close()


if __name__ == "__main__":
    asyncio.run(main())
