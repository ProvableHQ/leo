---
id: cli_deploy
title: ""
sidebar_label: Deploy
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_deploy, deploy, deployment, transaction, program"

# `leo deploy`

:::note
This command requires having a funded account.
:::

To deploy the project in the current working directory, run the following command:

```bash
leo deploy # Defaults to using the private key, network, and endpoint in .env or environment variables
```

```bash title="sample output:"
       Leo     ... statements before dead code elimination.
       Leo     ... statements after dead code elimination.
       Leo ✅ Compiled '{PROGRAM_NAME}.aleo' into Aleo instructions.

📢 Using the following consensus heights: 0,2950000,4800000,6625000,6765000,7600000,8365000,9173000,9800000
  To override, pass in `--consensus-heights` or override the environment variable `CONSENSUS_VERSION_HEIGHTS`.

Attempting to determine the consensus version from the latest block height at {ENDPOINT}...

🛠️  Deployment Plan Summary
──────────────────────────────────────────────
🔧 Configuration:
  Private Key:        {PRIVATE_KEY}
  Address:            {ADDRESS}
  Endpoint:           {ENDPOINT}
  Network:            {NETWORK}
  Consensus Version:  {CONSENSUS_VERSION}

📦 Deployment Tasks:
  • {PROGRAM_NAME}.aleo  │ priority fee: 0  │ fee record: no (public fee)

⚙️ Actions:
  • Transaction(s) will NOT be printed to the console.
  • Transaction(s) will NOT be saved to a file.
  • Transaction(s) will be broadcast to {ENDPOINT}

🔧 Your program '{PROGRAM_NAME}.aleo' has the following constructor.
──────────────────────────────────────────────
constructor:
    ...
──────────────────────────────────────────────
Once it is deployed, it CANNOT be changed.

📦 Creating deployment transaction for '{PROGRAM_NAME}.aleo'...


📊 Deployment Summary for '{PROGRAM_NAME}.aleo'
──────────────────────────────────────────────
  Total Variables:      ...
  Total Constraints:    ...
  Max Variables:        2,097,152
  Max Constraints:      2,097,152

💰 Cost Breakdown (credits)
  Transaction Storage:  ...
  Program Synthesis:    ...
  Namespace:            ...
  Constructor:          ...
  Priority Fee:         ...
  Total Fee:            ...
──────────────────────────────────────────────
```

See the **[Deploying](./../guides/deploying.md)** guide for more details.

## JSON Output

Use `--json-output` to save structured JSON results to disk for programmatic use:

```bash
# Save to default location (build/json-outputs/deploy.json)
leo deploy --json-output -y

# Save to custom path
leo deploy --json-output=deployment_result.json -y
```

Example output (`build/json-outputs/deploy.json`):

```json
{
  "deployments": [
    {
      "program_id": "my_program.aleo",
      "transaction_id": "at1..."
    }
  ]
}
```

This is useful for scripting and CI/CD pipelines:

```bash
# Deploy and extract the transaction ID
jq '.deployments[0].transaction_id' build/json-outputs/deploy.json
```

## Workspace Behavior

When run inside a [workspace](../guides/workspaces.md):

- **From workspace root:** Builds and deploys all members in dependency order. Programs shared between members (e.g. a dependency that appears in multiple members) are deployed only once.
- **From a member directory:** Deploys only that member.
- **With `--package <NAME>`:** Deploys only the specified member.

```bash
# Deploy all workspace members
leo deploy --broadcast

# Deploy only the swap member
leo deploy --broadcast -p swap
```

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

### `--skip <SUBSTRING_0> <SUBSTRING_1> ...`

Skips deployment of any program that contains one of the given substrings, delimited by a space.

### `--rename <NAME>`

Deploys the project's program under a different on-chain name, without editing your source. The program is recompiled so its on-chain identity becomes `<NAME>`, producing a **genuinely distinct deployment** — not an alias of the original name. The argument may be given with or without the `.aleo` suffix (`--rename token` and `--rename token.aleo` are equivalent).

```bash
# The package declares `original_prog.aleo`, but deploy it as `renamed_prog.aleo`.
leo deploy --broadcast --rename renamed_prog
```

The deployment plan and summary reflect the new name, and the on-chain program is the renamed one:

```bash title="sample output (abridged):"
       Leo ✅ Compiled 'renamed_prog.aleo' into Aleo instructions.
...
📦 Deployment Tasks:
  • renamed_prog.aleo  │ priority fee: 0  │ fee record: no (public fee)
...
✅ Deployment confirmed!
```

:::note
Only the project's own (primary) program is renamed. Programs that `import` the original name are **not** redirected to the renamed copy, and your local source and `program.json` are left unchanged.
:::

`--rename` is rejected, before any network interaction, when:

- the name is not a valid Aleo program name;
- the target collides with another program or dependency in the package (build artifacts are keyed by name, so this would be ambiguous);
- the target is the program's current name (a no-op rename);
- it is combined with `--build-tests` (tests keep their original names and would dangle against the renamed primary);
- deploying multiple [workspace](../guides/workspaces.md) members at once (deploy a single program instead).

**Executing a renamed program.** Because your local package keeps its original identity, run the renamed program by its **fully qualified** on-chain name so it resolves to the deployed copy on the network:

```bash
leo execute --broadcast renamed_prog.aleo::main 1u32 2u32
```

Running `leo execute main ...` (unqualified) from the same directory would instead target your local program under its original name, which was never deployed.

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
  Skips deployment of any program that contains one of the given substrings.
--rename <NAME>
  Deploy the program under a different name, producing a genuinely distinct on-chain deployment. Programs importing the original name are not redirected to the renamed copy.
--build-tests
    Build tests along with the main program and dependencies.
--no-cache
    Don't use the dependency cache.
--no-local
    Don't use the local source code.
```
