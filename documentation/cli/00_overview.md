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

- [`account`](./01_account.md) - Create a new Aleo account, sign and verify messages.
  - [`new`](./01_account.md#leo-account-new) - Generates a new Aleo account.
  - [`import`](./01_account.md#leo-account-import) - Derive and Aleo account from a private key.
  - [`sign`](./01_account.md#leo-account-sign) - Sign a message using your Aleo private key.
  - [`verify`](./01_account.md#leo-account-verify) - Verify a message and signature from an Aleo address.
- [`add`](./02_add.md) - Add a new onchain or local dependency to the current project.
- [`build`](./03_build.md) - Compile the current project.
- [`clean`](./04_clean.md) - Clean the build and output artifacts.
- [`debug`](./05_debug.md) - Run the interactive debugger in the current package.
- [`deploy`](./06_deploy.md) - Deploy a program to the Aleo network.
- [`devnet`](./07_devnet.md) - Initialize a local devnet.
- [`execute`](./08_execute.md) - Execute a program and produce a transaction containing a proof.
- [`new`](./09_new.md) - Create a new Leo project in a new directory.
- [`query`](./10_query.md) - Query live data and state from the Aleo network.
  - [`block`](./10_query.md#leo-query-block) - Query block information.
  - [`transaction`](./10_query.md#leo-query-transaction) - Query transaction information.
  - [`program`](./10_query.md#leo-query-program) - Query program source code and live mapping values.
  - [`stateroot`](./10_query.md#leo-query-stateroot) - Query the latest stateroot.
  - [`committee`](./10_query.md#leo-query-committee) - Query the current committee.
  - [`mempool`](./10_query.md#leo-query-mempool) - Query transactions and transmissions from the memory pool.
  - [`peers`](./10_query.md#leo-query-peers) - Query peer information.
- [`remove`](./11_remove.md) - Remove a dependency from the current project.
- [`run`](./12_run.md) - Run a program without producing a proof.
- [`test`](./13_test.md) - Run the test cases for a Leo project.
- [`update`](./14_update.md) - Update to the latest version of Leo.
- [`upgrade`](./15_upgrade.md) - Upgrade a deployed program on the Aleo network.
- [`synthesize`](./16_synthesize.md) - Generate proving and verifying keys for a program.
- [`fmt`](./17_fmt.md) - Format Leo source files. *(plugin)*
- [`plugins`](./18_plugins.md) - List installed CLI plugins.

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
