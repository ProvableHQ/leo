
program test.aleo {
    mapping Yo: u32 => u32;

    async fn main() -> Future {
        let f: Future = finalize_main(1u32, 1u32);
        return f;
    }
}

async fn finalize_main(a:u32, b:u32) {
    Mapping::set(Yo, a, b);
}
