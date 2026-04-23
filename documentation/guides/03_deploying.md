---
id: deploy
title: Deploying Your Programs
sidebar_label: Deploying
---

[general tags]: # "guides, deploy, deployment, program"

The `leo deploy` command is used for deploying a Leo program to a local devnet, Testnet, or Mainnet.
The `leo upgrade` command is used for upgrading an existing Leo program on the network.

## Getting Started

From the root of the Leo program directory, run the following command:

```bash
leo deploy --help
```

This will display the help message with all available options for the `leo deploy` command.

```bash
Deploy a program

Usage: leo deploy [OPTIONS]

Options:
      --base-fees <BASE_FEES>
          [UNUSED] Base fees in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to automatic calculation.
  -d
          Print additional information for debugging
      --priority-fees <PRIORITY_FEES>
          Priority fee in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to 0.
  -q
          Suppress CLI output
  -f, --fee-records <FEE_RECORDS>
          Records to pay for fees privately, delimited by '|', and used in order. The fees must either be valid plaintext, ciphertext, or `default`. Defaults to public fees.
      --print
          Print the transaction to stdout.
      --broadcast
          Broadcast the transaction to the network.
      --save <SAVE>
          Save the transaction to the provided directory.
      --private-key <PRIVATE_KEY>
          The private key to use for the deployment. Overrides the `PRIVATE_KEY` environment variable.
      --network <NETWORK>
          The network to deploy to. Overrides the `NETWORK` environment variable.
      --endpoint <ENDPOINT>
          The endpoint to deploy to. Overrides the `ENDPOINT` environment variable.
      --devnet
          Whether the network is a devnet. If not set, defaults to the `DEVNET` environment variable.
      --consensus-heights <CONSENSUS_HEIGHTS>
          Optional consensus heights to use. This should only be set if you are using a custom devnet.
  -y, --yes
          Don't ask for confirmation. DO NOT SET THIS FLAG UNLESS YOU KNOW WHAT YOU ARE DOING
      --consensus-version <CONSENSUS_VERSION>
          Consensus version to use. If one is not provided, the CLI will attempt to determine it from the latest block.
      --max-wait <MAX_WAIT>
          Seconds to wait for a block to appear when searching for a transaction. [default: 8]
      --blocks-to-check <BLOCKS_TO_CHECK>
          Number of blocks to look at when searching for a transaction. [default: 12]
      --skip <SKIP>...
          Skips deployment of any program that contains one of the given substrings.
      --offline
          Enables offline mode.
      --enable-ast-spans
          Enable spans in AST snapshots.
      --path <PATH>
          Path to Leo program root folder
      --enable-dce
          Enables dead code elimination in the compiler.
      --home <HOME>
          Path to aleo program registry
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
  -h, --help
          Print help
```

## A Quick Example

When you run `leo new`, it creates a new Leo project with default files and directories, including a `.env` file.
The defaults in the `.env` file are set to deploy on a testnet devnet using a local node API endpoint.
The defaults are set to deploy on a local devnet with the `testnet` configuration using a local node API endpoint.

