program test.aleo {
    transition empty_slice(a: [bool; 1]) -> bool {
        let b = a[0..0];
        return true;
    }
}
