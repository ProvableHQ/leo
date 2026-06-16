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

Unlike [`leo build`](./build.md), which automatically writes `build/abi.json` for the current package's source, `leo abi` works on any standalone `.aleo` file. This is useful for:

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

By default `leo abi` resolves imported `.aleo` files relative to the input file: for the per-unit build layout (`<root>/<unit>/<unit>.aleo`) it uses `<root>`, otherwise it falls back to a sibling `imports/` directory. Use `--imports-dir <DIR>` to point at a different location. Network builtins such as `credits.aleo` do not need to be present on disk.

The output is the same JSON shape that `leo build` produces in `build/abi.json`. See the [ABI Generation guide](../guides/abi.md) for the format reference and type-lowering specification.

## Checking compatibility with an ABI

Pass `--satisfies <FILE>` to check whether the input program's public interface is a superset of an interface standard, instead of printing its ABI. The standard is a JSON file containing an ABI (such as one produced by `leo abi` or `leo build`):

```bash
leo abi token.aleo --satisfies token_standard.abi.json
```

The program *satisfies* the standard when it declares every function, view, mapping, storage variable, record, and struct the standard requires, with matching signatures. The program may declare additional items beyond the standard. Type references are compared relative to each side's owning program, so a standard's reference to one of its own types matches the program's reference to the corresponding type even though the two programs have different names.

On success a one-line confirmation is printed; otherwise the unsatisfied items are listed and the command exits non-zero:

```text
`token.aleo` does not satisfy `token_standard.aleo`:
  - missing function `burn`
  - mapping `balances` differs
```

Pass `--output <FILE>` together with `--satisfies` to write the report as JSON (`{ "satisfied": <bool>, "problems": [...] }`) to that file instead of printing a human-readable summary. The command still exits non-zero when the program does not satisfy the standard.

## Flags

```text
<FILE>
    Path to the .aleo bytecode file. The file must have a `.aleo` extension and contain
    valid Aleo instructions.
--network <NETWORK>, -n <NETWORK>
    Network used to parse the bytecode (`mainnet`, `testnet`, or `canary`). Defaults to
    `testnet`. Network choice affects how some literals are interpreted.
--output <PATH>, -o <PATH>
    Without `--satisfies`: output directory. Writes `<PATH>/<program>.abi.json` for the
    input and for each declared dependency. Created if missing; existing files are
    overwritten. When omitted, every ABI is printed to stdout, separated by
    `=== <name> ===` headers.
    With `--satisfies`: the file path to write the JSON compatibility report to instead
    of printing a human-readable summary.
--imports-dir <DIR>
    Directory containing the program's `.aleo` imports. Defaults to a sibling
    `imports/` directory next to the input file if one exists. Network builtins
    (e.g. `credits.aleo`) do not need to be present here.
--satisfies <FILE>
    Check whether the input program satisfies an interface standard (a JSON file
    containing an ABI), instead of printing its ABI. Exits non-zero when it does not.
```
