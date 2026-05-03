import other_program.aleo;

program caller.aleo {
    fn demo() -> u32 {
        // ANCHOR: snippet
        // 4.0
        let result: u32 = other_program.aleo::some_fn(1u32);
        let s: other_program.aleo::MyStruct = other_program.aleo::MyStruct { x: 1u32 };
        // ANCHOR_END: snippet
        return result + s.x;
    }

    @noupgrade
    constructor() {}
}
