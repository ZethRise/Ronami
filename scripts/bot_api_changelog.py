#!/usr/bin/env python3
"""Prints the changelog entry of a Bot API version, e.g. `bot_api_changelog.py 10.3`."""

import html
import re
import sys

from bot_api_spec import CHANGELOG_URL, fetch


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: bot_api_changelog.py <version>, e.g. 10.3")
    version = sys.argv[1]

    page = fetch(CHANGELOG_URL)
    versions = re.findall(r"Bot API (\d+\.\d+)", page)
    marker = f"Bot API {version}"
    start = page.find(marker)
    if start == -1:
        latest = versions[0] if versions else "unknown"
        sys.exit(f"Bot API {version} is not published yet; the latest version is {latest}")

    end = len(page)
    for other in sorted(set(versions), key=lambda v: page.find(f"Bot API {v}")):
        position = page.find(f"Bot API {other}")
        if position > start:
            end = position
            break

    text = page[start:end]
    text = re.sub(r"<script.*?</script>", "", text, flags=re.S)
    text = re.sub(r"</?(p|ul|h3|h4|br)[^>]*>", "\n", text)
    text = re.sub(r"<li>", "\n- ", text)
    text = html.unescape(re.sub(r"<[^>]+>", "", text))
    print(re.sub(r"\n{3,}", "\n\n", text).strip())


if __name__ == "__main__":
    main()
