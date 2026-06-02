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

On invoking the build command, Leo automatically creates a `build/`⁠ folder in the project directory - or, when the package is inside a [workspace](../guides/03_workspaces.md), at the workspace root, shared across every member. Inside it, every program - your own program and each dependency - gets its own `build/{program}/` directory containing its compiled `.aleo` bytecode and ABI.

```bash title="console output:"
  Leo     2 statements before dead code elimination.
  Leo     2 statements after dead code elimination.
  Leo     The program checksum is: '[...]'.
  Leo ✅ Compiled '{PROGRAM_NAME}.aleo' into Aleo instructions.
  Leo ✅ Generated ABI for program '{PROGRAM_NAME}.aleo'.
```

The build also generates an **ABI file** at `build/{PROGRAM_NAME}/abi.json` describing your program's public interface (transitions, mappings, and types). See the [ABI Generation guide](../guides/11_abi.md) for details on the format and type lowering specification.

## Flags

```text
--enable-ast-spans
    Enable spans in AST snapshots.
--enable-initial-ast-snapshot
    Write an AST snapshot immediately after parsing.
--enable-all-ast-snapshots
    Writes all AST snapshots for the different compiler phases.
--ast-snapshots <AST_SNAPSHOTS>...
    Comma separated list of passes whose AST snapshots to capture.
--build-tests
    Build tests along with the main program and dependencies.
--no-cache
    Don't use the dependency cache.
--no-local
    Don't use the local source code.
--private-key <PRIVATE_KEY>
    The private key to use for the deployment. Overrides the `PRIVATE_KEY` environment variable.
--network <NETWORK>
    The network to deploy to. Overrides the `NETWORK` environment variable.
--endpoint <ENDPOINT>
    The endpoint to deploy to. Overrides the `ENDPOINT` environment variable.
--network-retries <N>
    Number of times to retry a network request on transient transport failure, with
    exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the
    NETWORK_RETRIES environment variable. Defaults to 2. HTTP errors and broadcast
    calls are not retried.
--devnet
    Whether the network is a devnet. If not set, defaults to the `DEVNET` environment variable.
--consensus-heights <CONSENSUS_HEIGHTS>
    Optional consensus heights to use. This should only be set if you are using a custom devnet.
```

## Workspace Behavior

When run inside a [workspace](../guides/03_workspaces.md):

- **From workspace root:** Builds all members in dependency order.
- **From a member directory:** Builds only that member.
- **With `--package <NAME>`:** Builds only the specified member.

```bash
# Build all workspace members
leo build

# Build only the swap member
leo build -p swap
```
