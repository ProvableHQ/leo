program test.aleo {
    fn check_equal(a: identifier, b: identifier) -> bool {
        return a == b;
    }

    fn check_not_equal(a: identifier, b: identifier) -> bool {
        return a != b;
    }

    fn literal_test(x: identifier) -> bool {
        let y: identifier = 'hello';
        return x == y;
    }
}
