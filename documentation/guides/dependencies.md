---
id: dependencies
title: Dependency Management
sidebar_label: Dependency Management
---

[general tags]: # "guides, dependency, dependency_management, imports, program, library"

Leo programs can import functionality from other programs and libraries. Any imported program or library is referred to as a dependency. There are four types of dependencies:

- **Network dependencies**: Programs already deployed on the Aleo network, fetched as pre-compiled bytecode.
- **Local dependencies**: Code on your filesystem. The code can be compiled Leo code, Aleo Instructions code, or a Leo library.
- **Workspace dependencies**: Other members of the same [workspace](./workspaces.md), resolved automatically from `workspace.json`.
- **Git dependencies**: Packages fetched from a git repository (a Leo program, a Leo library, or an Aleo Instructions file), pinned to an exact commit.

## Programs vs. Libraries

A regular Leo **program** has an on-chain identity (`program foo.aleo { }`). It can contain mappings and records. You can deploy a program to the Aleo network.

A Leo **library** is a source-only package with structs, constants, and helper functions. During compilation, Leo puts the library code in each program that uses it. Libraries can only be local or git dependencies. You cannot deploy a library.

See [Leo Libraries](../language/libraries.md) for details on how to write and use libraries.

## Adding Dependencies

### Network Dependencies

To add a program already deployed on the Aleo network as a dependency:

```bash
leo add credits.aleo --network
```

or

```bash
leo add credits --network
```

For mainnet dependencies:

```bash
NETWORK=mainnet leo add credits --network
```

You can also set `NETWORK=mainnet` in `.env`. Leo uses `ENDPOINT` from the environment unless you pass `--endpoint`. It verifies that the program exists on the selected network before it changes `program.json`. Use `--network-retries` to configure retries for this check.

This adds an entry to your `program.json`:

```json
{
  "program": "your_program.aleo",
  "version": "0.0.0",
  "description": "",
  "license": "MIT",
  "dependencies": [
    {
      "name": "credits.aleo",
      "location": "network",
      "network": "testnet",
      "path": null
    }
  ]
}
```

### Local Dependencies

To add a Leo package or Aleo Instructions file from your local filesystem as a dependency:

```bash
leo add my_library.aleo --local <PATH_TO_LIBRARY>
```

This records the path in `program.json`:

```json
{
  "dependencies": [
    {
      "name": "my_library.aleo",
      "location": "local",
      "network": null,
      "path": "./path/to/my_library"
    }
  ]
}
```

Local dependencies are compiled from source whenever you build. They never require network access.

**Leo libraries** are a special kind of local dependency. A library project (created with `leo new --lib`) contains only structs, constants, and helper functions — no program block, no mappings, no records. Because libraries have no on-chain identity, they can only be local dependencies and are never deployed. All library code is inlined into the consuming program at compile time. See [Leo Libraries](../language/libraries.md) for more.

### Workspace Dependencies

Inside a [workspace](./workspaces.md), members can depend on each other without specifying explicit paths. To add a workspace member as a dependency:

```bash
leo add --workspace token
```

or equivalently:

```bash
leo add -w token
```

This records a workspace dependency in `program.json`:

```json file=../code_snippets/workspaces/swap/program.json title="swap/program.json"
```

At build time, Leo resolves workspace dependencies by looking up the member in `workspace.json` and converting the entry to a local path automatically. The member is matched by directory name or program name (with or without the `.aleo` suffix).

Workspace dependencies require an enclosing `workspace.json`. If no workspace is found, or if the named member is not listed in the workspace, Leo reports an error.

To add a workspace member as a development dependency:

```bash
leo add --workspace --dev token
```

### Git Dependencies

To add a package from a git repository as a dependency:

```bash
leo add my_library --git https://github.com/example/my_library
```

