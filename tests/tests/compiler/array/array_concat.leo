
program test.aleo {
    // Basic array concatenation
    transition foo(a: [u32; 2], b: [u32; 2]) -> [u32; 4] {
        return a + b;
    }

    // Nested array concatenation (flatten)
    transition flatten(a: [[bool; 2]; 2]) -> [bool; 4] {
        let b: [bool; 4] = a[0] + a[1];
        let c: [bool; 4] = [a[0][0], a[0][1], a[1][0], a[1][1]];
        assert_eq(b, c);
        return b;
    }

    // Different sized arrays
    transition concat_different_sizes(a: [u8; 3], b: [u8; 2]) -> [u8; 5] {
        return a + b;
    }
}
