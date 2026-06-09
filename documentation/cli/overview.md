---
id: cli_overview
title: The Leo Command Line Interface
sidebar_label: Overview
---

[general tags]: # "cli"

The Leo CLI is a command line interface tool that comes equipped with the Leo compiler.

:::tip
You can print the list of commands by running `leo --help`
:::

## Commands

- [`abi`](./abi.md) - Generate an ABI JSON file from a compiled `.aleo` bytecode file.
- [`account`](./account.md) - Create a new Aleo account, sign and verify messages.
  - [`new`](./account.md#leo-account-new) - Generates a new Aleo account.
  - [`import`](./account.md#leo-account-import) - Derive an Aleo account from a private key.
  - [`sign`](./account.md#leo-account-sign) - Sign a message using your Aleo private key.
  - [`verify`](./account.md#leo-account-verify) - Verify a message and signature from an Aleo address.
- [`add`](./add.md) - Add a new onchain or local dependency to the current project.
- [`build`](./build.md) - Compile the current project.
- [`clean`](./clean.md) - Clean the build and output artifacts.
- [`deploy`](./deploy.md) - Deploy a program to the Aleo network.
- [`devnet`](./devnet.md) - Initialize a local devnet.
- [`devnode`](./devnode.md) - Run a local lightweight devnode.
- [`execute`](./execute.md) - Execute a program and produce a transaction containing a proof.
- [`new`](./new.md) - Create a new Leo project in a new directory.
- [`query`](./query.md) - Query live data and state from the Aleo network.
  - [`block`](./query.md#leo-query-block) - Query block information.
  - [`transaction`](./query.md#leo-query-transaction) - Query transaction information.
  - [`program`](./query.md#leo-query-program) - Query program source code and live mapping values.
  - [`stateroot`](./query.md#leo-query-stateroot) - Query the latest stateroot.
  - [`committee`](./query.md#leo-query-committee) - Query the current committee.
  - [`mempool`](./query.md#leo-query-mempool) - Query transactions and transmissions from the memory pool.
  - [`peers`](./query.md#leo-query-peers) - Query peer information.
- [`remove`](./remove.md) - Remove a dependency from the current project.
- [`run`](./run.md) - Run a program without producing a proof.
- [`test`](./test.md) - Run the test cases for a Leo project.
- [`update`](./update.md) - Update to the latest version of Leo.
- [`upgrade`](./upgrade.md) - Upgrade a deployed program on the Aleo network.
- [`synthesize`](./synthesize.md) - Generate proving and verifying keys for a program.
- [`fmt`](./fmt.md) - Format Leo source files. *(plugin)*
- [`plugins`](./plugins.md) - List installed CLI plugins.

## Universal Flags

These flags are available to use alongside all commands in the Leo CLI.

### `--help`

### `-h`

Prints available commands and flags.

### `--version`

### `-V`

Prints the currently installed version of Leo.

### `-q`

Suppresses the CLI output.

### `-d`

Prints out additional information for debugging if possible.

### `--path <PATH>`

Specifies the path to Leo program root folder. Defaults to `./`.

### `--home <HOME>`

Specifies the path to the `.aleo` program registry. This is where programs downloaded from the network will be cached. Defaults to `~/.aleo/registry`.

### `--json-output[=<PATH>]`

Saves structured JSON output to disk.

- **Default location**: `build/json-outputs/<command>.json`
- **Custom path**: `--json-output=my-results.json`

Supported commands: `deploy`, `upgrade`, `run`, `execute`, `test`, `query`, `synthesize`.

```bash title="Examples"
# Save to default location (build/json-outputs/run.json)
leo run --json-output main 1u32 2u32

# Save to custom path
leo execute main --json-output=my-results.json
```

### `--package <NAME>`

### `-p <NAME>`

Target a specific [workspace](../guides/workspaces.md) member by name. Matches the member's directory name, program name (e.g., `token.aleo`), or program name without the `.aleo` suffix. Only valid inside a workspace - errors if no `workspace.json` is found.
