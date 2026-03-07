# Dynamic Dispatch: Formatter + Disassembler Fixes

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix three gaps introduced by the dynamic dispatch prototype: a silent content-loss bug in the Leo formatter, missing formatter round-trip tests, and a `todo!()` panic in the disassembler when encountering `dynamic.future` signatures.

**Architecture:** Three independent patches touching `crates/fmt/` and `crates/ast/src/stub/function_stub.rs`. No new files, crates, or traits required.

**Tech Stack:** Rust, `leo-fmt` (rowan-based formatter), `leo-ast` (AST + disassembler stubs).

---

### Task 1: Fix formatter — `TYPE_DYN_RECORD` silently dropped

**Files:**
- Modify: `crates/fmt/src/format.rs` (function `format_type`, ~line 1187)

**Background:** `format_type` dispatches on syntax-node kind. `TYPE_DYN_RECORD` is a valid type kind (included in `SyntaxKind::is_type()`) but has no arm in the match — it falls to `_ => {}` which produces empty output. Any `dyn record` type annotation is silently erased when formatting.

**Step 1: Confirm the bug — run the idempotency test (expected to pass now, will fail after we add test fixtures)**

```bash
cargo test -p leo-fmt 2>&1 | tail -5
```

**Step 2: Add the missing arm to `format_type`**

In `crates/fmt/src/format.rs`, find `fn format_type` (~line 1177) and add `TYPE_DYN_RECORD` before the catch-all:

```rust
fn format_type(node: &SyntaxNode, out: &mut Output) {
    match node.kind() {
        TYPE_PRIMITIVE => format_type_primitive(node, out),
        TYPE_LOCATOR => format_type_locator(node, out),
        TYPE_PATH => format_type_path(node, out),
        TYPE_ARRAY => format_type_array(node, out),
        TYPE_VECTOR => format_type_vector(node, out),
        TYPE_TUPLE => format_type_tuple(node, out),
        TYPE_FINAL => format_type_final(node, out),
        TYPE_MAPPING => format_type_mapping(node, out),
        TYPE_OPTIONAL => format_type_optional(node, out),
        TYPE_DYN_RECORD => out.write("dyn record"),
        _ => {}
    }
}
```

`TYPE_DYN_RECORD` needs no child traversal — it is a leaf node containing the two tokens `dyn` and `record`. The output is always the literal string `"dyn record"`.

Also add the import if needed: `TYPE_DYN_RECORD` comes from `leo_parser_rowan::syntax_kind::SyntaxKind::*` which is already glob-imported at the top of the file.

**Step 3: Run formatter tests (expected to pass — no new test fixtures yet)**

```bash
cargo test -p leo-fmt
```

