/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping balances: field => field;

    struct Foo {
        data: field;
    }

    transition main(a: field) {
        // Cannot cast to a string.
        let b: string = a as string;
        // Cannot cast a struct.
        let c: Foo = Foo { data: a };
        let d: field = c as field;
        // Cannot cast a tuple.
        let e: (field, field) = (a, a);
        let f: field = e as field;
        return then finalize(a);
    }

    finalize main(a: field) {
        // Cannot cast a mapping.
        let b: field = balances as field;
        assert_eq(b, a);
    }
}
