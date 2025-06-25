
program test.aleo {
    struct S {
        x: [field; 2]
    }

    transition main() -> S {
        let array: [field; 2] = [1field, 1field];
        array[0] = 2field;
        // See bug #28711 - the write was not being captured.
        return S {
            x: array,
        };
    }

    transition main2() -> S {
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
