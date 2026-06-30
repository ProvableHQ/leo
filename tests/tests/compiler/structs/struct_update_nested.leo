struct Inner {
    a: u32,
    b: u32,
}

struct Outer {
    inner: Inner,
    tag: u32,
}

program test.aleo {
    fn main(base: Outer, new_a: u32) -> u32 {
        // The inner struct is rebuilt with its own `..` update from `base.inner`,
        // and the outer `..base` copies the `tag` field.
        let new_inner: Inner = Inner { a: new_a, ..base.inner };
        let updated: Outer = Outer { inner: new_inner, ..base };
        return updated.inner.a + updated.inner.b + updated.tag;
    }

    @noupgrade
    constructor() {}
}
