program test.aleo {
    // A ternary that produces `Optional<T>` is itself non-trivial; binding it
    // to a temp before `.is_some` / `.val` accesses avoids re-emitting both
    // ternary branches at the use-site.
    fn main(cond: bool, x: u32, y: u32, d: u32) -> u32 {
        let a: u32? = x;
        let b: u32? = y;
        let chosen: u32? = cond ? a : b;
        return chosen.unwrap_or(d);
    }
}
