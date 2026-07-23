---
id: upgradability
title: Upgrading Programs
sidebar_label: Upgrading Programs
---

[general tags]: # "guides, upgrade, program, transaction, constructor"

This guide provides a practical overview of Aleo's program upgradability framework, tailored for developers using the Leo language. You'll learn how to configure your program, implement common upgrade patterns, and follow best practices for writing secure, maintainable applications.

## Getting Started: The Upgrade Policy

Your program's upgrade policy is defined by an annotation on a constructor (see below) in the Leo program.
The Leo compiler reads the annotation to understand your intent and generates the appropriate underlying code.

There are four primary upgrade modes:

| Mode         | Description                                                                                       |
| ------------ | ------------------------------------------------------------------------------------------------- |
| `@noupgrade` | The program is not upgradable.                                                                    |
| `@admin`     | Upgrades are controlled by a single, hardcoded admin address.                                     |
| `@checksum`  | Upgrades are governed by an on-chain checksum, often managed by a separate program (e.g., a DAO). |
| `@custom`    | You write the entire upgrade logic from scratch in the `constructor`.                             |

### Annotation Syntax

Every constructor must carry **exactly one** of the four upgrade annotations. Their argument grammar is:

```text
@noupgrade
@custom
@admin(address = "<aleo-address>")
@checksum(mapping = "<locator>", key = "<literal>")
```

- **`@noupgrade`** and **`@custom`** take no arguments.
- **`@admin`** takes a single `address` argument — a string literal containing a valid Aleo address (`aleo1…`).
- **`@checksum`** takes two arguments:
  - `mapping` — a string locator naming the mapping that holds the approved checksum. Both `prog.aleo::mapping_name` (Leo path syntax) and `prog.aleo/mapping_name` (AVM locator syntax) are accepted.
  - `key` — a string containing a Leo plaintext literal of any primitive type (`field`, `group`, `address`, `scalar`, `bool`, integer types, `signature`, `identifier`). The compiler infers the key's type from the literal's suffix, so include the suffix when the literal needs one (e.g. `"5u32"`, `"1field"`).

All argument values are written as string literals; the compiler parses the contained text against the expected sub-grammar (address, locator, plaintext literal). Whitespace around the `=` is optional — `@admin(address="aleo1…")` and `@admin(address = "aleo1…")` are equivalent.

A constructor with **no annotation**, with **multiple annotations**, or with an unknown key (e.g. `@admin(owner = "…")`) is a compile error.

## Core Mechanics

Upgradability revolves around a special `constructor` function and on-chain program metadata.

### The `constructor`

The `constructor` is a special function that runs on-chain during every deployment and upgrade. Think of it as the gatekeeper for your program.
There are two key properties of the `constructor` related to upgradability:

- **Foundational:** All programs must be deployed with a `constructor`. If the `constructor` logic fails (e.g., a failed `assert`), the entire deployment or upgrade transaction is rejected.
- **Immutable:** The logic inside the `constructor` is set in stone at the first deployment. It can never be changed by a future upgrade. Any bugs introduced here are permanent, so audit your constructor carefully.

### Program Metadata Operands

