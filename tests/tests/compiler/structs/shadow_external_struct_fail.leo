export struct Foo {
    bar: [Bar; 1],
}

export struct Bar {
    baz: [Baz; 2],
}

export struct Baz {
    one: One,
}

export struct One {
    two: [Two; 2],
}

export struct Two {
    val1: u32,
    val2: u32,
}

program child.aleo {
    fn create() -> Foo {
        return Foo {
            bar: [
                Bar {
                    baz: [
                        Baz {
                            one: One {
                                two: [
                                    Two { val1: 1u32, val2: 2u32 },
                                    Two { val1: 3u32, val2: 4u32 },
                                ],
                            },
                        },
                        Baz {
                            one: One {
                                two: [
                                    Two { val1: 5u32, val2: 6u32 },
                                    Two { val1: 7u32, val2: 8u32 },
                                ],
                            },
                        },
                    ],
                },
            ],
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import child.aleo;

export struct Foo {
    bar: [Bar; 1],
}

export struct Bar {
    baz: [Baz; 2],
    bog: u32,
}

export struct Baz {
    one: One,
}

export struct One {
    two: [Two; 2],
}

export struct Two {
    val1: u32,
    val2: u32,
}

program parent.aleo {
    fn create_wrapper() -> Foo {
        return child.aleo::create();
    }

    @noupgrade
    constructor() {}
}
