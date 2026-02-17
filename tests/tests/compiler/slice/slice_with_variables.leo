
program test.aleo {
    const START: u32 = 1u32;
    const END: u32 = 4u32;

    // Slice with const variables
    transition foo(a: [u32; 8]) -> [u32; 3] {
        return a[START..END];
    }
}
