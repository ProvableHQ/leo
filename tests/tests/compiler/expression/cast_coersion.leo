struct foo {
    data: field,
}

program test.aleo {
    fn main(f: bool, a: group, b: address) -> (foo, foo) {
        let first: foo = foo { data: a as field };
        if f {
            let second: foo = foo { data: 1field };
            return (first, second);
        } else {
            let second: foo = foo { data: b as field };
            return (first, second);
        }
    }
}
