# Documentation

Source for [docs.leo-lang.org](https://docs.leo-lang.org). Plain Markdown plus compilable Leo snippets under [`code_snippets/`](./code_snippets/).

Most snippets are bare `.leo` files. Multi-program snippets put each program in its own segment separated by `// --- Next Program --- //` (the same convention the compiler test framework uses); a runner script materializes a temp project per segment and wires up local dependencies based on `import X.aleo;` lines. A few snippets that exercise multi-file libraries or modules remain as full project directories with their own `program.json`.

Markdown imports snippets with `` ```leo file=../code_snippets/<path>.leo[#anchor] ``. Anchors are `// ANCHOR: <name>` / `// ANCHOR_END: <name>` comments in the `.leo` source.

## Lint before opening a PR

Run from the repo root:

```bash
# Auto-fix what's safe, then verify nothing is left.
npx markdownlint-cli@0.47.0 --config .markdownlint.yaml --fix 'documentation/**/*.md'
npx markdownlint-cli@0.47.0 --config .markdownlint.yaml 'documentation/**/*.md'

# Every snippet must still compile.
scripts/build_doc_snippets.py --leo target/debug/leo
```

CI enforces both — `docs-lint` and `build-doc-code-snippets`.