Leo fetches the repository (so it can read the package's manifest and detect whether it is a program or a library, just like `--local`). By default the repository's default branch is tracked. Pin to a branch, tag, or revision:

```bash
leo add my_library --git https://github.com/example/my_library --branch main
leo add my_library --git https://github.com/example/my_library --tag v0.1.0
leo add my_library --git https://github.com/example/my_library --rev 0a1b2c3
```

This records a git dependency in `program.json`. For a concrete, working example, the package below depends on `helloworld.aleo` from the [Leo examples](https://github.com/ProvableHQ/leo-examples) repository, pinned to a commit:

```json file=../code_snippets/dependencies/git_dep/program.json title="program.json"
```

A git repository can contain a Leo program, a Leo library, or an Aleo Instructions (`.aleo`) file. Leo uses the dependency name to find the package in the checkout. It searches the repository for a `program.json` with a matching `program` field. If it does not find one, it searches the repository root for a `<name>.aleo` bytecode file.

Only public repositories are supported, fetched over HTTP(S). Leo never sends credentials, so private repositories and SSH URLs (`git@…`) are not supported.

#### The lock file

When Leo first resolves a git dependency, it writes a `leo.lock` file. This file is next to `program.json` or at the workspace root. It records the exact commit for each git dependency. For the preceding example:

```json file=../code_snippets/dependencies/git_dep/leo.lock title="leo.lock"
```

Subsequent builds reuse the locked commit, so builds are reproducible. Commit `leo.lock` to version control to share exact dependency versions. A change to `branch`, `tag`, or `rev` in `program.json` makes Leo resolve the dependency again and update the lock. The reference kind determines if a rebuild uses the network. A `tag` or `rev` uses the cache without network access. A `branch`, including the default branch, uses the remote during each build that has network access.

A `tag` or `rev` is immutable. After the lock operation, Leo reuses it from the cache. A `branch`, including the default branch, is mutable. During each build with network access, Leo resolves the latest branch commit and updates the lock. Thus, builds at different times can use different commits. Pin a `tag` or `rev` when you need a fixed dependency.

Pass `--offline` to `leo build` to skip all git fetching and build from the locked commits and the local cache, even for branch references. `leo remove` deletes the removed dependency's entries from `leo.lock`.

## `dependencies` vs. `dev_dependencies`

A manifest has two dependency lists, and they differ only in what can see them:

- **`dependencies`** are visible to your `src` program **and** to the package's in-package [tests](./testing.md). A dependency your program is built from is automatically available to the tests too.
- **`dev_dependencies`** are visible **only** to the in-package tests, never to `src`. Use this list for libraries or programs that the tests need but the program itself does not.

Tests can already use `dependencies`. Thus, put a library that `src` and tests use only in `dependencies`. Do not repeat it in `dev_dependencies`. Leo permits and deduplicates a repeated local library, but the entry is redundant.

## Manifest field reference

This reference applies to each dependency kind, not only workspace members. `leo add` completes these fields. If you edit `program.json` manually, Leo validates each dependency when it loads the manifest. Leo rejects incompatible field combinations. Each entry has a `location`, and the other fields depend on it:

| `location`  | `path`      | `edition`   | `network`            |
| ----------- | ----------- | ----------- | -------------------- |
| `network`   | not allowed | optional    | target network       |
| `local`     | required    | not allowed | —                    |
| `workspace` | not allowed | not allowed | —                    |
| `git`       | not allowed | not allowed | —                    |

The same rules apply to entries in `dev_dependencies`. `workspace` entries are looked up in `workspace.json` and resolved to a local path automatically. `git` entries additionally take a `git` object with a `url` and at most one of `branch`/`tag`/`rev`.

## Removing Dependencies

```bash
leo remove credits.aleo
```

## Using Dependencies

In your `main.leo` file, import dependencies before the program declaration:

```leo file=../code_snippets/dependencies/my_program/src/main.leo#file
```

## Dependency Resolution Process

When you run a Leo command, dependencies are resolved as follows:

1. **Read `program.json`** to find declared dependencies
2. **For each dependency:**
   - **Workspace**: Look up the member in `workspace.json` and resolve to a local path
   - **Local**: Read the Leo source from the specified path and compile it, or use Aleo Instructions file
   - **Network**: Fetch the bytecode from the Aleo network (or cache)
   - **Git**: Reuse the commit pinned in `leo.lock` if present. Otherwise clone the repository, resolve the reference to a commit, and check it out. The checked-out package is then treated exactly like a local one.
3. **Resolve transitive dependencies** - if your dependency imports other programs, those are fetched too
4. **Topologically sort** all programs so dependencies are processed before dependents

### Caching Behavior

Network dependencies are cached locally at `~/.aleo/registry/{network}/{program_name}/{edition}/` to avoid repeated downloads.

Git dependencies are checked out under `~/.aleo/git/checkouts/{repo}-{url_hash}/{commit}/`, keyed by the repository URL and the exact commit. Once a commit is checked out it is reused across all projects and dependencies without re-cloning.

Different commands handle caching differently:

| Command          | Cache Behavior       |
| ---------------- | -------------------- |
| `leo build`      | Uses cache           |
| `leo run`        | Uses cache           |
| `leo execute`    | Always fetches fresh |
| `leo deploy`     | Always fetches fresh |
| `leo upgrade`    | Always fetches fresh |
| `leo synthesize` | Always fetches fresh |

Commands that generate proofs (`execute`, `deploy`, `upgrade`, `synthesize`) always fetch fresh bytecode because proofs include commitments to the exact bytecode of dependencies. Using stale cached bytecode would produce invalid proofs if a dependency has been upgraded on-chain.

To force a fresh fetch during `build` or `run`:

```bash
leo build --no-cache
```

## Program Editions

An **edition** is the version number of a deployed program on the Aleo network:

- **Edition 0:** Initial deployment
- **Edition 1:** First upgrade
- **Edition 2:** Second upgrade
- ...and so on

By default, Leo fetches the **latest** edition of a network dependency. To pin to a specific edition:

```bash
leo add some_program.aleo --edition 3
```

This records the pinned edition in the manifest:

```json
{
  "name": "some_program.aleo",
  "location": "network",
  "network": "testnet",
  "path": null,
  "edition": 3
}
```

**When to pin editions:**

- When you need reproducible builds
- When a dependency upgrade would break your program
- When you want to avoid unexpected behavior changes

**Note:** Local dependencies do not have editions - they are always compiled from your current source code.

## Deploying Programs with Dependencies

### Recursive Deployment

When deploying a program that has local dependencies, use:

```bash
leo deploy --recursive
```

This deploys all local dependencies in topological order, then deploys your main program. Dependencies already deployed on-chain are skipped.

### Network Dependencies at Deploy Time

During deployment, Leo gets current bytecode for all network dependencies. This operation makes the deployment transaction reference the current on-chain editions. Since the V8 consensus upgrade, an edition 0 network dependency must have a constructor. If the constructor does not exist, Leo reports that you must first upgrade the dependency on-chain.
