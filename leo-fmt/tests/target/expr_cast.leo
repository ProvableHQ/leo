program test.aleo {
    function main(a: u64) -> u32 {
        let b: u32 = a as u32;
        let c: field = a as field;
        return b;
    }
}
