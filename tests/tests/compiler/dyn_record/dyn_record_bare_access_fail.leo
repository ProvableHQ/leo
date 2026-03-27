program test.aleo {
    fn main(r: dyn record) -> u64 {
        // Should fail: field access on dyn record without type annotation
        let balance = r.balance;
        return balance;
    }
}
