# leo-package

[![Crates.io](https://img.shields.io/crates/v/leo-package.svg?color=neon)](https://crates.io/crates/leo-package)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

## Trusted network programs

Network programs require trusted checksum pins in `leo.lock`. The `program.json` format does not change. A pin specifies a program ID, network, edition, and checksum. Leo checks the pin before it accepts cached or downloaded bytecode.

The lock format is JSON. Version 2 adds a `network` array to the existing `git` array. Each network entry has these fields:

| Field | Value |
| --- | --- |
| `name` | The full program ID, for example `token.aleo`. |
| `network` | `mainnet`, `testnet`, or `canary`. |
| `edition` | The trusted edition number, from 0 to 65535. |
| `checksum` | An array of 32 integers, each from 0 to 255. |

Use one entry for each program and network. Include the remote target and every transitive import. The checksum is the SHA3-256 hash of the canonical program text from snarkVM `Program::to_checksum()`. A hash of the raw file can differ because of comments or spacing.

To obtain a checksum from reviewed Leo source, run this command in that source package:

```sh
leo build --checksums --json-output=checksums.json
```

Copy the `program_checksum` byte array from the JSON output into the pin's `checksum` field. Obtain the program ID, network, and edition from an independent trusted source. For existing Aleo bytecode, use the snarkVM `Program::to_checksum()` method after you review the code. Do not create a trusted pin from an unchecked endpoint response or an old cache entry.

Leo reads `leo.lock` from the workspace root, or from the package directory when there is no workspace. For a standalone `.aleo` file, Leo reads the lock beside that file. To use a separate trust file, pass its path:

```sh
leo execute token.aleo/transfer <inputs> --network-lock /path/to/trusted.lock
```

The global `--network-lock` option also applies to builds, remote `--with` programs, and upgrades. Leo does not write to a separate trust file. Git resolution still updates the package or workspace lock. Use a trust file that you control; a dependency's own lock does not supply trusted pins.

An omitted manifest edition uses the pinned edition. An explicit edition must match the pin. Leo does not select the highest cached edition, request the latest edition, or fall back to an unversioned program URL. To approve an upgrade, replace the pin with the independently verified edition and checksum. For `leo upgrade`, pin the old deployed program and its imports so Leo can check the upgrade against trusted code.

Missing pins, checksum mismatches, duplicate pins, and invalid lock files stop the operation. Neither `--yes` nor `--no-cache` bypasses verification. A valid version 1 Git lock remains readable, but network dependencies require pins. Builds preserve network pins, including pins not used by the current package. The built-in `credits.aleo` program comes from snarkVM at edition zero and needs no external pin.

Pins authenticate program content. They do not authenticate other node responses, such as balances or block heights.
