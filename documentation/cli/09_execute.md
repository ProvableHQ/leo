---
id: cli_execute
title: ""
sidebar_label: Execute
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_execute, execute, execution, transaction, transaction_status"

# `leo execute`

You can execute an entry function by using the `leo execute` command. This differs from the `leo run` command in that a transaction and proof of execution are produced, and optionally can be broadcasted to the network.

To execute a Leo entry function with inputs from the command line, run the following command:

```bash
leo execute <FUNCTION_NAME> <INPUTS>
```

where `<FUNCTION_NAME>` is the name of the entry `fn` to execute and `<INPUTS>` is a list of inputs to the program separated by spaces.

:::note
This command requires having a funded account.
:::

Under the hood, this command synthesizes the program circuit and generates proving and verifying keys.

```bash title="sample output:"
       Leo     ... statements before dead code elimination.
       Leo     ... statements after dead code elimination.
       Leo     The program checksum is: '[...]'.
       Leo ✅ Compiled '{PROGRAM_NAME}.aleo' into Aleo instructions.

📢 Using the following consensus heights: 0,2950000,4800000,6625000,6765000,7600000,8365000,9173000,9800000
  To override, pass in `--consensus-heights` or override the environment variable `CONSENSUS_VERSION_HEIGHTS`.

Attempting to determine the consensus version from the latest block height at {ENDPOINT}...

🚀 Execution Plan Summary
──────────────────────────────────────────────
🔧 Configuration:
  Private Key:        {PRIVATE_KEY}
  Address:            {ADDRESS}
  Endpoint:           {ENDPOINT}
  Network:            {NETWORK}
  Consensus Version:  {CONSENSUS_VERSION}

🎯 Execution Target:
  Program:        {PROGRAM_NAME}.aleo
  Function:       {FUNCTION_NAME}
  Source:         remote

💸 Fee Info:
  Priority Fee:   {PRIORITY_FEE} μcredits
  Fee Record:     no (public fee) | {FEE RECORD}

⚙️ Actions:
  - Program and its dependencies will be downloaded from the network.
  - Transaction will NOT be printed to the console.
  - Transaction will NOT be saved to a file.
  - Transaction will NOT be broadcast to the network.

📊 Execution Summary for {PROGRAM_NAME}.aleo
──────────────────────────────────────────────
💰 Cost Breakdown (credits)
  Transaction Storage:  ...
  On‑chain Execution:   ...
  Priority Fee:         ...
  Total Fee:            ...
──────────────────────────────────────────────

➡️  Outputs

 • {OUTPUT_0}
 • {OUTPUT_1}
 ...
```

See the **[Executing](./../guides/04_executing.md)** guide for more details.

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

### `--with <WITH>...`

Specifies one or more additional programs to load into the VM at runtime. Each entry can be:

- A path to a local `.aleo` bytecode file (e.g. `./extra_prog.aleo`)
- The name of a remote program to fetch from the network endpoint (along with its transitive dependencies)

Multiple programs can be provided as a comma-separated list. When specifying local `.aleo` files, they must be listed in topological order — i.e. a dependency must appear before any program that depends on it.

This is useful when a program has dynamic dependencies that are not declared in `program.json` and therefore cannot be resolved at build time.

```bash
leo execute <FUNCTION_NAME> <INPUTS> --with ./extra_prog.aleo
leo execute <FUNCTION_NAME> <INPUTS> --with program1.aleo,program2.aleo
```

:::note
When loading remote programs, an `--endpoint` must be set.
:::

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
