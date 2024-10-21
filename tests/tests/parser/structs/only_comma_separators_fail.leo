/*
namespace = "Parse"
expectation = "Fail"
*/
program only_comma_separators.aleo {
    struct semicolons {
        x: u8;
        y: u8
    }

    struct semicolons_with_trailing {
        x: u8;
        y: u8;
    }

    transition main(public a: u32) -> u32 {
		return a;
    }
}
