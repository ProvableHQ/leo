---
id: cli_query
title: ""
sidebar_label: Query
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_query, query, block, transaction, program, stateroot, committee, mempool, peers, snarkOS, mapping"

# `leo query`

The `leo query` command is used to get data from a network supporting the canonical `snarkOS` endpoints.

# Subcommands

- [`block`](#leo-query-block) - Query block information.
- [`transaction`](#leo-query-transaction) - Query transaction information.
- [`program`](#leo-query-program) - Query program source code and live mapping values.
- [`stateroot`](#leo-query-stateroot) - Query the latest stateroot.
- [`committee`](#leo-query-committee) - Query the current committee.
- [`mempool`](#leo-query-mempool) - Query transactions and transmissions from the memory pool.
- [`peers`](#leo-query-peers) - Query peer information.

&nbsp;

---

## `leo query block`

To fetch blocks from a given network, run the following command

```bash
leo query <ID>
```

where `<ID>` is either a specific block height or block hash. The block will be returned in JSON format.

For example, you can fetch the Mainnet genesis block by running either of the following commands:

```bash
leo query block 0 --network mainnet --endpoint https://api.explorer.provable.com/v1
```

```bash
leo query block ab1sm6kyqle2ftg4z8gegafqrjy0jwjhzu6fmy73726dgszrtxhxvfqha0eee --network mainnet --endpoint https://api.explorer.provable.com/v1
```

### Flags

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

#### `--latest`

#### `-l`

Queries the latest block.

```bash title="Example:"
leo query block --latest
```

#### `--latest-hash`

Queries the hash of the latest block.

```bash title="Example:"
leo query block --latest-hash
```

#### `--latest-height`

Queries the height of the latest block

```bash title="Example:"
leo query block --latest-height
```

#### `--range <START_HEIGHT> <END_HEIGHT>`

#### `-r <START_HEIGHT> <END_HEIGHT>`

Queries up to 50 consecutive blocks.

```bash title="Example:"
leo query block --range <START_HEIGHT> <END_HEIGHT>
```

#### `--transactions`

#### `-t`

Queries all transactions at the specified block height

```bash title="Example:"
leo query block <BLOCK_HEIGHT> --transactions
```

#### `--to-height`

Queries the block height corresponding to a hash value

```bash title="Example:"
leo query block <BLOCK_HASH> --to-height
```

---

## `leo query transaction`

To fetch a specific transaction from a given network, run the following command:

```bash
leo query transaction <ID>
```

where `<ID>` is the ID of the transaction. The transaction will be returned in JSON format.

### Flags

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

#### `--confirmed`

#### `-c`

Returns information (including details about onchain execution) about the queried transaction if it was confirmed.

#### `--unconfirmed`

#### `-u`

Queries the original (unconfirmed) transaction.

#### `--from-io <INPUT_OR_OUTPUT_ID>`

Get the ID of the transaction that an input or output ID occurred in.

```bash title="Example:"
leo query transaction --from-io <INPUT_OR_OUTPUT_ID>
```

#### `--from-transition <TRANSITION_ID>`

Get the ID of the transaction containing the specified transition.

```bash title="Example:"
leo query transaction --from-transition <TRANSITION_ID>
```

#### `--from-program <PROGRAM_NAME>`

Get the ID of the transaction that the specified program was deployed in.

```bash title="Example:"
leo query transaction --from-program <PROGRAM_NAME>
```

---

## `leo query program`

To fetch a specific program from a given network, run the following command:

```bash
leo query program <PROGRAM_NAME>
```

You can also use this to query specific mappings and mapping values from a given program. For example, if you wanted to query your public balance of Aleo credits:

```bash
leo query program credits.aleo --mapping-value account <YOUR_ADDRESS>
```

### Flags

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

#### `--mappings`

Lists all mappings defined in the latest deployed edition of the program.

#### `--mapping-value <MAPPING> <KEY>`

Get the value corresponding to the specified mapping and key. Will return `null` if key is not present in mapping.

#### `--edition <EDITION>`

An optional edition to query for when fetching the program source. If not specified, the latest edition will be used.

The edition of the program is set to `0` upon initial deployment and is incremented by `1` each time the program is upgraded. See the **[Upgrading Programs](./../guides/09_program_upgradability.md)** guide for more details.

---

## `leo query stateroot`

This command queries the latest stateroot of a given network.

### Flags

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

---

## `leo query committee`

This command queries the current validator committee for a given network.

### Flags

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

---

## `leo query mempool`

This command queries transactions and transmissions from the memory pool for a node on a given network.

:::note
This command can only be run with a custom endpoint. Using the official Provable API endpoint will fail.
:::

To query transactions:

```bash
leo query mempool --transactions
```

To query transmissions:

```bash
leo query mempool --transmissions
```

### Flags

#### `--transactions`

Queries the transactions in the memory pool.

#### `--transmissions`

Queries the transactions in the memory pool.

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

---

## `leo query peers`

This command queries peer information for a node on a given network.

:::note
This command can only be run with a custom endpoint. Using the official Provable API endpoint will fail.
:::

### Flags

#### `--metrics`

#### `-m`

Queries all peer metrics

#### `--count`

#### `-c`

Queries the count of all participating peers

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

#### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

#### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.
