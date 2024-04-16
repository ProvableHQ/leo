
/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    mapping Yo: u32 => u32;

    async transition main() -> Future {
        let f: Future = finalize_main(1u32, 1u32);
        return f;
    }

    async function finalize_main(a:u32, b:u32) {
        Mapping::set(Yo, a, b);
    }
}