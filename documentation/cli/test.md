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

Check out the [**Testing**](./../guides/testing.md) guide for more information.

## Flags

```text
--build-tests
    Build tests along with the main program and dependencies.
--prove
    Generate a full ZK proof for each executed transaction. Proof generation is disabled by default to keep test runs fast.
--no-cache
    Don't use the dependency cache.
--no-local
    Don't use the local source code.
```

## Workspace Behavior

When run inside a [workspace](../guides/workspaces.md):

- **From workspace root:** Runs tests for all members in dependency order.
- **From a member directory:** Runs tests only for that member.
- **With `--package <NAME>`:** Runs tests only for the specified member.

```bash
# Test all workspace members
leo test

# Test only the token member
leo test -p token
```