Within a `constructor`, you can access on-chain metadata about the program through the [`std::ctx`](../language/standard_library.md#stdctx) module of the standard library.

| Operand                       | Leo Type   | Description                                                                                                                                  |
| ----------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `std::ctx::addr()`            | `address`  | The program's own account address.                                                                                                           |
| `std::ctx::edition()`         | `u16`      | The program's version number. Starts at `0` and is incremented by `1` for each upgrade. The edition is tracked automatically on the network. |
| `std::ctx::program_owner()`   | `address`  | The address that submitted the deployment transaction.                                                                                       |
| `std::ctx::checksum()`        | `[u8, 32]` | The program's checksum, which is a unique identifier for the program's code.                                                                 |

You may also refer to another program's metadata through the [`std::prog`](../language/standard_library.md#stdprog) module — for example `std::prog::edition::[credits.aleo]()` or `std::prog::program_owner::[foo.aleo]()`.
You will need to import the program in your Leo file to use this syntax.

Note. Programs deployed before the upgradability feature (i.e. using Leo version < v3.1.0) do not have a `program_owner`. Attempting to access it will result in a runtime error.

---

## Upgrade Patterns in Leo

Below are some common upgrade patterns in Leo.

You may also refer to the working Leo [examples](https://github.com/ProvableHQ/leo-examples/tree/main/upgrades).

### Pattern 1: Non-Upgradable

**Goal:** Explicitly prevent all future upgrades.

**`main.leo`**

The Leo compiler automatically generates a constructor that locks the program to its initial version.

```leo file=../code_snippets/upgradability/noupgrade/src/main.leo#file
```

The corresponding AVM code is:

```text
constructor:
    assert.eq edition 0u16
```

### Pattern 2: Admin-Driven Upgrade

**Goal:** Restrict upgrades to a single, hardcoded admin address.

**`main.leo`**

```leo file=../code_snippets/upgradability/admin/src/main.leo#file
```

The corresponding AVM code is:

```text
constructor:
    assert.eq program_owner aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
```

### Pattern 3: Checksum-Driven (Vote Example)

**Goal:** Delegate upgrade authority to a separate governance program that manages a list of approved code checksums.

**`main.leo`**

The compiler uses the `mapping` and `key` fields to generate a constructor that looks up the approved checksum from the `basic_voting.aleo` program.

```leo file=../code_snippets/upgradability/vote/src/main.leo#file
```

The corresponding AVM code is:

```text
constructor:
    branch.eq edition 0u16 to end;
    get basic_voting.aleo/approved_checksum[true] into r0;
    assert.eq checksum r0;
    position end;
```

### Warning: Keep Compiler Versions Consistent

Checksum-governed upgrades are bytecode-sensitive. Do not assume bytecode is stable across Leo compiler versions: different compiler versions can produce different bytecode for identical source, which can cause a checksum-governed upgrade to fail.

Version consistency matters beyond checksums, though. A program's constructor cannot change between upgrades, and compiler behavior or supported features can change (or even break) across versions. Building every edition with the same pinned toolchain avoids both bytecode drift and subtle semantic differences a different compiler version could introduce.

Pin the Leo compiler version used for the initial deployment and all subsequent upgrades. Before deploying or approving an upgrade, check the local toolchain:

```bash
leo --version
```

To inspect the source currently deployed on the network, query the program. Use `--edition <N>` when you need to inspect a specific edition:

```bash
leo query program <NAME>
leo query program <NAME> --edition <N>
```

The query returns deployed source; it does not print the compiler version. Use it with your pinned toolchain record and rebuild with the same Leo version to avoid bytecode drift.

### Pattern 4: Custom Logic (Time-lock Example)

**Goal:** Enforce a time delay before an upgrade is allowed. No pre-defined mode is available for this so we'll have to write our own upgrade policy

**`main.leo`**

With the `@custom` constructor, you are responsible for writing the entire constructor logic yourself.

```leo file=../code_snippets/upgradability/timelock/src/main.leo#file
```

The corresponding AVM code is:

```text
constructor:
    gt edition 0u16 into r0;
    branch.eq r0 false to end_then_0_0;
    gte block.height 1300u32 into r1;
    assert.eq r1 true;
    branch.eq true true to end_otherwise_0_1;
    position end_then_0_0;
    position end_otherwise_0_1;
```

---

## The Rules: What You Can and Cannot Change

The protocol enforces strict rules to ensure that upgrades don't break dependent applications or corrupt existing state.

An upgrade **can**:

- Change the internal logic of existing entry `fn` bodies and `final { }` blocks.
- Add new `struct`s, `record`s, `mapping`s, and `fn` declarations.

An upgrade **cannot**:

- Change the input or output signatures of any existing entry `fn`.
- Modify or delete any existing `struct`, `record`, or `mapping`.
- Delete any existing program component.

| Program Component         | Delete |   Modify   | Add |
| ------------------------- | :----: | :--------: | :-: |
| `import`                  |   ❌   |     ❌     | ✅  |
| `struct`                  |   ❌   |     ❌     | ✅  |
| `record`                  |   ❌   |     ❌     | ✅  |
| `mapping`                 |   ❌   |     ❌     | ✅  |
| inlined `fn` (helper)     |   ✅   |     ✅     | ✅  |
| non-inlined `fn` (helper) |   ❌   |     ❌     | ✅  |
| `fn` (entry)              |   ❌   | ✅ (logic) | ✅  |
| `final fn` (entry)        |   ❌   | ✅ (logic) | ✅  |
| `constructor`             |   ❌   |     ❌     | ❌  |

---

## Security Checklist

Program mutability introduces new risks. Keep these points in mind:

- **Audit the `constructor` intensely.** Its logic is permanent and cannot be fixed after deployment.
- **Prefer multi-sig or DAO governance over a single admin.** A single point of failure is risky.
- **Implement time-locks for major upgrades.** Giving users a window to react builds trust.
- **Plan for "ossification".** Provide a way to make your program immutable (e.g., by transferring admin rights to a burn address) to give users long-term certainty.

## Legacy Programs

If you have a program that was deployed before the upgradability feature was enabled (or any program deployed without a `constructor`):

**It is permanently non-upgradable.**

There is **no migration path** to make a legacy program upgradable. If you need to add new features, you must deploy an entirely new program and have your users migrate to it.
