# leo-package

[![Crates.io](https://img.shields.io/crates/v/leo-package.svg?color=neon)](https://crates.io/crates/leo-package)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

## Network dependency locking

Leo creates network entries in `leo.lock` automatically when it resolves project dependencies. Each entry records the program ID, network, edition, and checksum. This includes transitive network imports. The `program.json` format does not change, and users do not need to enter checksums.

The first download trusts the configured endpoint. Leo checks the program ID and size, then records the checksum of the canonical Aleo program. An old cache entry cannot supply the first checksum. This protects subsequent builds against changed bytecode, but it does not authenticate the first response from a malicious endpoint.

Later builds use the locked edition. Leo checks cached and downloaded bytecode against its checksum. A checksum mismatch stops the operation without replacing the lock or cache. `--no-cache` downloads the locked edition again; it does not select a new edition or replace its checksum.

To update a dependency, use `leo add` with the required `--edition`. A different edition is fetched from the endpoint and gets a new checksum. Re-adding a dependency without an edition selects the latest edition. If the edition is unchanged, the existing checksum still applies.

Leo reads and writes the lock at the workspace root, or beside `program.json` for a standalone project. It preserves network entries for other workspace members, development dependencies, and networks. A build writes the lock only after all dependencies are resolved and the dependency graph is valid. The file replacement is atomic.

Standalone `.aleo` files can resolve network imports without manual pins. They do not create a project lock. Remote command targets can also load without a project; their first response has the same endpoint trust limit. Upgrade checks must inspect the current deployed edition.

The lock format is JSON. Version 2 adds a `network` array to the existing `git` array. A valid version 1 Git lock remains readable. Each network entry has these fields:

| Field | Value |
| --- | --- |
| `name` | The full program ID, for example `token.aleo`. |
| `network` | `mainnet`, `testnet`, or `canary`. |
| `edition` | The resolved edition number, from 0 to 65535. |
| `checksum` | An array of 32 integers, each from 0 to 255. |

The checksum is the SHA3-256 hash from snarkVM `Program::to_checksum()`. Comments and spacing do not change it. Duplicate entries and invalid lock files stop resolution. The built-in `credits.aleo` program comes from snarkVM at edition zero and needs no network entry.

Checksums protect the recorded program content. They do not authenticate other node responses, such as balances or block heights. Keep `leo.lock` under version control with the project.