To test the deployment, run a devnet via `leo devnet` (you'll need to configure it appropriately and run it in a separate terminal) and then run:

```bash
> leo deploy  --devnet --broadcast
       Leo
2 statements before dead code elimination.
       Leo     2 statements after dead code elimination.
       Leo     The program checksum is: '[96u8, 221u8, 32u8, 227u8, 44u8, 46u8, 93u8, 242u8, 17u8, 214u8, 17u8, 134u8, 170u8, 250u8, 59u8, 72u8, 48u8, 182u8, 210u8, 153u8, 135u8, 38u8, 214u8, 209u8, 12u8, 135u8, 252u8, 74u8, 132u8, 140u8, 123u8, 209u8]'.
       Leo ✅ Compiled 'helloworld.aleo' into Aleo instructions.

📢 Using the following consensus heights: 0,10,11,12,13,14,15,16,17
  To override, pass in `--consensus-heights` or override the environment variable `CONSENSUS_VERSION_HEIGHTS`.

Attempting to determine the consensus version from the latest block height at http://localhost:3030...

🛠️  Deployment Plan Summary
──────────────────────────────────────────────
🔧 Configuration:
  Private Key:        APrivateKey1zkp8CZNn3yeC...
  Address:            aleo1rhgdu77hgyqd3xjj8uc...
  Endpoint:           http://localhost:3030
  Network:            testnet
  Consensus Version:  9

📦 Deployment Tasks:
  • helloworld.aleo  │ priority fee: 0  │ fee record: no (public fee)

⚙️ Actions:
  • Transaction(s) will NOT be printed to the console.
  • Transaction(s) will NOT be saved to a file.
  • Transaction(s) will be broadcast to http://localhost:3030
──────────────────────────────────────────────

? Do you want to proceed with deployment? (y/n) › no
> leo deploy  --devnet --broadcast
       Leo
2 statements before dead code elimination.
       Leo     2 statements after dead code elimination.
       Leo     The program checksum is: '[96u8, 221u8, 32u8, 227u8, 44u8, 46u8, 93u8, 242u8, 17u8, 214u8, 17u8, 134u8, 170u8, 250u8, 59u8, 72u8, 48u8, 182u8, 210u8, 153u8, 135u8, 38u8, 214u8, 209u8, 12u8, 135u8, 252u8, 74u8, 132u8, 140u8, 123u8, 209u8]'.
       Leo ✅ Compiled 'helloworld.aleo' into Aleo instructions.

📢 Using the following consensus heights: 0,10,11,12,13,14,15,16,17
  To override, pass in `--consensus-heights` or override the environment variable `CONSENSUS_VERSION_HEIGHTS`.

Attempting to determine the consensus version from the latest block height at http://localhost:3030...

🛠️  Deployment Plan Summary
──────────────────────────────────────────────
🔧 Configuration:
  Private Key:        APrivateKey1zkp8CZNn3yeC...
  Address:            aleo1rhgdu77hgyqd3xjj8uc...
  Endpoint:           http://localhost:3030
  Network:            testnet
  Consensus Version:  9

📦 Deployment Tasks:
  • helloworld.aleo  │ priority fee: 0  │ fee record: no (public fee)

⚙️ Actions:
  • Transaction(s) will NOT be printed to the console.
  • Transaction(s) will NOT be saved to a file.
  • Transaction(s) will be broadcast to http://localhost:3030
──────────────────────────────────────────────

✔ Do you want to proceed with deployment? · yes


🔧 Your program 'helloworld.aleo' has the following constructor.
──────────────────────────────────────────────
constructor:
    assert.eq edition 0u16;
──────────────────────────────────────────────
Once it is deployed, it CANNOT be changed.

✔ Would you like to proceed? · yes

📦 Creating deployment transaction for 'helloworld.aleo'...


📊 Deployment Summary for helloworld.aleo
──────────────────────────────────────────────
  Total Variables:      16,995
  Total Constraints:    12,927
  Max Variables:        2,097,152
  Max Constraints:      2,097,152

💰 Cost Breakdown (credits)
  Transaction Storage:  0.879000
  Program Synthesis:    0.748050
  Namespace:            1.000000
  Constructor:          0.050000
  Priority Fee:         0.000000
  Total Fee:            2.677050
──────────────────────────────────────────────

📡 Broadcasting deployment for helloworld.aleo...
💰Your current public balance is 93749999.894112 credits.

✔ This transaction will cost you 2.67705 credits. Do you want to proceed? · yes

✉️ Broadcasted transaction with:
  - transaction ID: 'at1wnrupt8fvsck0jll4mu94e23uhmgwhjpftaazcephm8nu0yyvqrsm27apa'
  - fee ID: 'au1rqczm86uw6jwcx8ychgvy677axrsh2vjjz8kh0cmpaw87xyp7q9q20fpa7'
  - fee transaction ID: 'at12rgh8c58sc0npxusg065p6xrsrk60pmfg02t5047rf5dp096g5ysdftz4f'
    (use this to check for rejected transactions)

🔄 Searching up to 12 blocks to confirm transaction (this may take several seconds)...
Explored 2 blocks.
Transaction accepted.
✅ Deployment confirmed!
```

Leo will:

- Compile the program and generate the necessary AVM instructions.
- Tell you the program's checksum, which is a unique identifier for the program's code.
- Display a deployment summary, including the total number of variables and constraints.
- Ask for confirmation before proceeding with the deployment.
- Broadcast the deployment transaction to the specified network.
- Wait for the transaction to be confirmed and display the transaction ID.

## Upgrading a Program

If your program is already deployed, you can upgrade it using the `leo upgrade` command.
The upgrade will only work if your program is upgradable, meaning it has a constructor that allows for upgrades.
See the [Upgradability Guide](../guides/09_program_upgradability.md) for more details on how to make your program upgradable.

## Options and Environment Variables

The target network, the Private Key, and a node API endpoint need to be specified for a deployment or upgrade.
They can be set in one of the following ways, in order of precedence:

1. CLI options,
2. environment variables, or
3. `.env` file:

The options are selected in that order of precedence.
For example, if the `--network` option is specified, it will override the value in the `.env` file.
A `.env` file should be formatted as follows:

```bash
NETWORK=testnet
PRIVATE_KEY=APrivateKey1z...GPWH
ENDPOINT=https://api.explorer.provable.com/v1
```

If you are deploying to a local devnet, use the `--devnet` flag.
