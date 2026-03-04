program child.aleo {
    fn foo() -> Final {
        return final { finalize_foo(); };
    }
}

final fn finalize_foo() {}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main(cond: bool) -> Final {
        if cond {
            let f: Final = child.aleo/foo();
            return final { f.run(); };
        }
        return final { fallback(); };
    }
}

final fn fallback() {}
