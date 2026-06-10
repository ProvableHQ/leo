struct MyStruct {
    x: u32,
}

program other_program.aleo {
    fn some_fn(value: u32) -> u32 {
        return value;
    }

    @noupgrade
    constructor() {}
}
