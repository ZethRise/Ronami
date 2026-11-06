"""Helpers shared by the Bot API scripts: download and parse the API reference."""

import html
import re
import urllib.request

API_URL = "https://core.telegram.org/bots/api"
CHANGELOG_URL = "https://core.telegram.org/bots/api-changelog"


def fetch(url):
    request = urllib.request.Request(url, headers={"User-Agent": "ronami-bot-api-scripts"})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read().decode("utf-8")


def parse_reference(page):
    """Returns `{heading: {"description": str, "fields": [(name, type, description)]}}`.

    Methods start with a lowercase letter (`sendMessage`), objects with an
    uppercase one (`Message`).
    """
    sections = re.split(r'<h4><a class="anchor" name="[^"]*"[^>]*>(?:<i[^>]*></i>)?</a>', page)
    result = {}
    for body in sections[1:]:
        body = body.split("<h3>")[0].split("<h4>")[0]
        heading = html.unescape(re.sub(r"<[^>]+>", "", body.split("</h4>")[0])).strip()
        fields = []
        for row in re.findall(r"<tr>.*?</tr>", body, flags=re.S):
            cells = re.findall(r"<td[^>]*>(.*?)</td>", row, flags=re.S)
            if len(cells) >= 3:
                cells = [html.unescape(re.sub(r"<[^>]+>", "", cell)).strip() for cell in cells]
                fields.append((cells[0], cells[1], cells[-1]))
        if heading:
            result[heading] = {"fields": fields}
    return result
