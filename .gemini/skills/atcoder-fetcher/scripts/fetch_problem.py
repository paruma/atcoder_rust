# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
# ]
# ///

import argparse
import re

import requests
from bs4 import BeautifulSoup


def fetch_atcoder_problem(url):
    try:
        response = requests.get(url)
        response.raise_for_status()
    except requests.RequestException as e:
        print(f"Error fetching URL: {e}")
        return

    soup = BeautifulSoup(response.content, "html.parser")

    # Get Title
    title = soup.find("title").text.strip() if soup.find("title") else "Unknown Problem"
    print(f"# {title}\n")
    print(f"Source: {url}\n")

    # Time and Memory Limits
    # Usually in the first paragraph or a specific div
    # Attempt to find "Time Limit" and "Memory Limit"
    text_content = soup.get_text()
    time_limit_match = re.search(r"Time Limit:\s*(\d+(\.\d+)?\s*sec)", text_content)
    memory_limit_match = re.search(r"Memory Limit:\s*(\d+\s*(?:MB|MiB))", text_content)

    if time_limit_match:
        print(f"- **Time Limit:** {time_limit_match.group(1)}")
    if memory_limit_match:
        print(f"- **Memory Limit:** {memory_limit_match.group(1)}")
    print("\n---\n")

    # Main Task Statement
    # AtCoder problems usually have English and Japanese.
    # We try to find the English section.
    # Structure: <span class="lang-en"> ... </span> OR <div id="task-statement"> ... </div>

    # Priority 1: lang-en
    content_root = soup.find("span", class_="lang-en")

    # Priority 2: task-statement (might contain both langs, but usually separated)
    if not content_root:
        content_root = soup.find("div", id="task-statement")

    if not content_root:
        # Fallback: Just try to find sections
        content_root = soup

    # Extract sections
    # Sections are usually <h3>Title</h3> followed by content.
    # In newer format: <div class="part"><section><h3>...</h3>...</section></div>
    parts = content_root.find_all("section")

    for part in parts:
        h3 = part.find("h3")
        if not h3:
            continue

        section_title = h3.get_text().strip()

        # Skip Japanese sections if we are in a mixed container (rough heuristic)
        # Usually English sections start with "Problem Statement", "Constraints", "Input", "Output", "Sample Input"
        # Japanese: "問題文", "制約", "入力", "出力", "入力例"
        # We allow standard headers.

        allowed_headers = [
            "Problem Statement",
            "Constraints",
            "Input",
            "Output",
            "Sample Input",
            "Sample Output",
            "Note",
            "Explanation",
        ]

        # Check if title starts with "Sample" (e.g. "Sample Input 1")
        is_sample = section_title.startswith("Sample")

        if not (any(h in section_title for h in allowed_headers) or is_sample):
            # check against Japanese to be sure we skip it if we found English?
            # actually, if we found 'lang-en', we are safe.
            # if we didn't, we might be printing Japanese.
            # Let's assume the user wants whatever is there if we couldn't filter by lang.
            pass

        print(f"## {section_title}\n")

        # Get content siblings of h3
        # In standard HTML5 section, h3 is just a child. We want everything else.
        for sibling in part.children:
            if sibling.name == "h3":
                continue
            if sibling.name is None:  # NavigableString
                text = sibling.strip()
                if text:
                    print(text)
            else:
                # If it's a pre tag (sample input/output), format as code block
                if sibling.name == "pre":
                    print(f"```\n{sibling.get_text().strip()}\n```\n")
                else:
                    # For other tags (p, ul, etc.), get text.
                    # Ideally we would preserve basic markdown (lists), but plain text is often OK.
                    # Let's try to preserve some structure.

                    # Convert simple HTML to Markdown-ish
                    text = sibling.get_text()
                    # Basic LaTeX handling: AtCoder uses <var> or \(. \)
                    # BeautifulSoup get_text() keeps the content, which is good.
                    print(text.strip() + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Fetch AtCoder problem statement and format as Markdown."
    )
    parser.add_argument("url", help="URL of the AtCoder problem task")
    args = parser.parse_args()

    fetch_atcoder_problem(args.url)
