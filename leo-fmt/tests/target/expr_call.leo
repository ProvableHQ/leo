program test.aleo {
    function add(a: u64, b: u64) -> u64 {
        return a + b;
    }

    function to_field(a: u64) -> field {
        return a as field;
    }

    transition main() -> u64 {
        let x: u64 = add(1u64, 2u64);
        let y: u64 = add(1u64, 2u64);
        let hash: field = to_field(1u64);
        return x;
    }
}
