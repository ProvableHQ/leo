// Test: an interface prototype declares an optional return type (`u32?`). When an
// entry point function implements that signature, the type checker rejects it because
// entry point fns cannot return optional types. The failure is a ETYC error from
// the type checker, not an ECHI error from the interface checker.

interface Finder {
    fn find(key: u32) -> u32?;
}

program test.aleo: Finder {
    fn find(key: u32) -> u32? {
        let result: u32? = 42u32;
        return result;
    }

    fn main(key: u32) -> u32 {
        return find(key).unwrap();
    }
}
