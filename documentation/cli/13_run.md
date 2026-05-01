---
id: cli_run
title: ""
sidebar_label: Run
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_run, run"

# `leo run`

You can run an entry function by using the `leo run` command. This will simply run the specified entry function with the provided inputs and show what the produced output will be. It will NOT generate the zero-knowledge proof of execution or the transaction, and nothing will be run onchain. For that, please see the [`leo execute`](09_execute.md) command.

To run a Leo entry function with inputs from the command line, run the following command:

```bash
leo run <FUNCTION_NAME> <INPUTS>
```

where `<FUNCTION_NAME>` is the name of the entry `fn` to run and `<INPUTS>` is a list of inputs to the program separated by spaces.

This command does not synthesize the program circuit or generate proving and verifying keys.

```bash title="sample output:"
       Leo     ... statements before dead code elimination.
       Leo     ... statements after dead code elimination.
       Leo     The program checksum is: '[...]'.
       Leo ✅ Compiled '{PROGRAM_NAME}.aleo' into Aleo instructions.

⛓  Constraints

 •  '{PROGRAM_NAME}.aleo::{FUNCTION_NAME}' - ... constraints (called 1 time)

➡️  Outputs

 • {OUTPUT_0}
 • {OUTPUT_1}
 ...
```

If one or more of your inputs are negatives, and consequently begin with a `-`,
you may separate the inputs with a `--` so that the command line parser
won't attempt to parse them as options:

```bash
leo run <FUNCTION_NAME> -- <INPUT_0> -- <INPUT_1> ...
```

## Flags

```text
--with <WITH>...
    Comma-separated list of additional programs to load into the VM at runtime.
    Each entry can be a path to a local `.aleo` bytecode file or the name of a
    remote program to fetch (with its transitive dependencies) from the network
    endpoint. Local files must be listed in topological order (dependencies
    before the programs that depend on them).
--offline
    Enables offline mode.
--enable-ast-spans
    Enable spans in AST snapshots.
--enable-dce
    Enables dead code elimination in the compiler.
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
--private-key <PRIVATE_KEY>
    The private key to use for the deployment. Overrides the `PRIVATE_KEY` environment variable.
--network <NETWORK>
    The network to deploy to. Overrides the `NETWORK` environment variable.
--endpoint <ENDPOINT>
    The endpoint to deploy to. Overrides the `ENDPOINT` environment variable.
--network-retries <N>
    Number of times to retry a network request on transient transport failure, with
    exponential backoff (1 s, 2 s, 4 s, … capped at 64 s). Overrides the
    NETWORK_RETRIES environment variable. Defaults to 2. HTTP errors and broadcast
    calls are not retried.
--devnet
    Whether the network is a devnet. If not set, defaults to the `DEVNET` environment variable.
--consensus-heights <CONSENSUS_HEIGHTS>
    Optional consensus heights to use. This should only be set if you are using a custom devnet.
```
