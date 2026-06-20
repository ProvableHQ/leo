/*
seed = 123456789
start_height = 16

[case]
program = "outer.aleo"
function = "outer_1"
input = ["0u32"]

[case]
program = "outer.aleo"
function = "outer_2"
input = ["0u32"]
*/

program inner.aleo {
    mapping foo: u32 => u32;

    fn inner(a: u32) -> Final {
        return final { finalize(a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize(a: u32) {
    Mapping::set(foo, 0u32, a);
}

// --- Next Program --- //
import inner.aleo;

program middle.aleo {
    fn mid(a: u32) -> Final {
        let f1: Final = inner.aleo::inner(0u32);
        let f2: Final = inner.aleo::inner(1u32);
        return final { finalize(f2, f1); };
    }

    fn dummy() {}

    @noupgrade
    constructor() {}
}

export final fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}

// --- Next Program --- //
import inner.aleo;
import middle.aleo;

program outer.aleo {
    fn outer_1(a: u32) -> Final {
        middle.aleo::dummy();
        let f1: Final = middle.aleo::mid(0u32);
        let f2: Final = middle.aleo::mid(1u32);
        return final { finalize(f1, f2); };
    }

    fn outer_2(a: u32) -> Final {
        let f1: Final = middle.aleo::mid(0u32);
        middle.aleo::dummy();
        let f2: Final = middle.aleo::mid(1u32);
        return final { finalize(f1, f2); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}
