
struct S {
    x: [field; 2]
}

program test.aleo {
    fn main() -> S {
        let array: [field; 2] = [1field, 1field];
        array[0] = 2field;
        // See bug #28711 - the write was not being captured.
        return S {
            x: array,
        };
    }

    fn main2() -> S {
        let array: [field; 2] = [1field, 1field];
        for i in 0u32..2u32 {
            array[i] = 2field;
        }
        // See bug #28711 - the write was not being captured.
        return S {
            x: array,
        };
    }
}
