---
id: cli_synthesize
title: ""
sidebar_label: Synthesize
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, synthesize, proving, verifying, keys, circuit"

# `leo synthesize`

This command is used to generate proving and verifying keys for all transitions in a local or remote Leo program, along with circuit metadata.

```bash
leo synthesize <PROGRAM_NAME> --save <SAVE_DIRECTORY>
```

Each output of this command includes:

- Number of public inputs
- Number of variables
- Number of constraints
- Non-zero entries in matrices
- Circuit ID
- Proving and verifying keys saved to disk

This enables better understanding of program size and key management.

## Flags

### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.

### `--endpoint <ENDPOINT>`

The endpoint to deploy to. Overrides any `ENDPOINT` environment variable set manually or in a `.env` file.

### `--network-retries <N>`

Number of times to retry a network request on transient transport failure, with exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the `NETWORK_RETRIES` environment variable. Defaults to `2`. HTTP errors (4xx/5xx) and broadcast calls are not retried.

### `--local`

### `-l`

Specifies that the keys should be generated for the local Leo project in the current working directory.

### `--skip <SKIP>`

### `-s <SKIP>`

Specifies to skip the key generation for any function names that contain the provided substrings

### `--save <SAVE_DIRECTORY>`

The directory to save the key files to. If the provided path does not exist, it will be created in your current working directory.
