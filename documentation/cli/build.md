---
id: cli_build
title: ""
sidebar_label: Build
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_build, build, compile, program"

# `leo build`

To compile your program into Aleo instructions and verify that it builds properly, run:

```bash
leo build
```

On invoking the build command, Leo automatically creates a `build/`⁠ folder in the project directory - or, when the package is inside a [workspace](../guides/workspaces.md), at the workspace root, shared across every member. Inside it, every program - your own program and each dependency - gets its own `build/{program}/` directory containing its compiled `.aleo` bytecode and ABI.

```bash title="console output:"
  Leo     2 statements before dead code elimination.
  Leo     2 statements after dead code elimination.
  Leo ✅ Compiled '{PROGRAM_NAME}.aleo' into Aleo instructions.
  Leo ✅ Generated ABI for program '{PROGRAM_NAME}.aleo'.
```

The build also generates an **ABI file** at `build/{PROGRAM_NAME}/abi.json` describing your program's public interface (transitions, mappings, and types). See the [ABI Generation guide](../guides/abi.md) for details on the format and type lowering specification.

## Checksums

The program checksum and the checksum of each entry and view function are the values that the [`std::prog::function_checksum`](../language/standard_library.md#stdprog) stdlib function returns, so they are useful when writing a [constructor](../language/structure.md#constructor) that pins specific functions across upgrades. To print them, pass `--checksums`:

```bash
leo build --checksums
```

```bash title="console output:"
  Leo     The program checksum is: '[141u8, 87u8, ...]'.
  Leo       `main` function checksum is: '[140u8, 56u8, ...]'.
  Leo       `peek` function checksum is: '[239u8, 16u8, ...]'.
```

Each checksum is the SHA3-256 of the component's Aleo source, as 32 bytes. The same checksums are written to the [`--json-output`](./overview.md#--json-outputpath) build JSON (as integer arrays under `program_checksum` and `function_checksums`), so `leo build --json-output` is a convenient way to consume them programmatically.

## Flags

```text
--build-tests
    Build tests along with the main program and dependencies.
--checksums
    Print the program checksum and the checksum of each entry and view function
    (the `std::prog::function_checksum` targets).
--no-cache
    Don't use the dependency cache.
--no-local
    Don't use the local source code.
--network <NETWORK>
    The network to build for. Overrides the `NETWORK` environment variable.
--endpoint <ENDPOINT>
    The endpoint to resolve network dependencies from. Overrides the `ENDPOINT` environment variable.
--network-retries <N>
    Number of times to retry a network request on transient transport failure, with
    exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the
    NETWORK_RETRIES environment variable. Defaults to 2. HTTP errors and broadcast
    calls are not retried.
```

## Workspace Behavior

When run inside a [workspace](../guides/workspaces.md):

- **From workspace root:** Builds all members in dependency order.
- **From a member directory:** Builds only that member.
- **With `--package <NAME>`:** Builds only the specified member.

```bash
# Build all workspace members
leo build

# Build only the swap member
leo build -p swap
```
