---
id: execute
title: Executing Your Programs
sidebar_label: Executing
---

[general tags]: # "guides, execute, execution, transaction, transaction_status"

The `leo execute` command executes the Leo program and outputs a transaction object

## `leo run` vs `leo execute`

Leo offers two commands for running an entry function, and they serve different
purposes:

- `leo run` evaluates the function and prints its outputs locally. It does not
  generate a zero-knowledge proof, does not produce a transaction, and nothing
  touches the network. This makes it fast and ideal for iterating on your logic.
- `leo execute` compiles the program, synthesizes the circuit, generates an
  execution proof, and produces a broadcastable [`Transaction`](https://docs.aleo.org/learn/core-concepts/transactions/index.html).
  Pass `--broadcast` to send that transaction to the network.

A typical development loop is to iterate with `leo run` until the function
behaves as expected, then switch to `leo execute` to produce the proof and
transaction.

## Running `leo execute`

```bash
leo execute <FUNCTION_NAME> <INPUT_1> <INPUT_2> ...
```

Optionally, you can execute a function in a remote Leo program by using

```bash
leo execute <PROGRAM_NAME>.aleo::<FUNCTION_NAME> <INPUT_1> <INPUT_2> ...
```

If executing a function from a local program, the `leo execute` command will first build/compile that program:

```bash title="console output:"
       Leo     2 statements before dead code elimination.
       Leo     2 statements after dead code elimination.
       Leo ✅ Compiled 'hello.aleo' into Aleo instructions.

```

It will then print out the summary of the execution plan with

```bash
🚀 Execution Plan Summary
──────────────────────────────────────────────
🔧 Configuration:
  Private Key:        APrivateKey1zkp...
  Address:            aleo1...
  Endpoint:           https://api.explorer.provable.com/v1
  Network:            <testnet | mainnet>
  Consensus Version:  9

🎯 Execution Target:
  Program:        <PROGRAM_NAME>
  Function:       <FUNCTION_NAME>
  Source:         <local | remote>

💸 Fee Info:
  Priority Fee:   0 μcredits
  Fee Record:     no (public fee)

⚙️ Actions:
  - Transaction will NOT be printed to the console.
  - Transaction will NOT be saved to a file.
  - Transaction will NOT be broadcast to the network.
```

Finally, an execution cost breakdown will be printed alongside any outputs from the function itself.

```bash
📊 Execution Summary for <PROGRAM_NAME>
──────────────────────────────────────────────
💰 Cost Breakdown (credits)
  Transaction Storage:  0.001316
  On‑chain Execution:   0.000000
  Priority Fee:         0.000000
  Total Fee:            0.001316
──────────────────────────────────────────────

➡️  Output

  • <OUTPUT_1>
  • <OUTPUT_2>
  ...
```

Under the hood, `leo execute` produces a JSON object. This is a [`Transaction`](https://docs.aleo.org/learn/core-concepts/transactions/index.html) that can be broadcast to the Aleo network. You can view this JSON by passing the `--print` flag to `leo execute`.
