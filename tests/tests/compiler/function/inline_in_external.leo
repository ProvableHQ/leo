// This test ensures that `FunctionInlining` works in Leo stubs

program child.aleo {
    fn foo() -> Final {
        return final {
            assert(f() == 4);
            assert(f() != 4);
        };
    }
}

fn f() -> u32 { return 4; }

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    fn foo() -> Final {
        return final {
            assert(f() == 4);
        };
    }
}

fn f() -> u32 { return 4; }
