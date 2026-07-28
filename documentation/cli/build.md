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

The build command automatically creates a `build/` directory. For a single project, this directory is in the project directory. For a [workspace](../guides/workspaces.md), it is at the workspace root and is shared by all members. Each program and dependency gets a `build/{program}/` directory with its compiled `.aleo` bytecode and ABI.

```bash title="console output:"
  Leo     2 statements before dead code elimination.
  Leo     2 statements after dead code elimination.
  Leo ✅ Compiled '{PROGRAM_NAME}.aleo' into Aleo instructions.
  Leo ✅ Generated ABI for program '{PROGRAM_NAME}.aleo'.
```

The build also generates an **ABI file** at `build/{PROGRAM_NAME}/abi.json` describing your program's public interface (transitions, mappings, and types). See the [ABI Generation guide](../guides/abi.md) for details on the format and type lowering specification.

## Checksums

The [`std::prog::function_checksum`](../language/standard_library.md#stdprog) function returns program, entry, and view function checksums. Use these checksums in a [constructor](../language/structure.md#constructor) that pins functions across upgrades. To print the checksums, pass `--checksums`:

```bash
leo build --checksums
```

```bash title="console output:"
  Leo     The program checksum is: '[141u8, 87u8, ...]'.
  Leo       `main` function checksum is: '[140u8, 56u8, ...]'.
  Leo       `peek` function checksum is: '[239u8, 16u8, ...]'.
```

Each checksum is the 32-byte SHA3-256 hash of the component's Aleo source. The [`--json-output`](./overview.md#--json-outputpath) build JSON also contains these checksums. They are integer arrays under `program_checksum` and `function_checksums`. Use `leo build --json-output` to consume them in a program.

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
