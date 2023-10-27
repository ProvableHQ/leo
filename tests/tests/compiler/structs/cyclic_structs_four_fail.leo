/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    struct Foo {
        foo: [Foo; 1],
    }

    struct Bar {
        baz: [Baz, 2],
    }

    struct Baz {
        bar: [Bar, 3],
    }

    struct One {
        two: [Two, 2],
    }

    struct Two {
        three: [Three, 3],
        four: [Four, 4],
    }

    struct Three {
        one: [One, 1],
    }

    struct Four {
        one: [One, 1],
    }
}
