/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "increment"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[case]
program = "test.aleo"
function = "increment"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[case]
program = "test.aleo"
function = "increment"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[case]
program = "test.aleo"
function = "increment"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program test.aleo {
    storage counter: u32;

    async transition initialize() -> Future {
        return async {
            counter = 0;
        };
    }

    async transition increment() -> Future {
        return async {
            let old_counter_value = counter.unwrap_or(0);
            counter = old_counter_value + 1u32;
            assert(counter.unwrap() == old_counter_value + 1u32);
        };
    }

    @noupgrade
    async constructor() {}
}

