#!/usr/bin/env python3
"""
Compile every Leo snippet under `documentation/code_snippets/` to verify that
documentation examples stay valid as the language evolves.

Each `.leo` file is treated as a self-contained snippet. Multi-program
snippets use the `// --- Next Program --- //` separator (same convention the
compiler test framework uses) and may declare imports between segments.

For each snippet the script materializes a temporary project structure
(`program.json` + `src/main.leo` per program), wires up local dependencies
based on `import X.aleo;` statements, then runs `leo build`.

Usage:
    scripts/build_doc_snippets.py
    scripts/build_doc_snippets.py --leo /path/to/leo
    scripts/build_doc_snippets.py --filter operators
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

# Mirror of `PROGRAM_DELIMITER` in `crates/compiler/src/test_utils.rs`.
NEXT_PROGRAM_SEP = "// --- Next Program --- //"
PROGRAM_RE = re.compile(r"^\s*program\s+([\w]+\.aleo)\b[^{]*\{", re.MULTILINE)
IMPORT_RE = re.compile(r"^\s*import\s+([\w]+\.aleo)\s*;", re.MULTILINE)


def parse_segments(content: str) -> list[tuple[str, str]]:
    """Split a snippet into (program_name, source) tuples in declaration order.

    Segments without a `program X.aleo {` declaration are skipped silently —
    that lets us tolerate leading comments or scratch content in a file.
    """
    out: list[tuple[str, str]] = []
    for part in content.split(NEXT_PROGRAM_SEP):
        m = PROGRAM_RE.search(part)
        if m:
            out.append((m.group(1), part.strip() + "\n"))
    return out


def write_project(work: Path, name: str, src: str, deps: list[dict]) -> Path:
    """Write a single program project (program.json + src/main.leo).

    Schema mirrors `Manifest` in `crates/package/src/manifest.rs`.
    """
    proj = work / name.replace(".aleo", "")
    (proj / "src").mkdir(parents=True, exist_ok=True)
    (proj / "src" / "main.leo").write_text(src)
    program_json = {
        "program": name,
        "version": "0.1.0",
        "description": "",
        "license": "MIT",
        "dependencies": deps or None,
        "dev_dependencies": None,
    }
    (proj / "program.json").write_text(json.dumps(program_json, indent=2) + "\n")
    return proj


def find_owning_project(leo_file: Path) -> "Path | None":
    """Walk up from `leo_file` looking for a sibling `program.json`. Returns
    the project root, or None if `leo_file` is a bare snippet."""
    for parent in leo_file.parents:
        if (parent / "program.json").is_file():
            return parent
    return None


def run_leo_build(leo_bin: str, project: Path) -> tuple[bool, str]:
    """Run `leo build` in `project`. Returns (ok, captured_output)."""
    result = subprocess.run(
        [leo_bin, "--disable-update-check", "build"],
        cwd=project,
        capture_output=True,
        text=True,
    )
    return result.returncode == 0, f"--- {project} ---\n{result.stdout}{result.stderr}"


def build_snippet(leo_bin: str, leo_file: Path, scratch_root: Path) -> tuple[bool, str]:
    """Build every program in `leo_file`. Returns (success, captured_output)."""
    segments = parse_segments(leo_file.read_text())
    if not segments:
        return True, "(no program declarations — skipped)"

    work = scratch_root / leo_file.stem
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)

    # Project dirs are deterministic, so we can compute them up front and use
    # them to resolve dependency paths before any file is written.
    project_dirs = {name: work / name.replace(".aleo", "") for name, _ in segments}

    captured: list[str] = []
    # Build in declaration order — earlier programs in the file are deps of later ones.
    for name, src in segments:
        deps = []
        seen = set()
        for m in IMPORT_RE.finditer(src):
            imp = m.group(1)
            if imp in seen or imp not in project_dirs:
                continue
            seen.add(imp)
            rel = os.path.relpath(project_dirs[imp], project_dirs[name])
            deps.append({"name": imp, "location": "local", "path": rel, "edition": None})
        write_project(work, name, src, deps)

        ok, output = run_leo_build(leo_bin, project_dirs[name])
        captured.append(output)
        if not ok:
            return False, "\n".join(captured)

    return True, "\n".join(captured)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--leo", default="leo", help="Path to the leo binary (default: PATH lookup).")
    parser.add_argument("--root", default="documentation/code_snippets", help="Root directory to walk.")
    parser.add_argument("--scratch", default=None, help="Scratch directory for temp projects (default: mktemp).")
    parser.add_argument("--filter", default=None, help="Only build files whose path contains this substring.")
    parser.add_argument("--verbose", "-v", action="store_true", help="Print compiler output for every snippet.")
    parser.add_argument("--keep-scratch", action="store_true", help="Don't delete the scratch dir on exit.")
    args = parser.parse_args()

    leo_bin = shutil.which(args.leo) or args.leo
    root = Path(args.root)
    if not root.is_dir():
        print(f"error: root {root} does not exist", file=sys.stderr)
        return 2

    scratch_root = Path(args.scratch) if args.scratch else Path(tempfile.mkdtemp(prefix="leo-doc-snippets-"))
    scratch_root.mkdir(parents=True, exist_ok=True)

    # One work item per project (regardless of how many .leo files live under it)
    # plus one item per bare snippet.
    seen_projects: set[Path] = set()
    work_items: list[tuple[str, Path]] = []
    for leo_file in sorted(root.rglob("*.leo")):
        project = find_owning_project(leo_file)
        if project is not None:
            if project not in seen_projects:
                seen_projects.add(project)
                work_items.append(("project", project))
        else:
            work_items.append(("snippet", leo_file))

    if args.filter:
        work_items = [w for w in work_items if args.filter in str(w[1])]

    if not work_items:
        print(f"error: no work found under {root}" + (f" matching '{args.filter}'" if args.filter else ""), file=sys.stderr)
        return 2

    print(f"Building {len(work_items)} item(s) with {leo_bin}")
    print(f"Scratch: {scratch_root}")

    failed: list[Path] = []
    for kind, target in work_items:
        rel = target.relative_to(root)
        if kind == "project":
            ok, output = run_leo_build(leo_bin, target)
        else:
            ok, output = build_snippet(leo_bin, target, scratch_root)
        if ok:
            print(f"  ok    {rel}")
            if args.verbose:
                print(output)
        else:
            print(f"  FAIL  {rel}")
            print(output)
            failed.append(target)

    print()
    print(f"=== {len(work_items) - len(failed)}/{len(work_items)} succeeded ===")

    if not args.keep_scratch:
        shutil.rmtree(scratch_root, ignore_errors=True)

    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
