
/*
namespace: Compile
expectation: Fail
*/


program test.aleo {

    function one(n: u8) -> u8 {
        return two(n);
    }

    function two(n: u8) -> u8 {
        return three(n) + four(n);
    }

    function three(n: u8) -> u8 {
        return one(n);
    }

    function four(n: u8) -> u8 {
        return one(n);
    }

    transition five(n: u8) -> u8 {
        return six(n);
    }

    transition six(n: u8) -> u8 {
        return seven(n) + eight(n);
    }

    transition seven(n: u8) -> u8 {
        return five(n);
    }

    transition eight(n: u8) -> u8 {
        return five(n);
    }
}
