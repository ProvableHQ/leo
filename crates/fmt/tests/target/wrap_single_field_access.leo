program test.aleo {
    @noupgrade
    constructor() {}

    fn compute(use_lower_boundary: bool) -> u128 {
        let selected_growth_value: u128 = use_lower_boundary ? lower_tick_record.fee_growth_global_value_x : fallback_growth_amount;
        return selected_growth_value;
    }
}
