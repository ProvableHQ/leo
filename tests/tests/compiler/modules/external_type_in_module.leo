export struct Pair {
    a: u64,
    b: u64,
}

program dep.aleo {
    fn make(a: u64, b: u64) -> Pair {
        return Pair { a, b };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import dep.aleo;

program external_type_in_module.aleo {
    fn main(a: u64, b: u64) -> u64 {
        let p = dep.aleo::Pair { a, b };
        return m::sum(p);
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: m.leo --- //

// An explicit program locator inside a module must resolve at the external
// program's root, not relative to the current module.
export fn sum(p: dep.aleo::Pair) -> u64 {
    return p.a + p.b;
}
