/*
namespace: Parse
expectation: Fail
*/
program comma_separators.aleo {
    struct none {
        x: u8
        y: u8
    }

    transition main(public a: u32) -> u32 {
		return a;
    }
}
