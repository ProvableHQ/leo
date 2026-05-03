program other_program.aleo {
    struct MyStruct {
        x: u32,
    }

    fn some_fn(value: u32) -> u32 {
        return value;
    }

    @noupgrade
    constructor() {}
}
