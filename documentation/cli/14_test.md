---
id: cli_test
title: ""
sidebar_label: Test
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_test, testing"

# `leo test`

This command runs all the test cases specified in the Leo file in `tests/`.

If you want to run a specific set of tests, run the following command:

```bash
leo test <TEST_NAME>
```

where `<TEST_NAME>` is the string to match against the qualified name of each test.

Check out the [**Testing**](./../guides/08_testing.md) guide for more information.

## Flags

```text
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
--prove
    Generate a full ZK proof for each executed transaction. Proof generation is disabled by default to keep test runs fast.
--no-cache
    Don't use the dependency cache.
--no-local
    Don't use the local source code.
```
