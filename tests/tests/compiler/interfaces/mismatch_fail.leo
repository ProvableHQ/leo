
interface AbstractProgram {
    fn foo() -> u64;
    fn bar(a: u64);
    fn baz(a: u64, b: u8, c: u32) -> Final;

}


program test.aleo : AbstractProgram {
    fn foo() -> u32 {
        return 0;
    }
    fn bar(b: u64) {
        return 0;
    }
    fn baz(a: u32, b: u8, c: u32) {}
}
