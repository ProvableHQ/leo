---
id: cli_upgrade
title: ""
sidebar_label: Upgrade
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_upgrade, upgrade, program"

# `leo upgrade`

Upgrades a program that is already deployed on the network.

See the **[Upgrading Programs](./../guides/09_program_upgradability.md)** guide for more details.

## Flags

### `--private-key <PRIVATE_KEY>`

Specifies the private key to use for the deployment. Overrides any `$PRIVATE_KEY` environment variable set manually or in a `.env` file.

### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

**Common Endpoints:**

<!-- markdown-link-check-disable -->

|    Network     |                Endpoint                |
| :------------: | :------------------------------------: |
| Devnet (local) |        <https://localhost:3030>        |
|    Testnet     | <https://api.explorer.provable.com/v1> |
|    Mainnet     | <https://api.explorer.provable.com/v1> |

<!-- markdown-link-check-enable -->

### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

### `--devnet`

Specifies whether the network being deployed to is a devnet. If not set, defaults to the `DEVNET` environment variable.

:::info
This flag requires a devnet to be running locally. See INSERT DEVNET GUIDE HERE for more information
:::

### `-print`

Prints the transaction to the terminal/stdout in JSON format.

### `-broadcast`

Broadcasts the transaction to the network upon successful execution. Without passing this flag, the transaction will just be generated locally.

### `--save <SAVE>`

Saves the transaction to the directory located at the `<SAVE>` path.

### `--yes`

### `-y`

The CLI will ask for manual confirmation on several steps throughout the deployment process. Setting this flag automatically agrees to all confirmations.

:::warning
Do not use this feature unless you know what you are doing!
:::

### `--priority-fees <PRIORITY_FEES>`

Specifies the priority fee for the deployment transaction(s) delimited by `|` and used in order. The fees are in microcredits and must either be valid `u64` or `default`. Defaults to 0.

:::tip
1 Credit == 1,000,000 Microcredits
:::

### `--fee-records <FEE_RECORDS>`

### `-f <FEE_RECORDS>`

Specifies the record(s) to pay for fees privately, delimited by `|` and used in order. The fees must either be valid plaintext, ciphertext, or `default`. If not specified, then transaction fees will be public.

### `--consensus-heights <CONSENSUS_HEIGHTS>`

Specifies the consensus heights to use, delimited by `,`. This should only be set if you are using a custom devnet.

The following will enable Consensus_V0 at block 0, Consensus_V1 at block 1, etc.:

```bash
--consensus-heights 0,1,2,3....
```

### `--consensus-version <CONSENSUS_VERSION>`

Specifies the consensus version to use. If one is not provided, the CLI will attempt to determine it from the latest block.

### `--max-wait <MAX_WAIT>`

Specifies the number of seconds to wait for a block to appear when searching for a transaction. Defaults to 8 seconds.

### `--blocks-to-check <BLOCKS_TO_CHECK>`

Specifies the number of blocks to look at when searching for a transaction. Defaults to 12 blocks

```text
Options:
--base-fees <BASE_FEES>
  [UNUSED] Base fees in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to automatic calculation.
--skip <SKIP>...
  Skips the upgrade of any program that contains one of the given substrings.
--offline
    Enables offline mode.
--enable-ast-spans
    Enable spans in AST snapshots.
--enable-dce
    Enables dead code elimination in the compiler.
--conditional-block-max-depth <CONDITIONAL_BLOCK_MAX_DEPTH>
    Max depth to type check nested conditionals. [default: 10]
--disable-conditional-branch-type-checking
    Disable type checking of nested conditional branches in finalize scope.
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
```
