/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    struct Foo {
        a: u8,
    }

    transition foo(flag: bool, a: u8, b: u8, foo: Foo, i: i8) -> u8 {
        a + b;
        flag ? a : b;
        foo.a;
        Foo {
            a: a,
        };
        a;
        1u8;
        -i8;
        ();
        return a + b;
    }

}
