
program test.aleo {
    mapping Yo: u32 => u32;

    fn main() -> Final {
        let f: Final = final { finalize_main(1u32, 1u32); };
        return f;
    }
}

final fn finalize_main(a:u32, b:u32) -> Final {
    Mapping::set(Yo, a, b);
}
