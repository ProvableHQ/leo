/*
seed = 123456789

[case]
program = "storage_pop_ternary.aleo"
function = "true_arm_unselected"
input = ["false"]

[case]
program = "storage_pop_ternary.aleo"
function = "true_arm_selected"
input = ["true"]

[case]
program = "storage_pop_ternary.aleo"
function = "false_arm_unselected"
input = ["true"]

[case]
program = "storage_pop_ternary.aleo"
function = "false_arm_selected"
input = ["false"]

[case]
program = "storage_pop_ternary.aleo"
function = "assigned_unselected"
input = ["false"]

[case]
program = "storage_pop_ternary.aleo"
function = "nested_unselected"
input = ["true", "false"]

[case]
program = "storage_pop_ternary.aleo"
function = "nested_selected"
input = ["true", "true"]

[case]
program = "storage_pop_ternary.aleo"
function = "compound_unselected"
input = ["true", "false"]

[case]
program = "storage_pop_ternary.aleo"
function = "compound_selected"
input = ["true", "true"]

[case]
program = "storage_pop_ternary.aleo"
function = "both_arms_true"
input = ["true"]

[case]
program = "storage_pop_ternary.aleo"
function = "both_arms_false"
input = ["false"]
*/

program storage_pop_ternary.aleo {
    storage true_unselected: [u32];
    storage true_selected: [u32];
    storage false_unselected: [u32];
    storage false_selected: [u32];
    storage assigned: [u32];
    storage nested: [u32];
    storage nested_selected_values: [u32];
    storage compound: [u32];
    storage compound_selected_values: [u32];
    storage both_true_values: [u32];
    storage both_false_values: [u32];

    fn true_arm_unselected(public take: bool) -> Final {
        return final {
            true_unselected.push(1u32);
            let popped: u32? = take ? true_unselected.pop() : none;
            assert(popped == none);
            assert(true_unselected.len() == 1u32);
        };
    }

    fn true_arm_selected(public take: bool) -> Final {
        return final {
            true_selected.push(2u32);
            let popped: u32? = take ? true_selected.pop() : none;
            assert(popped != none);
            assert(popped.unwrap() == 2u32);
            assert(true_selected.len() == 0u32);
        };
    }

    fn false_arm_unselected(public take: bool) -> Final {
        return final {
            false_unselected.push(3u32);
            let popped: u32? = take ? none : false_unselected.pop();
            assert(popped == none);
            assert(false_unselected.len() == 1u32);
        };
    }

    fn false_arm_selected(public take: bool) -> Final {
        return final {
            false_selected.push(7u32);
            let popped: u32? = take ? none : false_selected.pop();
            assert(popped != none);
            assert(popped.unwrap() == 7u32);
            assert(false_selected.len() == 0u32);
        };
    }

    fn assigned_unselected(public take: bool) -> Final {
        return final {
            assigned.push(4u32);
            let popped: u32? = none;
            popped = take ? assigned.pop() : none;
            assert(popped == none);
            assert(assigned.len() == 1u32);
        };
    }

    fn nested_unselected(public outer: bool, public inner: bool) -> Final {
        return final {
            nested.push(5u32);
            let popped: u32? = outer ? (inner ? nested.pop() : none) : none;
            assert(popped == none);
            assert(nested.len() == 1u32);
        };
    }

    fn nested_selected(public outer: bool, public inner: bool) -> Final {
        return final {
            nested_selected_values.push(8u32);
            let popped: u32? = outer ? (inner ? nested_selected_values.pop() : none) : none;
            assert(popped != none);
            assert(popped.unwrap() == 8u32);
            assert(nested_selected_values.len() == 0u32);
        };
    }

    fn compound_unselected(public first: bool, public second: bool) -> Final {
        return final {
            compound.push(6u32);
            let popped: u32? = (first && second) ? compound.pop() : none;
            assert(popped == none);
            assert(compound.len() == 1u32);
        };
    }

    fn compound_selected(public first: bool, public second: bool) -> Final {
        return final {
            compound_selected_values.push(9u32);
            let popped: u32? = (first && second) ? compound_selected_values.pop() : none;
            assert(popped != none);
            assert(popped.unwrap() == 9u32);
            assert(compound_selected_values.len() == 0u32);
        };
    }

    fn both_arms_true(public choose_first: bool) -> Final {
        return final {
            both_true_values.push(10u32);
            both_true_values.push(11u32);
            let popped: u32? = choose_first ? both_true_values.pop() : both_true_values.pop();
            assert(popped != none);
            assert(popped.unwrap() == 11u32);
            assert(both_true_values.len() == 1u32);
        };
    }

    fn both_arms_false(public choose_first: bool) -> Final {
        return final {
            both_false_values.push(12u32);
            both_false_values.push(13u32);
            let popped: u32? = choose_first ? both_false_values.pop() : both_false_values.pop();
            assert(popped != none);
            assert(popped.unwrap() == 13u32);
            assert(both_false_values.len() == 1u32);
        };
    }

    @noupgrade
    constructor() {}
}
