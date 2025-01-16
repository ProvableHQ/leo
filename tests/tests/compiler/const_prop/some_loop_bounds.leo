/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    const START: u32 = 0u32;
    const STOP: u32 = 6u32;

    mapping m: u32 => u32;

    async function foo() {
        for i: u32 in START+1u32..STOP-2u32 {
            const SOME: u32 = i + 1u32;
            for j: u32 in SOME+1u32..STOP {
                m.set(i, j);
            }
        }
    }

    async transition main() -> Future {
        return foo();
    }
}
