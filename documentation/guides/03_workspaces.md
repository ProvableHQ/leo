---
id: workspaces
title: Workspaces
sidebar_label: Workspaces
---

[general tags]: # "guides, workspace, workspaces, multi-program, monorepo"

A Leo **workspace** groups multiple Leo packages (programs or libraries) under a single root directory. When you run commands like `leo build` or `leo test` from the workspace root, Leo operates on every member in the correct dependency order - no manual sequencing required.

Workspaces are useful when your application is made up of several interacting programs. For example, a DeFi protocol might have a `token` program and a `swap` program that depends on it. A workspace lets you build, test, and clean them all with a single command.

## Creating a Workspace

Create a `workspace.json` file in your project root. It contains a `members` array listing the relative paths to each member package:

```json
{
  "members": ["token", "swap"]
}
```

Each entry is a path to a directory containing a standard Leo package with its own `program.json` and `src/main.leo`.

## Directory Structure

A typical workspace looks like this:

```text
my_project/
├── workspace.json
├── token/
│   ├── program.json
│   └── src/
│       └── main.leo
└── swap/
    ├── program.json
    └── src/
        └── main.leo
```

Each member is a self-contained Leo package. It has its own `build/` and `output/` directories when compiled.

## Member Dependencies

Members can depend on each other using [workspace dependencies](./02_dependencies.md#workspace-dependencies). For example, if `swap` depends on `token`, add the dependency with:

```bash
cd swap/
leo add --workspace token
```

This writes a workspace dependency entry to the member's `program.json`:

```json file=../code_snippets/workspaces/swap/program.json title="swap/program.json"
```

A workspace dependency uses `"location": "workspace"` and requires no `path` - Leo automatically resolves the member's location from `workspace.json` at build time.

Alternatively, members can use [local path dependencies](./02_dependencies.md#local-dependencies) with `"location": "local"` and an explicit relative `path`. Workspace dependencies are preferred because they are shorter to write and do not break if you reorganize directories.

## Build Order

Leo automatically determines the correct build order by analyzing the dependency graph across workspace members. When one member depends on another, Leo ensures the dependency is built first.

In the example above, `token` has no dependencies and `swap` depends on `token`, so Leo builds `token` first, then `swap`. This ordering is computed automatically regardless of the order members are listed in `workspace.json`.

If the dependency graph contains a cycle (e.g., A depends on B and B depends on A), Leo reports an error rather than attempting to build.

## Working with Workspaces

### Building

From the workspace root, `leo build` compiles all members in dependency order:

```bash
leo build
```

From inside a member directory, it builds only that member:

```bash
cd token/
leo build
```

### Testing

From the workspace root, `leo test` runs test suites for all members:

```bash
leo test
```

From inside a member directory, it tests only that member:

```bash
cd swap/
leo test
```

### Cleaning

From the workspace root, `leo clean` removes build artifacts for all members:

```bash
leo clean
```

From inside a member directory, it cleans only that member:

```bash
cd token/
leo clean
```

## Targeting a Specific Member

Use the `--package` (or `-p`) flag to target a specific member from anywhere within the workspace:

```bash
leo build -p swap
leo test --package token
leo clean -p swap
```

The flag accepts any of:

- The member's directory name (e.g., `token`)
- The program name with `.aleo` suffix (e.g., `token.aleo`)
- The program name without suffix (e.g., `token`)

If the name does not match any workspace member, Leo reports an error and suggests checking the `members` list in `workspace.json`.

## Example

Here is a minimal workspace with two members. The `token` program exposes a `mint` function:

```leo file=../code_snippets/workspaces/token/src/main.leo#program
```

The `swap` program imports and calls into `token`:

```leo file=../code_snippets/workspaces/swap/src/main.leo#program
```

Building from the workspace root compiles `token` first (no dependencies), then `swap`.

## Backward Compatibility

Projects without a `workspace.json` continue to work exactly as before. Workspaces are purely opt-in - Leo only activates workspace behavior when it discovers a `workspace.json` by walking up the directory tree from the current working directory.
