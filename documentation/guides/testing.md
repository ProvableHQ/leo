---
id: test
title: Testing, Testing, 123
sidebar_label: Testing
---

[general tags]: # "guides, tests, testing, unit_testing, integration_testing, devnode, devnet, testnet"

After deployment, an application stays on the ledger permanently. Thus, consider each edge case and test your code fully. You can use the following tools and techniques.

- [**Unit and Integration Testing**](#unit-and-integration-testing) - Validate Leo program logic through test cases.
- [**Running a Devnode**](#running-a-devnode) - Deploy and execute against a lightweight local node.
- [**Running a Devnet**](#running-a-devnet) - Deploy and execute on a full local devnet backed by snarkOS.
- [**Deploying/Executing on Testnet**](#deployingexecuting-on-testnet) - Deploy and execute on the Aleo Testnet.
- [**Other Tools**](#other-tools) - Tools and methodologies developed by the open-source Aleo community.

## Choosing a Testing Strategy

| Tool | Best for | Notes |
| ---- | -------- | ----- |
| `leo test` | Logic, record fields, mappings | Fast. No network round-trip. No fee credits required |
| `leo devnode` | End-to-end deploy/execute cycles, multi-program interaction | No snarkOS required. Proof generation can be skipped |
| `leo devnet` | Full consensus scenarios, multi-validator behavior | Requires a snarkOS installation. Heavier setup |
| Testnet | Final validation before mainnet | Real credits required. Use the [Aleo faucet](https://faucet.aleo.org/) |

Start with `leo test` for pure logic. Use `leo devnode` to test deployment and execution cycles against a live node. Use `leo devnet` when the scenario requires full consensus behavior. Use Testnet for final validation before Mainnet.

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

A test program can name a library or program in the package `dependencies`. Thus, tests can construct dependency types or call dependency functions directly. See [`dependencies` vs. `dev_dependencies`](./dependencies.md#dependencies-vs-dev_dependencies) for visibility rules.

This tutorial will use an example program which can be found in the [example's repository](https://github.com/ProvableHQ/leo-examples/tree/main/example_with_test).

:::info
You can add multiple `.leo` files to the test directory. Each test file name must match the program name in that file. For example, `test_example_program.leo` must contain the program name `test_example_program.aleo`.

Leo compiles each test file independently. Test files are separate test programs, not modules of one combined program.
:::

### Testing Entry Functions

The `example_program.leo` program contains an entry function which returns the sum of two `u32` inputs.

```leo file=../code_snippets/testing/example_program/src/main.leo#simple_addition
```

`test_example_program.leo` contains two tests. They verify the correct sum and the failure that occurs when the output does not match the input sum.

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

To test an access-controlled entry point, pair a privileged test with a `@should_fail` test. Run the second test with the default or another nonprivileged account. The privileged test uses `@test(private_key = "...")` to override the caller. The failing test uses `@test`, so it uses the default test account:

```leo file=../code_snippets/testing/example_program/tests/test_example_program.leo#test_admin_pair
```

`private_key` is the only recognized argument to `@test`. Passing any other key (for example `@test(seed = ...)`) is a compile error. The value must be a string literal containing a valid Leo private key.

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

The Leo test framework executes tests in the real VM. Thus, `@test fn` functions fully support on-chain mappings and storage without special syntax. Call entry functions that return `Final` in the same way as other functions. The test run executes the finalization.

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

Submodule functions are accessible through their qualified path (for example, `my_lib::math::triple(4u32)`).

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

Results use qualified names such as `test_example_program.leo::test_addition`. A filter can match that name or the compiled form `test_example_program.aleo/test_addition`:

```bash
leo test test_example_program.leo::test_addition
leo test test_example_program.aleo/test_addition
```

See the [`leo test` CLI documentation](./../cli/test.md).

## Running a Devnode

`leo devnode` is a lightweight, single-process node that bypasses consensus and proof generation. It is the recommended local tool for end-to-end deploy/execute testing — no snarkOS installation required.

:::warning
`--skip-deploy-certificate` skips both proof generation **and** the circuit deployment limit check. A deployment that succeeds on a devnode with this flag can still be rejected by Testnet or Mainnet if the circuit exceeds the on-chain limits. Run `leo synthesize --local` before deploying to a public network to verify your program's constraint count.
:::

See the [`leo devnode` CLI reference](./../cli/devnode.md) for setup instructions, all flags, and a step-by-step workflow.

## Running a Devnet

`leo devnet` starts a full multi-validator snarkOS network locally. It requires more resources than `leo devnode` but provides a closer approximation of consensus behavior.

See the [`leo devnet` CLI reference](./../cli/devnet.md) for setup instructions and flags.

## Deploying/Executing on Testnet

To deploy and execute on Testnet, you will need to set your endpoint back to one of the public facing options. Additionally, you will need to obtain Testnet credits — visit [**https://faucet.aleo.org/**](https://faucet.aleo.org/) to request them.

## Other Tools

The Aleo community has developed some neat tools to aid in testing.

- [**doko.js**](https://github.com/venture23-aleo/doko-js)
