program test.aleo {
    // Define a generic function `foo` that takes all allowed types and returns them as a tuple
    inline foo::[
        a1: bool,
        a2: u8,
        a3: u16,
        a4: u32,
        a5: u64,
        a6: u128,
        a7: i8,
        a8: i16,
        a9: i32,
        a10: i64,
        a11: i128,
        a12: scalar,
        a13: address,
        a14: group,
        a15: field
    ]() -> (
        bool,
        u8,
        u16,
        u32,
        u64,
        u128,
        i8,
        i16,
        i32,
        i64,
        i128,
        scalar,
        address,
        group,
        field
    ) {
        return (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15);
    }

    // Main transition that calls `foo` with values of all types and returns its result
    transition main() -> (
        bool,
        u8,
        u16,
        u32,
        u64,
        u128,
        i8,
        i16,
        i32,
        i64,
        i128,
        scalar,
        address,
        group,
        field
    ) {
        // Call `foo` with values for all allowed types. Also check for type inference of unsuffixed literals here
        return foo::[
            true,
            8,
            16,
            32,
            64,
            128,
            8,
            16,
            32,
            64,
            128,
            42, // scalar
            aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4, // address
            0, // group
            1, // field
        ]();
    }
}
