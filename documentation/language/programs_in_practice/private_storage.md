---
id: private_state
title: Private State
sidebar_label: Private State
---

[general tags]: # "program, record"

## Records

A [record](https://developer.aleo.org/concepts/fundamentals/records) data type is the method of encoding private state on Aleo. Records are declared as `record {name} {}`. A record name must not contain the keyword `aleo`, and must not be a prefix of any other record name.

Records contain component declarations `{visibility} {name}: {type},`. Names of record components must not contain the keyword `aleo`. The visibility qualifier may be specified as `constant`, `public`, or `private`. If no qualifier is provided, Leo defaults to `private`.

Record data structures must always contain a component named `owner` of type `address`, as shown below. When passing a record as input to a program function, the `_nonce: group` and `_version: u8` components are also required but do not need to be declared in the Leo program. They are inserted automatically by the compiler.

```leo file=../../code_snippets/data_types/demo/src/main.leo#token_record showLineNumbers
```
