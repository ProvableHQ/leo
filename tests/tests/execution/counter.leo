/*
namespace: Execute
expectation: Pass
initial_state:
    counter: []
cases:
    dubble:
    - input: []
    - input: []
    - input: []
    - input: []
*/

program test.aleo {
    mapping counter: address => u64;

    transition dubble() {
        return then finalize(self.caller);
    }

    finalize dubble(addr: address) {
        let current_value: u64 = Mapping::get_or_init(counter, addr, 0u64);
        Mapping::set(counter, addr, current_value + 1u64);
        current_value = Mapping::get(counter, addr);
        Mapping::set(counter, addr, current_value + 1u64);
    }

    transition unsafe_increment() {
        return then finalize(self.caller);
    }

    finalize unsafe_increment(addr: address) {
        let current_value: u64 = Mapping::get(counter, addr);
        Mapping::set(counter, addr, current_value + 1u64);
    }
}
