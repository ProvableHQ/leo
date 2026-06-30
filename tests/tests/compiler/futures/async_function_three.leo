// Ensure Final types are correctly found through
// two layers of dependencies.

export final fn finalize(x: u8) {
    assert_eq(1u8, 1u8);
}

program dependent.aleo {
    fn t1() -> Final {
        let f: Final = final { finalize(1u8); };
        return f;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import dependent.aleo;

export final fn finalize(f: Final) {
    f.run();
}

program dependent2.aleo {
    fn t1() -> Final {
        let f: Final = dependent.aleo::t1();
        return final { finalize(f); };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import dependent2.aleo;

export final fn finalize(f: Final) {
    f.run();
}

program test.aleo {
    fn t1() -> Final {
        let f: Final = dependent2.aleo::t1();
        return final { finalize(f); };
    }

    @noupgrade
    constructor() {}
}
