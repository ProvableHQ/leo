// ANCHOR: file
// The 'admin_example' program.
program admin_example.aleo {
    // This constructor is for the "admin" mode.
    // It ensures that only the designated admin can upgrade the program.
    @admin(address="aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4")
    constructor() {
        // The Leo compiler automatically generates the constructor logic.
    }

    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
// ANCHOR_END: file
