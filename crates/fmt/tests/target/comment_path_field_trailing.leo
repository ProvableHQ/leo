program test.aleo {
    @noupgrade
    constructor() {}

    struct SwapHop {
        pool: field,
        sqrt_price_limit: swap_math::U256, // price limit for this hop
    }
}
