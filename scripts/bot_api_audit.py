#!/usr/bin/env python3
"""Reports methods, objects and fields of the live Bot API reference that the Rust code lacks.

The check is textual, so it has false positives for things that are modelled
differently from the reference (enum variants, flattened fields, tag fields,
`FileMeta` based files); read the output as a list of places to look at.
"""

import glob
import os
import re

from bot_api_spec import API_URL, fetch, parse_reference

CORE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "crates", "ronami-core", "src")


def snake_case(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def main():
    reference = parse_reference(fetch(API_URL))
    sources = {}
    for path in glob.glob(os.path.join(CORE, "**", "*.rs"), recursive=True):
        if "codegen" not in path:
            with open(path, encoding="utf-8") as file:
                sources[path] = file.read()

    missing_methods, missing_objects, missing_fields = [], [], []
    for heading, section in reference.items():
        fields = [name for name, _, _ in section["fields"]]
        if not fields or not heading.replace(" ", "").isalnum():
            continue

        if heading[0].islower():
            payload = os.path.join(CORE, "payloads", snake_case(heading) + ".rs")
            if payload not in sources:
                missing_methods.append(heading)
                continue
            for field in fields:
                if not re.search(rf"\bpub {re.escape(field)}:", sources[payload]):
                    missing_fields.append((heading, field))
            continue

        candidates = [
            text
            for text in sources.values()
            if re.search(rf"\b(struct|enum) {heading}\b", text)
        ]
        if not candidates:
            missing_objects.append(heading)
            continue
        text = "\n".join(candidates)
        if heading in ("Message", "MessageCommon"):
            text = "\n".join(t for p, t in sources.items() if "types/message" in p)
        for field in fields:
            if field != "type" and not re.search(rf"\b{re.escape(field)}\b", text):
                missing_fields.append((heading, field))

    print("Methods without a payload:", missing_methods or "none")
    print("Objects without a Rust type:", missing_objects or "none")
    print("Fields not found in the Rust type:", missing_fields or "none")


if __name__ == "__main__":
    main()
