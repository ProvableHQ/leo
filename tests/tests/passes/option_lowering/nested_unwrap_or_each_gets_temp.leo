program test.aleo {
    // Nested `.unwrap_or(...)` calls each synthesize their own `$opt$N` temp:
    // the outer receiver (`a`) and the inner receiver (`b`) should each get a
    // fresh binding, not share one or reuse the other's subtree.
    fn main(x: u32, y: u32, d: u32) -> u32 {
        let a: u32? = x;
        let b: u32? = y;
        return a.unwrap_or(b.unwrap_or(d));
    }
}
