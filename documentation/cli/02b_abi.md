---
id: cli_abi
title: ""
sidebar_label: ABI
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_abi, abi, bytecode, disassemble"

# `leo abi`

Generate an ABI JSON document from a compiled `.aleo` bytecode file:

```bash
leo abi <FILE>
```

Unlike [`leo build`](./03_build.md), which automatically writes `build/abi.json` for the current package's source, `leo abi` works on any standalone `.aleo` file. This is useful for:

- inspecting the public interface of a deployed program when you do not have its source,
- tooling pipelines that consume raw `.aleo` bytecode and need the ABI separately,
- comparing the ABI generated from local sources against the on-chain bytecode.

By default the ABI is printed to stdout. If the program declares imports, the main ABI is printed first, followed by each dependency's ABI under a `=== <name> ===` header so a single invocation produces the full set:

```text
{ ... main program ABI ... }

=== dep_one.aleo ===
{ ... dep_one ABI ... }

=== dep_two.aleo ===
{ ... dep_two ABI ... }
```

Pass `--output <DIR>` to write each ABI as a separate file under that directory instead. The directory is created if missing; existing files are overwritten.

```bash
leo abi credits.aleo --output ./abis
```

```bash title="console output:"
       Leo     ABI written to './abis/credits.aleo.abi.json'.
```

For a program with imports, one `<DIR>/<program>.abi.json` file is written for the main program and one for each dependency.

By default `leo abi` resolves imported `.aleo` files from a sibling `imports/` directory next to the input file. Use `--imports-dir <DIR>` to point at a different location. Network builtins such as `credits.aleo` do not need to be present on disk.

The output is the same JSON shape that `leo build` produces in `build/abi.json`. See the [ABI Generation guide](../guides/11_abi.md) for the format reference and type-lowering specification.

## Flags

```text
<FILE>
    Path to the .aleo bytecode file. The file must have a `.aleo` extension and contain
    valid Aleo instructions.
--network <NETWORK>, -n <NETWORK>
    Network used to parse the bytecode (`mainnet`, `testnet`, or `canary`). Defaults to
    `testnet`. Network choice affects how some literals are interpreted.
--output <DIR>, -o <DIR>
    Output directory. Writes `<DIR>/<program>.abi.json` for the input and for each
    declared dependency. Created if missing; existing files are overwritten. When
    omitted, every ABI is printed to stdout, separated by `=== <name> ===` headers.
--imports-dir <DIR>
    Directory containing the program's `.aleo` imports. Defaults to a sibling
    `imports/` directory next to the input file if one exists. Network builtins
    (e.g. `credits.aleo`) do not need to be present here.
```
