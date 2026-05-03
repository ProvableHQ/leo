---
id: dependencies
title: Dependency Management
sidebar_label: Dependency Management
---

[general tags]: # "guides, dependency, dependency_management, imports, program, library"

Leo programs can import functionality from other programs and libraries. Any imported program or library is referred to as a dependency. There are two types of dependencies:

- **Network dependencies**: Programs already deployed on the Aleo network, fetched as pre-compiled bytecode.
- **Local dependencies**: Code on your filesystem; either Leo code compiled from source, Aleo Instructions code, or a Leo library.

## Programs vs. Libraries

A regular Leo **program** has an on-chain identity (`program foo.aleo { }`), can hold mappings and records, and is deployed to the Aleo network. A Leo **library** is a source-only package containing structs, constants, and helper functions with no on-chain footprint — all library code is inlined into programs that use it at compile time. Libraries can only be local dependencies; they are never deployed.

See [Leo Libraries](../language/06_libraries.md) for details on how to write and use libraries.

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
leo add credits --network mainnet
```

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

**Leo libraries** are a special kind of local dependency. A library project (created with `leo new --lib`) contains only structs, constants, and helper functions — no program block, no mappings, no records. Because libraries have no on-chain identity, they can only be local dependencies and are never deployed. All library code is inlined into the consuming program at compile time. See [Leo Libraries](../language/06_libraries.md) for more.

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
   - **Local**: Read the Leo source from the specified path and compile it, or use Aleo Instructions file
   - **Network**: Fetch the bytecode from the Aleo network (or cache)
3. **Resolve transitive dependencies** - if your dependency imports other programs, those are fetched too
4. **Topologically sort** all programs so dependencies are processed before dependents

### Caching Behavior

Network dependencies are cached locally at `~/.aleo/registry/{network}/{program_name}/{edition}/` to avoid repeated downloads.

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

**Note:** Local dependencies don't have editions - they're always compiled from your current source code.

## Deploying Programs with Dependencies

### Recursive Deployment

When deploying a program that has local dependencies, use:

```bash
leo deploy --recursive
```

This deploys all local dependencies in topological order, then deploys your main program. Dependencies already deployed on-chain are skipped.

### Network Dependencies at Deploy Time

When you deploy, Leo fetches fresh bytecode for all network dependencies to ensure your deployment transaction references the current on-chain editions. If a network dependency is at edition 0 and lacks a constructor (required since the V8 consensus upgrade), Leo will error with a clear message explaining that the dependency needs to be upgraded on-chain first.
