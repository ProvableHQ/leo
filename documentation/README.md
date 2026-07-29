# Documentation

This directory contains the source for [docs.leo-lang.org](https://docs.leo-lang.org). It contains Markdown files and compilable Leo example projects in [`code_snippets/`](./code_snippets/).

Markdown files import snippets with `` ```leo file=../code_snippets/<project>/src/main.leo[#anchor] ``. The `.leo` source uses `// ANCHOR: <name>` and `// ANCHOR_END: <name>` comments for anchors.

## Writing standard

Use [ASD-STE100 Simplified Technical English, Issue 9](https://www.asd-ste100.org/assets/files/ASD-STE100_ISSUE9.pdf) for all authored prose in this directory. Use the official standard as the primary reference.

Do not change code, commands, identifiers, literal output, or error messages to comply with the writing standard. These items must remain technically correct.

Names from Leo, the Leo CLI, Aleo, snarkVM, APIs, and cryptography are approved technical terms. Use the official spelling of each term. Use one term for one concept.

### Procedures

- Start each instruction with an imperative verb.
- Put a required condition before the instruction.
- Give one instruction in each sentence. Combine actions only when they occur at the same time.
- Use no more than 20 words in each sentence.
- Use notes only for information. Do not put instructions in notes.

### Descriptions

- Use the active voice when you know the actor.
- Use no more than 25 words in each sentence.
- Give one topic in each paragraph.
- Use no more than six sentences in each paragraph.
- Give complex information gradually. Use vertical lists when they make the text easier to understand.

### Words and punctuation

- Use American English spelling.
- Use approved words only for their approved meaning and part of speech.
- Do not use contractions, slang, jargon, or Latin abbreviations.
- Do not use semicolons.
- Do not use an `-ing` verb form unless it is part of an approved technical term.
- Use the same wording and sentence structure for repeated instructions.
- Use a warning or caution label for safety instructions. State the command first, and then explain the risk.

## Lint before opening a PR

Run these commands from the repository root:

```bash
# Apply safe fixes. Then, verify that no errors remain.
npx markdownlint-cli@0.47.0 --config .markdownlint.yaml --fix 'documentation/**/*.md'
npx markdownlint-cli@0.47.0 --config .markdownlint.yaml 'documentation/**/*.md'

# Compile each code snippet. Use `leo test` when a snippet has a tests/ directory.
for d in $(find documentation/code_snippets -name program.json -not -path '*/build/*' | xargs -n1 dirname); do
  if [ -d "$d/tests" ]; then
    (cd "$d" && leo test) || break
  else
    (cd "$d" && leo build) || break
  fi
done
```

CI runs the `docs-lint` and `build-doc-code-snippets` checks.
