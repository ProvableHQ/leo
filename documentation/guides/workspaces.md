---
id: workspaces
title: Workspaces
sidebar_label: Workspaces
---

[general tags]: # "guides, workspace, workspaces, multi-program, monorepo"

A Leo **workspace** groups multiple Leo packages under one root directory.
Run commands such as `leo build` or `leo test` from the workspace root.
Leo operates on each member in dependency order. You do not have to specify the sequence.

Workspaces are useful when your application is made up of several interacting programs. For example, a DeFi protocol might have a `token` program and a `swap` program that depends on it. A workspace lets you build, test, and clean them all with a single command.

## Creating a Workspace

The quickest way to start a workspace is to scaffold one with `leo new`:

```bash
leo new --workspace my_project
```

This creates `my_project/` with a `workspace.json` that has an empty `members` array.
Do not use `--workspace` and `--library` together.
A workspace groups packages, but it is not a package. Thus, it has no `src/`, `program.json`, or `tests/` directory.

Equivalently, you can create `workspace.json` by hand in any directory. It contains a `members` array listing the relative paths to each member package:

```json
{
  "members": ["token", "swap"]
}
```

Each entry is a path to a directory containing a standard Leo package with its own `program.json` and `src/main.leo`.

### Glob Members

Entries in `members` can also be glob patterns, resolved relative to the workspace root:

```json
{
  "members": ["libraries/core", "programs/*"]
}
```

Standard `glob` syntax is supported, including `*` (matches a single path segment), `**` (matches recursively across directories), `?`, and character classes like `[abc]`. A glob match is included only if the matched directory contains a `program.json`. Other matches (files, directories without a manifest, non-UTF-8 paths) are silently skipped.

A glob that matches zero packages logs a warning and continues - it is not an error:

```text
workspace member glob `programs/*` in <root> matched no packages
```

A literal entry for a missing directory still causes an error. Thus, Leo strictly validates explicit paths.
Leo resolves literal entries before globs and removes duplicate canonical paths.
It includes a directory only once when a literal entry and glob both match it.

### Adding Members

When `leo new <name>` is run anywhere inside a workspace, the new package's path is appended to the enclosing `workspace.json` automatically and Leo prints:

```text
Added <name> to the enclosing workspace.
```

Leo does not append a package that an existing entry includes.
The existing entry can be an equal literal path or a matching glob.
If the new package is outside the workspace root, Leo prints a warning and does not change `workspace.json`.
For example, `leo new ../sibling` causes this warning:

```text
new package at `...` is not inside the discovered workspace root `...`; skipping auto-add
```

Leo keeps the existing `members` order and puts new entries at the end.
To avoid manual changes to `members`, use a glob entry such as `programs/*`. See [Glob Members](#glob-members).
Leo finds new packages in that directory without a change to `workspace.json`.

## Directory Structure

A typical workspace looks like this:

```text
my_project/
├── workspace.json
├── .gitignore        # ignores `build/`
├── build/            # shared across all members (created on `leo build`)
│   ├── token/
│   │   ├── token.aleo
│   │   └── abi.json
│   └── swap/
│       ├── swap.aleo
│       └── abi.json
├── token/
│   ├── program.json
│   └── src/
│       └── main.leo
└── swap/
    ├── program.json
    └── src/
        └── main.leo
```

Build artifacts are in one `build/` directory at the workspace root.
The compilation unit name identifies each artifact.
After Leo builds a unit as a member or dependency, other members reuse it.
Run `leo clean` from any workspace directory to remove the shared `build/` directory.

## Member Dependencies

Members can depend on each other using [workspace dependencies](./dependencies.md#workspace-dependencies). For example, if `swap` depends on `token`, add the dependency with:

```bash
cd swap/
leo add --workspace token
```

This writes a workspace dependency entry to the member's `program.json`:

```json file=../code_snippets/workspaces/swap/program.json title="swap/program.json"
```

A workspace dependency uses `"location": "workspace"` and requires no `path` - Leo automatically resolves the member's location from `workspace.json` at build time.

Alternatively, members can use [local path dependencies](./dependencies.md#local-dependencies) with `"location": "local"` and an explicit relative `path`. Workspace dependencies are preferred because they are shorter to write and do not break if you reorganize directories.

## Build Order

Leo automatically determines the correct build order by analyzing the dependency graph across workspace members. When one member depends on another, Leo ensures the dependency is built first.

In the example above, `token` has no dependencies and `swap` depends on `token`, so Leo builds `token` first, then `swap`. This ordering is computed automatically regardless of the order members are listed in `workspace.json`.

If the dependency graph contains a cycle, Leo reports an error and does not build.
For example, a cycle occurs when A depends on B and B depends on A.

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

### Deploying

From the workspace root, `leo deploy` deploys all members in dependency order using a shared VM for accurate fee estimation. Programs shared across members are deployed only once:

```bash
leo deploy --broadcast
```

From inside a member directory, it deploys only that member:

```bash
cd swap/
leo deploy --broadcast
```

## Targeting a Specific Member

Use the `--package` (or `-p`) flag to target a specific member from anywhere within the workspace:

```bash
leo build -p swap
leo test --package token
leo deploy --broadcast -p swap
leo clean -p swap
```

The flag accepts any of:

- The member's directory name (for example, `token`)
- The program name with `.aleo` suffix (for example, `token.aleo`)
- The program name without suffix (for example, `token`)

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