Expected: all pass (the fix is purely additive; existing tests don't involve `dyn record`).

**Step 4: Commit**

```bash
git add crates/fmt/src/format.rs
git commit -m "fix(fmt): emit 'dyn record' in format_type instead of silently dropping it"
```

---

### Task 2: Add formatter round-trip tests for dynamic dispatch syntax

**Files:**
- Create: `crates/fmt/tests/source/dynamic_dispatch.leo`
- Create: `crates/fmt/tests/target/dynamic_dispatch.leo`

**Background:** The formatter test harness (`crates/fmt/tests/harness.rs`) runs two checks for every `source/foo.leo` + `target/foo.leo` pair:
1. `format(source)` == `target` (source→target)
2. `format(target)` == `target` (idempotency)

Create a source file with sloppy whitespace covering all dynamic dispatch syntax, and a target file with clean formatting. The harness picks these up automatically — no test registration needed.

**Step 1: Create `crates/fmt/tests/source/dynamic_dispatch.leo`**

This covers `dyn record` (parameter + return), `_dynamic_call` with and without type params, and the finalize-only intrinsics. Use realistic but badly formatted whitespace to exercise the formatter.

```leo
program   test.aleo   {

    fn   transfer(  r  :   dyn  record   )  ->   dyn   record   {
        return   r   ;
    }

    fn   call_no_ret(   prog  :field,net:  field,func  :  field,  arg:u64  )  {
        _dynamic_call(  prog ,  net  ,   func  ,  arg  )  ;
    }

    fn   call_u64(  prog  :  field  ,  net  :  field  ,  func  :  field  ,  arg:u64  )  ->  u64  {
        let   result  :  u64  = _dynamic_call  ::  [  public   u64  ]  (  prog  ,  net  ,  func  ,  arg  )  ;
        return  result  ;
    }

    fn   call_dyn_record(  prog  :  field  ,  net  :  field  ,  func  :  field  ,  r  :  dyn   record  )  ->  dyn   record  {
        let   out  :  dyn   record  = _dynamic_call  ::  [  dyn   record  ]  (  prog  ,  net  ,  func  ,  r  )  ;
        return   out  ;
    }

    mapping   balances  :  address  =>  u64  ;

    fn   check(  prog  :  field  ,  net  :  field  ,  m  :  field  ,  key  :  address  )  ->  Final  {
        return   final   {   finalize_check(  prog  ,  net  ,  m  ,  key  )  ;   }   ;
    }
}

final   fn   finalize_check(  prog  :  field  ,  net  :  field  ,  m  :  field  ,  key  :  address  )  {
    let   exists  :  bool  = _dynamic_contains(  prog  ,  net  ,  m  ,  key  )  ;
    let   val  :  u64  = _dynamic_get  ::  [  u64  ]  (  prog  ,  net  ,  m  ,  key  )  ;
    let   val2  :  u64  = _dynamic_get_or_use  ::  [  u64  ]  (  prog  ,  net  ,  m  ,  key  ,  0u64  )  ;
    assert(  exists  )  ;
}
```

**Step 2: Run formatter to generate the expected target**

```bash
cargo run -p leo-fmt -- crates/fmt/tests/source/dynamic_dispatch.leo 2>&1
```

Or use the format_source function directly via a quick one-off invocation. Alternatively, write the expected output by hand based on existing patterns, then confirm with the test.

**Step 3: Create `crates/fmt/tests/target/dynamic_dispatch.leo`**

Write the clean expected output. Based on formatting rules (80-char wrap threshold, space after `:`, single space around `->`, etc.):

```leo
program test.aleo {
    fn transfer(r: dyn record) -> dyn record {
        return r;
    }

    fn call_no_ret(prog: field, net: field, func: field, arg: u64) {
        _dynamic_call(prog, net, func, arg);
    }

    fn call_u64(prog: field, net: field, func: field, arg: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64](prog, net, func, arg);
        return result;
    }

    fn call_dyn_record(prog: field, net: field, func: field, r: dyn record) -> dyn record {
        let out: dyn record = _dynamic_call::[dyn record](prog, net, func, r);
        return out;
    }

    mapping balances: address => u64;

    fn check(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_check(prog, net, m, key); };
    }
}

final fn finalize_check(prog: field, net: field, m: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, m, key);
    let val: u64 = _dynamic_get::[u64](prog, net, m, key);
    let val2: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, 0u64);
    assert(exists);
}
```

**Step 4: Run tests and iterate**

```bash
cargo test -p leo-fmt 2>&1
```

If source→target mismatches: the test prints a diff showing what the formatter actually produces. Adjust the target file to match the actual formatted output (the formatter is the source of truth, not our hand-written expectation — as long as the output is reasonable). Re-run until both `test_source_to_target` and `test_idempotency` pass.

**Step 5: Commit**

```bash
git add crates/fmt/tests/source/dynamic_dispatch.leo crates/fmt/tests/target/dynamic_dispatch.leo
git commit -m "test(fmt): add round-trip tests for dyn record and _dynamic_call syntax"
```

---

### Task 3: Fix disassembler — `DynamicFuture` panics instead of returning a type

**Files:**
- Modify: `crates/ast/src/stub/function_stub.rs` (two `todo!()` sites)

**Background:** `FunctionStub::from_function_core` and `FunctionStub::from_finalize` both have `todo!("dynamic futures are not yet supported in Leo")` for the `DynamicFuture` variant. When the disassembler encounters an AVM function that takes or returns a `dynamic.future`, it panics. The fix is to map `DynamicFuture` to a generic `Future` type (`FutureType::new(Vec::new(), None, false)`), which is the best Leo approximation — it signals "this is a future" without encoding the specific call site.

**Step 1: Find the two `todo!()` sites**

```bash
grep -n "DynamicFuture" crates/ast/src/stub/function_stub.rs
```

Expected output (4 lines: 2 match arms with `todo!()`, 2 already handled correctly):
- Line ~209: output in `from_function_core`
- Line ~293: input in `from_function_core`
- Line ~327: input to finalize in `from_finalize`
- Lines ~354,386: closure panics (correct — closures can't have dynamic types)

**Step 2: Fix `from_function_core` — output (line ~209)**

Change:
```rust
ValueType::DynamicFuture => todo!("dynamic futures are not yet supported in Leo"),
```
To:
```rust
ValueType::DynamicFuture => vec![Output {
    mode: Mode::None,
    type_: Type::Future(FutureType::new(Vec::new(), None, false)),
    span: Default::default(),
    id: Default::default(),
}],
```

**Step 3: Fix `from_function_core` — input (line ~293)**

Change:
```rust
ValueType::DynamicFuture => todo!("dynamic futures are not yet supported in Leo"),
```
To:
```rust
ValueType::DynamicFuture => Input {
    identifier: arg_name,
    mode: Mode::None,
    type_: Type::Future(FutureType::new(Vec::new(), None, false)),
    span: Default::default(),
    id: Default::default(),
},
```

**Step 4: Fix `from_finalize` — finalize input (line ~327)**

Change:
```rust
FinalizeType::DynamicFuture => todo!("dynamic futures are not yet supported in Leo"),
```
To:
```rust
FinalizeType::DynamicFuture => Type::Future(FutureType::new(Vec::new(), None, false)),
```

Note: the finalize match returns a `Type` (not an `Input`), so the fix is a one-liner.

**Step 5: Check it compiles**

```bash
cargo check -p leo-ast
```

**Step 6: Run relevant tests**

```bash
cargo test -p leo-ast
cargo test -p leo-disassembler
```

Expected: all pass (the `credits_test` and `array_test` are `#[ignore]` so they won't run).

**Step 7: Commit**

```bash
git add crates/ast/src/stub/function_stub.rs
git commit -m "fix(disassembler): map DynamicFuture to Future instead of panicking"
```

---

## Validation

After all three tasks:

```bash
cargo check -p leo-fmt -p leo-ast -p leo-disassembler
cargo clippy -p leo-fmt -p leo-ast -p leo-disassembler -- -D warnings
cargo +nightly fmt --check
cargo test -p leo-fmt
cargo test -p leo-ast
cargo test -p leo-disassembler
```
