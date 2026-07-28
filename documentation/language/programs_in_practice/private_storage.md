---
id: private_state
title: Private State
sidebar_label: Private State
---

[general tags]: # "program, record"

## Records

A [record](https://docs.aleo.org/learn/core-concepts/public-and-private-state#private-state) data type is the method of encoding private state on Aleo. Records are declared as `record {name} {}`. A record name must not contain the keyword `aleo`, and must not be a prefix of any other record name.

Records contain component declarations `{visibility} {name}: {type},`. Names of record components must not contain the keyword `aleo`. The visibility qualifier may be specified as `constant`, `public`, or `private`. If no qualifier is provided, Leo defaults to `private`.

Each record must contain an `owner` component of type `address`, as shown below.
A record function input also requires the `_nonce: group` and `_version: u8` components.
Do not declare these components in the Leo program. The compiler inserts them automatically.

```leo file=../../code_snippets/data_types/demo/src/main.leo#token_record showLineNumbers
```
