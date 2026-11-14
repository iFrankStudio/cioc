#!/usr/bin/env python3
"""Trusted GitHub Actions guard for untrusted Codex research patches.

Run after git apply --index in a read-only CI job and again before publishing.
"""
from __future__ import annotations

import subprocess


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", *args])


def validate() -> None:
    raw = git("diff", "--cached", "--name-status", "-z")
    tokens = raw.split(b"\0")
    if tokens[-1:] == [b""]:
        tokens.pop()
    if len(tokens) % 2:
        raise SystemExit("invalid staged change list")
    entries = []
    for i in range(0, len(tokens), 2):
        status, filename = tokens[i].decode(), tokens[i + 1].decode()
        if status not in {"A", "M"}:
            raise SystemExit(f"deletions, renames and mode changes disallowed: {filename}")
        if not filename or filename.startswith("/") or ".." in filename.split("/"):
            raise SystemExit("invalid path")
        if filename.startswith("cl1/") and filename.endswith(".py"):
            pass
        elif filename.startswith("cl1/") and filename.endswith(".md"):
            if status != "A":
                raise SystemExit(f"historical research document cannot be edited: {filename}")
        elif (filename.startswith("cl0/crates/lab/src/bin/cl1-study")
              and filename.endswith(".rs")):
            pass
        else:
            raise SystemExit(f"outside research allowlist: {filename}")
        modes = git("ls-files", "-s", "--", filename).decode().split()
        if not modes or modes[0] != "100644":
            raise SystemExit(f"only regular text files allowed: {filename}")
        entries.append(filename)
    if not entries or len(entries) > 8:
        raise SystemExit(f"expected 1..8 approved research files, got {len(entries)}")
    print("approved autonomous patch:", ", ".join(entries))


if __name__ == "__main__":
    validate()
