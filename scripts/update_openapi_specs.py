#!/usr/bin/env python3
"""
NEXON MapleStory OpenAPI Specification Updater

This script fetches the latest OpenAPI specifications from the Nexon Open API website,
extracts the OpenAPI YAML file URLs and metadata from __NEXT_DATA__, and downloads/updates
the YAML files in the target directory (default: openapi/).

It detects prefixes like '14_', '18_', '27_', '62_' to clean up old versions and overwrite
with the latest specifications.
"""

import argparse
import json
import os
import re
import sys
import urllib.request
from pathlib import Path

BASE_URL = "https://openapi.nexon.com/ko/game/maplestory/?id=14"
USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/124.0.0.0 Safari/537.36"
)


def fetch_html(url: str) -> str:
    """Fetch HTML content from the given URL with a custom User-Agent."""
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=30) as response:
        return response.read().decode("utf-8")


def extract_next_data(html: str) -> dict:
    """Extract and parse the JSON payload from the <script id="__NEXT_DATA__"> tag."""
    pattern = r'<script\s+id="__NEXT_DATA__"\s+type="application/json">\s*(.*?)\s*</script>'
    match = re.search(pattern, html, re.DOTALL)
    if not match:
        raise ValueError("Could not find '__NEXT_DATA__' script tag in the page HTML.")
    return json.loads(match.group(1))


def fetch_file_content(url: str) -> bytes:
    """Download a file from url and return its raw bytes."""
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=30) as response:
        return response.read()


def update_specs(target_dir: Path) -> bool:
    """
    Fetch the latest OpenAPI specs and metadata, updating target_dir.
    Only writes files if their content has actually changed.
    Returns True if any files were changed/added/deleted.
    """
    target_dir.mkdir(parents=True, exist_ok=True)
    print(f"Fetching page from: {BASE_URL}")
    html = fetch_html(BASE_URL)

    data = extract_next_data(html)
    page_props = data.get("props", {}).get("pageProps", {})
    category_result = page_props.get("categoryResult", [])

    if not category_result:
        raise ValueError("No categoryResult found in pageProps.")

    # 1. Check and save formatted maplestory.json only if changed
    json_path = target_dir / "maplestory.json"
    new_json_content = (json.dumps(data, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    files_changed = False

    if not json_path.exists() or json_path.read_bytes() != new_json_content:
        json_path.write_bytes(new_json_content)
        print(f"Updated metadata: {json_path}")
        files_changed = True
    else:
        print(f"Metadata is identical, skipping: {json_path}")

    # 2. Iterate over each category and language specification
    for category in category_result:
        cat_id = category.get("id")
        cat_name = category.get("categoryName")
        languages = category.get("language", [])

        for lang in languages:
            lang_id = lang.get("id")
            file_name = lang.get("fileName")
            file_url = lang.get("fileUrl")

            if not file_name or not file_url:
                continue

            # Identify the prefix (e.g., '14_', '62_', '18_', '27_')
            prefix = f"{lang_id}_"
            if not file_name.startswith(prefix) and "_" in file_name:
                prefix = file_name.split("_")[0] + "_"

            dest_file = target_dir / file_name

            # Check for existing files matching this prefix
            existing_prefix_files = [
                p for p in target_dir.glob(f"{prefix}*.yaml")
            ] + [
                p for p in target_dir.glob(f"{prefix}*.yml")
            ]

            # Fetch remote content
            remote_bytes = fetch_file_content(file_url)

            # Check if current dest_file exists and is identical
            if dest_file.exists() and dest_file.read_bytes() == remote_bytes:
                # Content and filename are completely identical - do nothing!
                print(f"Spec is identical, skipping: {file_name} [{cat_name}]")
            else:
                # Content or filename changed - update the file
                print(f"Updating [{cat_name}] spec: {file_name}")
                dest_file.write_bytes(remote_bytes)
                files_changed = True

                # Remove any other older versions with this prefix
                for old_file in existing_prefix_files:
                    if old_file.name != file_name:
                        print(f"Removing outdated spec: {old_file.name} (prefix: {prefix})")
                        old_file.unlink()
                        files_changed = True

    return files_changed


def main():
    parser = argparse.ArgumentParser(description="Update MapleStory OpenAPI Specifications")
    parser.add_argument(
        "--dir",
        type=Path,
        default=Path("openapi"),
        help="Target directory to store OpenAPI specs (default: openapi)",
    )
    args = parser.parse_args()

    try:
        changed = update_specs(args.dir)
        if changed:
            print("OpenAPI specs updated successfully with changes.")
        else:
            print("All OpenAPI specs are already up to date.")
    except Exception as e:
        print(f"Error updating OpenAPI specs: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
