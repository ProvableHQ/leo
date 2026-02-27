interface Base {
    fn base(public x: u32) -> public u32;
    record Receipt;
}

interface ParentB {
}

interface Child: Base + ParentB {
    // comment before prototype
    fn compute::[N: u32](public x: u32, private y: u32) -> public u32;
    // comment before record
    record Log;
}

program interface_test.aleo: Base + Child {
    record Receipt {
        owner: address,
    }

    fn base(public x: u32) -> public u32 {
        return x;
    }

    fn main(public x: u32) -> public u32 {
        return x;
    }

    fn compute::[N: u32](public x: u32, private y: u32) -> public u32 {
        return x + y;
    }

    record Log {
        owner: address,
    }
}
