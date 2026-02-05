# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "beautifulsoup4",
#     "requests",
# ]
# ///

import argparse
import requests
from bs4 import BeautifulSoup
import re


def fetch_atcoder_editorial(url):
    try:
        response = requests.get(url)
        response.raise_for_status()
    except requests.RequestException as e:
        print(f"Error fetching URL: {e}")
        return

    soup = BeautifulSoup(response.content, "html.parser")

    # Find the main container for the editorial
    # Based on the HTML structure, it seems to be in a div with class "col-sm-12" inside "main-container"
    # Or specifically, looking for the h2 that contains "Editorial"

    main_container = soup.find("div", id="main-container")
    if not main_container:
        print("Could not find main container.")
        return

    content_div = None
    h2_title = None

    # Try to find the specific h2
    for h2 in main_container.find_all("h2"):
        if "Editorial" in h2.get_text():
            h2_title = h2
            # The content is usually in the parent of this h2, or siblings following it.
            # In the provided HTML, h2 is inside a div.col-sm-12.
            content_div = h2.find_parent("div", class_="col-sm-12")
            break

    if not content_div:
        # Fallback: just look for the col-sm-12 that is likely to contain content
        # It is usually the one after the nav tabs
        divs = main_container.find_all("div", class_="col-sm-12")
        for d in divs:
            if d.find("h2"):
                content_div = d
                break

    if not content_div:
        print("Could not find editorial content.")
        return

    # Extract Title
    title = h2_title.get_text().strip() if h2_title else "Editorial"
    print(f"# {title}\n")
    print(f"Source: {url}\n")
    print("---\n")

    # Process content elements
    # We want to iterate through children of content_div and format them.
    # Start after the h2 if found.

    start_processing = False
    if h2_title:
        # If h2 is a direct child
        if h2_title in content_div.children:
            start_processing = False  # Will flip to True after hitting h2
        else:
            # If h2 is nested deeper, we might just process everything in content_div
            start_processing = True
    else:
        start_processing = True

    for element in content_div.children:
        if h2_title and element == h2_title:
            start_processing = True
            continue

        if not start_processing:
            continue

        if element.name is None:
            text = element.strip()
            if text:
                print(text)
            continue

        # Ignore script, style, etc.
        if element.name in ["script", "style", "noscript"]:
            continue

        # HR
        if element.name == "hr":
            print("\n---\n")
            continue

        # Headers
        if element.name in ["h1", "h2", "h3", "h4", "h5", "h6"]:
            level = int(element.name[1])
            # Adjust level because main title is h1
            print(f"{'#' * (level + 1)} {element.get_text().strip()}\n")
            continue

        # Code blocks (pre)
        if element.name == "pre":
            # Check for language class
            code = element.get_text().strip()
            # Try to guess language or find class like "language-cpp" or "prettyprint"
            lang = ""
            if element.get("class"):
                for cls in element.get("class"):
                    if cls.startswith("language-"):
                        lang = cls.replace("language-", "")
                    elif cls == "prettyprint":
                        # Default to cpp or python if ambiguous, but usually cpp in competitive programming
                        # Let's verify if there is a code tag inside with class
                        pass

            # Often <pre class="prettyprint"><code>...</code></pre>
            code_tag = element.find("code")
            if code_tag:
                if code_tag.get("class"):
                    for cls in code_tag.get("class"):
                        if cls.startswith("language-"):
                            lang = cls.replace("language-", "")

            print(f"```{lang}\n{code}\n```\n")
            continue

        # Blockquotes
        if element.name == "blockquote":
            # Simple handling of blockquote
            text = element.get_text().strip()
            # Prefix each line with >
            quoted = "\n".join([f"> {line}" for line in text.splitlines()])
            print(f"{quoted}\n")
            continue

        # Lists (ul, ol)
        if element.name == "ul":
            for li in element.find_all("li", recursive=False):
                print(f"- {li.get_text().strip()}")
            print()  # Newline after list
            continue

        if element.name == "ol":
            for i, li in enumerate(element.find_all("li", recursive=False), 1):
                print(f"{i}. {li.get_text().strip()}")
            print()
            continue

        # Paragraphs and divs
        # Recurse or just get text?
        # Getting text might lose inline formatting like bold or links,
        # but is simpler. For an LLM, plain text with math is usually fine.
        # However, we want to preserve MathJax/KaTeX delimiters.
        # The content already has \( \) or $$ $$, so get_text() should preserve them.

        text = element.get_text()

        # Clean up excessive newlines
        text = re.sub(r"\n\s*\n", "\n\n", text)
        print(text.strip() + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Fetch AtCoder editorial and format as Markdown."
    )
    parser.add_argument("url", help="URL of the AtCoder editorial")
    args = parser.parse_args()

    fetch_atcoder_editorial(args.url)
