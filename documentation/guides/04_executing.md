---
id: execute
title: Executing Your Programs
sidebar_label: Executing
---

[general tags]: # "guides, execute, execution, transaction, transaction_status"

The `leo execute` command executes the Leo program and outputs a transaction object

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
       Leo     The program checksum is: '[212u8, 91u8, ... , 107u8]'.
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

Under the hood, `leo execute` produces a JSON object. This is a [`Transaction`](https://developer.aleo.org/concepts/fundamentals/transactions) that can be broadcast to the Aleo network. You can view this JSON by passing the `--print` flag to `leo execute`.
