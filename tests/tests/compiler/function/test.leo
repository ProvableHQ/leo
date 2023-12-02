/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    struct message {
        sender: u8,
        object: u8,
    }
    mapping balances: u8 => u8;

    transition a(arg1: message) -> message {
        return message {
            sender: 1u8,
            object: 1u8,
        };
    }

    function b(arg1: message) -> message {
        return message {
            sender: 1u8,
            object: 1u8,
        };
    }

    function c(arg1: balances) -> balances {
        return arg1;
    }

    transition d(arg1: balances) -> balances {
        return arg1;
    }
}
