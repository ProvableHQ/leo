// Regression for #29390. When a callee like `finish` is rejected with
// ETYC0372043 ("Cannot call a local entry point fn from an entry point
// fn") its finalize-table entry is not recorded. The visitor that fans
// out to those call sites used to look up the missing entry with a
// hard `panic!` and ICE the compiler. The fix soft-fails to Type::Err
// so the user sees the real diagnostic instead of an internal panic.
program test.aleo {
    fn caller() -> Final {
        return finish();
    }

    fn finish() -> Final {
        return final {};
    }
}
