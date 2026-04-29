struct S2 {
    x: field,
    y: u32,
}

program external_program.aleo {
    fn build_s2() -> S2 {
        return S2 { x: 1field, y: 2u32 };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import external_program.aleo;

program consumer.aleo {
    fn build() -> external_program.aleo::S2 {
        // ANCHOR: file
        let s: external_program.aleo::S2 = external_program.aleo::S2 {
            x: 1field,
            y: 2u32,
        };
        // ANCHOR_END: file
        return s;
    }

    @noupgrade
    constructor() {}
}
