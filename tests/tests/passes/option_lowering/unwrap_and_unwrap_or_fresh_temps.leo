program test.aleo {
    // Repeated unwrap operations on the same logical value each produce a
    // fresh `$opt$N` temp and do not share a binding. The counter is global
    // to the function, so indices are monotonically increasing.
    fn main(x: u32, d: u32) -> u32 {
        let a: u32? = x;
        let first: u32 = a.unwrap();
        let second: u32 = a.unwrap_or(d);
        return first + second;
    }
}
