# Documentation

Source for [docs.leo-lang.org](https://docs.leo-lang.org). Plain Markdown plus compilable Leo example projects under [`code_snippets/`](./code_snippets/).

Markdown imports snippets with `` ```leo file=../code_snippets/<project>/src/main.leo[#anchor] ``. Anchors are `// ANCHOR: <name>` / `// ANCHOR_END: <name>` comments in the `.leo` source.

## Lint before opening a PR

Run from the repo root:

```bash
# Auto-fix what's safe, then verify nothing is left.
npx markdownlint-cli@0.47.0 --config .markdownlint.yaml --fix 'documentation/**/*.md'
npx markdownlint-cli@0.47.0 --config .markdownlint.yaml 'documentation/**/*.md'

# Every code snippet must still compile.
for d in $(find documentation/code_snippets -name program.json -not -path '*/build/*' | xargs -n1 dirname); do
  (cd "$d" && leo build) || break
done
```

CI enforces both — `docs-lint` and `build-doc-code-snippets`.
