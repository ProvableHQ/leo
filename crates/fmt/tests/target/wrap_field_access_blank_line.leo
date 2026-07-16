program test.aleo {
    @noupgrade
    constructor() {}

    fn choose() -> i32 {
        let chosen_field: i32 = predicate_flag_a1 && operand_value_lhs == operand_rhs ? record_value_x.next : default_operand_z;

        return chosen_field;
    }
}
