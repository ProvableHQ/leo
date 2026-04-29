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
