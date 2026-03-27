program test.aleo {
    fn main(r: dyn) -> u64 {
        let x: u64 = r.owner;
        return x;
    }
}
