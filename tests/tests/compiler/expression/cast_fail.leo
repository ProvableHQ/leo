
struct Foo {
    data: field,
}

program test.aleo {
    mapping balances: field => field;
    fn main(a: field) -> Final {
        // Cannot cast to a string.
        let b: string = a as string;
        // Cannot cast a struct.
        let c: Foo = Foo { data: a };
        let d: field = c as field;
        // Cannot cast a tuple.
        let e: (field, field) = (a, a);
        let f: field = e as field;
        return final { finalize_main(a); };
    }
}

final fn finalize_main(a: field) {
    // Cannot cast a mapping.
    let b: field = balances as field;
    assert_eq(b, a);
}
