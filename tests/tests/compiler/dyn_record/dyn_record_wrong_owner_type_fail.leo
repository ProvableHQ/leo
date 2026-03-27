program test.aleo {
    fn main(r: dyn record) -> u64 {
        let x: u64 = r.owner;
        return x;
    }
}
