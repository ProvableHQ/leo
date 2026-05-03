// ANCHOR: file
struct S {
    x: field,
    y: u32,
}

program test.aleo {
    fn foo(y: u32) -> S {
        let s: S = S {
            x: 172field,
            y,
        };
        return s;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
