---
id: test
title: Testing, Testing, 123
sidebar_label: Testing
---

[general tags]: # "guides, tests, testing, unit_testing, integration_testing, devnode, devnet, testnet"

Once deployed, an application lives on the ledger forever. Consequently, it's important to consider every edge case and rigorously test your code. There are a number of tools and techniques you can use.

- [**Unit and Integration Testing**](#unit-and-integration-testing) - Validate Leo program logic through test cases.
- [**Running a Devnode**](#running-a-devnode) - Deploy and execute against a lightweight local node.
- [**Running a Devnet**](#running-a-devnet) - Deploy and execute on a full local devnet backed by snarkOS.
- [**Deploying/Executing on Testnet**](#deployingexecuting-on-testnet) - Deploy and execute on the Aleo Testnet.
- [**Other Tools**](#other-tools) - Tools and methodologies developed by the open-source Aleo community.

## Choosing a Testing Strategy

| Tool | Best for | Notes |
| ---- | -------- | ----- |
| `leo test` | Logic, record fields, mappings | Fast; no network round-trip; no fee credits required |
| `leo devnode` | End-to-end deploy/execute cycles, multi-program interaction | No snarkOS required; proof generation can be skipped |
| `leo devnet` | Full consensus scenarios, multi-validator behaviour | Requires a snarkOS installation; heavier setup |
| Testnet | Final validation before mainnet | Real credits required; use the [Aleo faucet](https://faucet.aleo.org/) |

Start with `leo test` for pure logic. Reach for `leo devnode` when you need to test deploy/execute cycles against a live node. Use `leo devnet` when your scenario requires full consensus behaviour. Promote to Testnet for final validation before mainnet.

## Unit and Integration Testing

The Leo testing framework enables developers to validate their Leo program logic by writing unit and integration tests. Tests are written in Leo and are located in a `tests/` subdirectory of the main Leo project directory.

```bash
example_program
├── build
│   ├── imports
│   │   └── test_example_program.aleo
│   ├── main.aleo
│   └── program.json
├── outputs
├── src
│   └── main.leo
├── tests
│   └── test_example_program.leo
└── program.json
```

The test file is a Leo program that imports the program in `main.leo`. The test functions will all be annotated with `@test` above the function declaration.

This tutorial will use an example program which can be found in the [example's repository](https://github.com/ProvableHQ/leo-examples/tree/main/example_with_test).

:::info
Developers can add multiple `leo` files to the test directory but must ensure that the name of the test file matches the program name within that test file. For example, if the name of the test file is `test_example_program.leo`, the program name in that file must be `test_example_program.aleo`.
:::

### Testing Entry Functions

The `example_program.leo` program contains an entry function which returns the sum of two `u32` inputs.

```leo file=../code_snippets/testing/example_program/src/main.leo#simple_addition
```

The `test_example_program.leo` contains two tests to ensure that the function logic returns a correct output and fails when the output does not match the sum of the input values.

```leo file=../code_snippets/testing/example_program/tests/test_example_program.leo#test_simple_addition
```

The `@should_fail` annotation should be added after the `@test` annotation for tests that are expected to fail.

```leo file=../code_snippets/testing/example_program/tests/test_example_program.leo#test_simple_addition_fail
```

### Testing as a Specific Account

By default, every `@test` function runs as the same fixed test account. The corresponding address is what [`std::ctx::caller()`](../language/standard_library.md#stdctx) and [`std::ctx::signer()`](../language/standard_library.md#stdctx) resolve to inside the test. The default key is:

```text
APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH
```

To run a single test as a different account, pass a `private_key` argument to the annotation:

```leo file=../code_snippets/testing/example_program/tests/test_example_program.leo#test_with_private_key
```

This is the standard way to exercise access-controlled entry points: pair a privileged-account test that runs to success with a `@should_fail` counterpart that runs under the default (or any other non-privileged) account. The privileged test uses `@test(private_key = "...")` to override the caller; the failing counterpart uses bare `@test` so the default test account is the caller:

```leo file=../code_snippets/testing/example_program/tests/test_example_program.leo#test_admin_pair
```

`private_key` is the only recognized argument to `@test`; passing any other key (e.g. `@test(seed = ...)`) is a compile error. The value must be a string literal containing a valid Leo private key.

### Testing Leo Types

Developers can test that record and struct fields match their expected values. In `example_program.leo`, a record is minted by an entry function shown here:

```leo file=../code_snippets/testing/example_program/src/main.leo#mint_record
```

The corresponding test in `test_example_program.leo` checks that the Record field contains the correct value:

```leo file=../code_snippets/testing/example_program/tests/test_example_program.leo#test_record_maker
```

:::info
Each test file is required to have at least one `@test fn` function.
:::

### Modeling Onchain State

The Leo test framework executes tests via the real VM, so on-chain state (mappings, storage) is fully supported in `@test fn` functions — no special syntax is required. Call entry functions that return `Final` the same way as any other function; the finalization will be executed as part of the test run.

For end-to-end and integration testing against a live network or a local devnet, use the [SDK](https://github.com/ProvableHQ/sdk) directly or `snarkVM` as a library.

### Testing Library Packages

`leo test` works on library packages directly — no wrapper program is needed. Place test files in the `tests/` directory of the library project and call library functions using the `library_name::function` path syntax:

```leo file=../code_snippets/testing/my_lib/tests/test_my_lib.leo#test_program title="tests/test_my_lib.leo"
```

Run `leo test` from the library's root directory:

```bash
cd my_lib
leo test
```

Submodule functions are accessible through their qualified path (e.g., `my_lib::math::triple(4u32)`).

### Running Tests

Invoking the `leo test` command will run all of the compiled and interpreted tests. Developers may optionally select individual tests by supplying a test function name or a string that is contained within a test function name. For instance, to run the test for `test_final`, developers would use the following command:

```bash
leo test test_final
```

Either of the following commands will run both of the addition function tests:

```bash
leo test simple
```

or

```bash
leo test addition
```

See the [`leo test` CLI documentation](./../cli/test.md).

## Running a Devnode

`leo devnode` is a lightweight, single-process node that bypasses consensus and proof generation. It is the recommended local tool for end-to-end deploy/execute testing — no snarkOS installation required.

:::warning
`--skip-deploy-certificate` skips both proof generation **and** the circuit deployment limit check. A deployment that succeeds on a devnode with this flag can still be rejected by Testnet or Mainnet if the circuit exceeds the on-chain limits. Run `leo synthesize --local` before deploying to a public network to verify your program's constraint count.
:::

See the [`leo devnode` CLI reference](./../cli/devnode.md) for setup instructions, all flags, and a step-by-step workflow.

## Running a Devnet

`leo devnet` spins up a full multi-validator snarkOS network locally. It is heavier to set up than `leo devnode` but provides a closer approximation of consensus behaviour for scenarios that require it.

See the [`leo devnet` CLI reference](./../cli/devnet.md) for setup instructions and flags.

## Deploying/Executing on Testnet

To deploy and execute on Testnet, you'll need to set your endpoint back to one of the public facing options. Additionally, you'll need to obtain Testnet credits — visit [**https://faucet.aleo.org/**](https://faucet.aleo.org/) to request them.

## Other Tools

The Aleo community has developed some neat tools to aid in testing.

- [**doko.js**](https://github.com/venture23-aleo/doko-js)
