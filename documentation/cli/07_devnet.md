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

If you don't have snarkOS installed, you can do so by passing the `--install` flag, which will install the binary at the path specified above.

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

Specifies which features of snarkOS to use (e.g. `test_network`)

### `--install`

Installs (or reinstalls) snarkOS at the provided `--snarkos` path with the given `--snarkos-features`.

<!-- markdown-link-check-disable -->

### `--snarkos-version <SNARKOS_VERSION>`

Specifies which version of snarkOS to use or install. Defaults to latest version on [crates.io](https://crates.io/crates/snarkos)

<!-- markdown-link-check-enable -->

### `--consensus-heights <CONSENSUS_HEIGHTS>`

Optional blocks heights to use for each successive consensus upgrade. Must have `--snarkos-features test_network` enabled as well.

The following will enable Consensus_V0 at block 0, Consensus_V1 at block 1, etc.:

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
