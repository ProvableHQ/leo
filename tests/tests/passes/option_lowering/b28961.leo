program test.aleo {
    fn assign_optional_from_unwrap() -> u32 {
        let a: u32? = 5u32;

        // Case 1: T? = unwrap(T?)
        let b: u32? = a.unwrap();   // should implicitly wrap to u32?

        // Case 2: T? = unwrap(T?) + T
        let c: u32? = a.unwrap() + 7u32;

        // Case 3: T? = unwrap_or(T?, <unsuffixed>) + T
        let d: u32? = a.unwrap_or(0);

        return b.unwrap() + c.unwrap() + d.unwrap();
    }
}
