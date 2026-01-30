import hello.aleo;
final fn finalize_main() {
    Mapping::set(hello.aleo/counter, addr, current_value + 1u64);
    Mapping::remove(hello.aleo/foo, addr);
    let a:u32 = Mapping::get(hello.aleo/foo, addr);
    let b:u32 = Mapping::get_or_use(hello.aleo/foo, addr, 0u64);
    let c:u32 = Mapping::contains(hello.aleo/foo, addr);
}
program test.aleo {
    fn main() {
        let x:u8 = 1u8;
        return;
    }
}
