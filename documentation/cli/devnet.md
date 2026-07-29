---
id: cli_devnet
title: ""
sidebar_label: Devnet
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_devnet, devnet local_devnet, snarkos"

# `leo devnet`

To initialize a local devnet, run the following command:

```bash
leo devnet --snarkos <SNARKOS>
```

where `<SNARKOS>` should be the path to an installed binary of snarkOS.

If snarkOS is not installed, pass `--install`. This option installs the binary at the specified path.

<!-- markdown-link-check-disable -->

:::info
The default ENDPOINT for a local devnet is `http://localhost:3030`
:::

<!-- markdown-link-check-enable -->

## Flags

### `--snarkos <SNARKOS>`

Specifies the path to the installed snarkOS binary.

:::info
This flag is required!
:::

### `--snarkos-features <FEATURES>`

Specifies which features of snarkOS to use (for example `test_network`)

### `--install`

Installs (or reinstalls) snarkOS at the provided `--snarkos` path with the given `--snarkos-features`.

<!-- markdown-link-check-disable -->

### `--snarkos-version <SNARKOS_VERSION>`

Specifies which version of snarkOS to use or install. Defaults to latest version on [crates.io](https://crates.io/crates/snarkos)

<!-- markdown-link-check-enable -->

### `--consensus-heights <CONSENSUS_HEIGHTS>`

Optional blocks heights to use for each successive consensus upgrade. Must have `--snarkos-features test_network` enabled as well.

The following settings enable each consensus version at its corresponding block:

```bash
--consensus-heights 0,1,2,3....
```

### `--storage <STORAGE>`

Root directory path for snarkOS ledgers and logs. Defaults to `./`

### `--clear-storage`

Clear existing snarkOS ledgers before starting the devnet

### `--network <NETWORK_ID>`

Specifies what the network ID of the devnet will be.

| ID  |      Network      |
| :-: | :---------------: |
|  0  |      Mainnet      |
|  1  | Testnet (default) |
|  2  |      Canary       |

### `--tmux`

Run devnet nodes in tmux (only available on Unix-based systems)

### `--num-validators <NUM_VALIDATORS>`

Number of validators to use in snarkOS. Defaults to 4.

### `--num-clients <NUM_CLIENTS>`

Number of clients to use in snarkOS. Defaults to 2.

### `--verbosity <VERBOSITY>`

Specifies the verbosity of snarkOS (0-4). Defaults to 1.

### `--yes`

### `-y`

Skips confirmation prompts and proceeds with the devnet startup.

### `--rest-port <REST_PORT>`

Base REST port. Each node uses `base + dev_index`.

### `--node-port <NODE_PORT>`

Base node port. Each node uses `base + dev_index`.

### `--bft-port <BFT_PORT>`

Base BFT port. Each node uses `base + dev_index`.

### `--metrics-port <METRICS_PORT>`

Base metrics port. Each validator uses `base + dev_index`.

### `--clean-only`

Only cleans devnet storage (ledgers, node data, logs) without starting the devnet.
