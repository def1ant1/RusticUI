#!/usr/bin/env python3
"""Synchronise shared README sections across the MUI framework adapters."""
from __future__ import annotations

import pathlib

REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
SHARED_SECTION = REPO_ROOT / "mui-shared" / "docs" / "shared-readme-sections.md"
# README targets relative to `examples/`
TARGETS = [
    pathlib.Path("mui-dioxus/README.md"),
    pathlib.Path("mui-leptos/README.md"),
    pathlib.Path("mui-ssr-accessibility/README.md"),
    pathlib.Path("mui-sycamore/README.md"),
    pathlib.Path("mui-yew/README.md"),
]
BEGIN_MARKER = "<!-- BEGIN_SHARED_SECTIONS -->"
END_MARKER = "<!-- END_SHARED_SECTIONS -->"


def sync_file(readme: pathlib.Path, shared: str) -> None:
    content = readme.read_text(encoding="utf-8")
    if BEGIN_MARKER not in content or END_MARKER not in content:
        raise SystemExit(f"Missing shared markers in {readme}")
    prefix, remainder = content.split(BEGIN_MARKER, 1)
    _, suffix = remainder.split(END_MARKER, 1)
    readme.write_text(
        f"{prefix}{BEGIN_MARKER}\n\n{shared}\n{END_MARKER}{suffix}",
        encoding="utf-8",
    )


def main() -> None:
    shared_text = SHARED_SECTION.read_text(encoding="utf-8").strip()
    examples_root = REPO_ROOT.parent / "examples"
    for target in TARGETS:
        readme_path = examples_root / target
        sync_file(readme_path, shared_text)


if __name__ == "__main__":
    main()
