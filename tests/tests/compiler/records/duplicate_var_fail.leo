
program test.aleo {
    // test 1: adjacent duplicate - no "..." gap marker
    record Token {
        owner: address,
        owner: address, // Cannot define two record variables with the same name.
        amount: u64,
    }

    // test 2: duplicate with gap - "..." marker
    record Asset {
        owner: address,
        val: u64,
        description: field,
        owner: address,
    }

    // test 3: multiple different duplicates
    record Data {
        owner: address,
        amount: u64,
        owner: address,
        amount: u64,
    }

    function main(zz: bool) -> bool {
        return true;
    }}
