// ANCHOR: file
struct Bar {
    data: [u8; 8],
}

program array_structs.aleo {
    record Foo {
        owner: address,
        data: [u8; 8],
    }

    fn build() -> Bar {
        // Array of structs
        let arr_of_structs: [Bar; 2] = [
            Bar { data: [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] },
            Bar { data: [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] },
        ];
        return arr_of_structs[0u8];
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
