// Realistic single-file benchmark: routing and risk scoring for DEX-style pools.
// Written in current Leo syntax with top-level helper functions and program entry fns.

struct Pool {
    reserve_x: u64,
    reserve_y: u64,
    fee_bps: u16,
    depth: u64,
}

struct RouteParams {
    amount_in: u64,
    slippage_limit_bps: u64,
    risk_limit: u64,
    horizon: u32,
}

struct TradeWindow {
    w0: u64,
    w1: u64,
    w2: u64,
    w3: u64,
    w4: u64,
    w5: u64,
    w6: u64,
    w7: u64,
}

fn min_u64(a: u64, b: u64) -> u64 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn max_u64(a: u64, b: u64) -> u64 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn abs_diff(a: u64, b: u64) -> u64 {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

fn clamp_fee_bps(fee_bps: u16) -> u16 {
    if fee_bps > 1500u16 {
        return 1500u16;
    } else {
        return fee_bps;
    }
}

fn apply_fee(amount: u64, fee_bps: u16) -> (u64, u64) {
    let fee = (amount * fee_bps as u64) / 10_000u64;
    return (amount - fee, fee);
}

fn quote_constant_product(amount_in: u64, reserve_in: u64, reserve_out: u64, fee_bps: u16) -> u64 {
    let (effective_in, _) = apply_fee(amount_in, fee_bps);
    let numerator = effective_in * reserve_out;
    let denominator = reserve_in + effective_in;
    return numerator / denominator;
}

fn impact_bps(amount_in: u64, reserve_in: u64) -> u64 {
    if reserve_in == 0u64 {
        return 10_000u64;
    }
    return (amount_in * 10_000u64) / reserve_in;
}

fn liquidity_ratio(reserve_x: u64, reserve_y: u64) -> u64 {
    let hi = max_u64(reserve_x, reserve_y);
    let lo = min_u64(reserve_x, reserve_y);
    if lo == 0u64 {
        return 0u64;
    }
    return (hi * 10_000u64) / lo;
}

fn skew_penalty(reserve_x: u64, reserve_y: u64) -> u64 {
    return abs_diff(reserve_x, reserve_y) / 10u64;
}

fn window_pressure(window: TradeWindow) -> u64 {
    let lanes = [
        window.w0,
        window.w1,
        window.w2,
        window.w3,
        window.w4,
        window.w5,
        window.w6,
        window.w7,
    ];
    let acc = 0u64;
    for i: u8 in 0u8..8u8 {
        acc += lanes[i] / (i as u64 + 1u64);
    }
    return acc;
}

fn window_variance(window: TradeWindow) -> u64 {
    let lanes = [
        window.w0,
        window.w1,
        window.w2,
        window.w3,
        window.w4,
        window.w5,
        window.w6,
        window.w7,
    ];
    let low = lanes[0u8];
    let high = lanes[0u8];
    for i: u8 in 1u8..8u8 {
        if lanes[i] < low {
            low = lanes[i];
        }
        if lanes[i] > high {
            high = lanes[i];
        }
    }
    return high - low;
}

fn lane_score_00(pool: Pool, params: RouteParams) -> u64 {
    let out = quote_constant_product(
        params.amount_in,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(params.amount_in, pool.reserve_x);
    let ratio = liquidity_ratio(pool.reserve_x, pool.reserve_y);
    return out + ratio - impact;
}

fn lane_score_01(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 20u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(scaled, pool.reserve_x);
    let penalty = skew_penalty(pool.reserve_x, pool.reserve_y);
    return out + pool.depth - impact - penalty;
}

fn lane_score_02(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + (params.horizon as u64 * 3u64);
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(scaled, pool.reserve_x);
    return out + pool.reserve_y / 4u64 - impact;
}

fn lane_score_03(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + params.risk_limit / 100u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let ratio = liquidity_ratio(pool.reserve_x, pool.reserve_y);
    return out + ratio + pool.depth / 10u64;
}

fn lane_score_04(pool: Pool, params: RouteParams) -> u64 {
    let out = quote_constant_product(
        params.amount_in + 7u64,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(params.amount_in + 7u64, pool.reserve_x);
    let penalty = skew_penalty(pool.reserve_x, pool.reserve_y);
    return out + params.risk_limit / 10u64 - impact - penalty;
}

fn lane_score_05(pool: Pool, params: RouteParams) -> u64 {
    let out = quote_constant_product(
        params.amount_in + 11u64,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let ratio = liquidity_ratio(pool.reserve_x, pool.reserve_y);
    return out + ratio + pool.reserve_x / 20u64;
}

fn lane_score_06(pool: Pool, params: RouteParams) -> u64 {
    let out = quote_constant_product(
        params.amount_in + 13u64,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(params.amount_in + 13u64, pool.reserve_x);
    return out + pool.depth / 5u64 - impact;
}

fn lane_score_07(pool: Pool, params: RouteParams) -> u64 {
    let out = quote_constant_product(
        params.amount_in + 17u64,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let penalty = skew_penalty(pool.reserve_x, pool.reserve_y);
    return out + params.slippage_limit_bps - penalty;
}

fn lane_score_08(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 33u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(scaled, pool.reserve_x);
    let ratio = liquidity_ratio(pool.reserve_x, pool.reserve_y);
    return out + ratio + params.horizon as u64 - impact;
}

fn lane_score_09(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 25u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let penalty = skew_penalty(pool.reserve_x, pool.reserve_y);
    return out + pool.reserve_x / 16u64 - penalty;
}

fn lane_score_10(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 40u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(scaled, pool.reserve_x);
    return out + params.risk_limit / 20u64 - impact;
}

fn lane_score_11(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 45u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let ratio = liquidity_ratio(pool.reserve_x, pool.reserve_y);
    return out + ratio + params.slippage_limit_bps / 2u64;
}

fn lane_score_12(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 50u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(scaled, pool.reserve_x);
    let penalty = skew_penalty(pool.reserve_x, pool.reserve_y);
    return out + pool.depth / 7u64 - impact - penalty;
}

fn lane_score_13(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 55u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let ratio = liquidity_ratio(pool.reserve_x, pool.reserve_y);
    return out + ratio + params.horizon as u64;
}

fn lane_score_14(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 60u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let impact = impact_bps(scaled, pool.reserve_x);
    return out + params.slippage_limit_bps / 3u64 - impact;
}

fn lane_score_15(pool: Pool, params: RouteParams) -> u64 {
    let scaled = params.amount_in + pool.depth / 65u64;
    let out = quote_constant_product(
        scaled,
        pool.reserve_x,
        pool.reserve_y,
        clamp_fee_bps(pool.fee_bps),
    );
    let penalty = skew_penalty(pool.reserve_x, pool.reserve_y);
    return out + params.risk_limit / 30u64 - penalty;
}

fn route_total(pool: Pool, params: RouteParams) -> u64 {
    let lanes = [
        lane_score_00(pool, params),
        lane_score_01(pool, params),
        lane_score_02(pool, params),
        lane_score_03(pool, params),
        lane_score_04(pool, params),
        lane_score_05(pool, params),
        lane_score_06(pool, params),
        lane_score_07(pool, params),
        lane_score_08(pool, params),
        lane_score_09(pool, params),
        lane_score_10(pool, params),
        lane_score_11(pool, params),
        lane_score_12(pool, params),
        lane_score_13(pool, params),
        lane_score_14(pool, params),
        lane_score_15(pool, params),
    ];
    let acc = 0u64;
    for i: u8 in 0u8..16u8 {
        acc += lanes[i] / (i as u64 + 1u64);
    }
    return acc;
}

fn best_of_three(a: u64, b: u64, c: u64) -> u64 {
    if a >= b {
        if a >= c {
            return a;
        } else {
            return c;
        }
    } else {
        if b >= c {
            return b;
        } else {
            return c;
        }
    }
}

fn worst_of_three(a: u64, b: u64, c: u64) -> u64 {
    if a <= b {
        if a <= c {
            return a;
        } else {
            return c;
        }
    } else {
        if b <= c {
            return b;
        } else {
            return c;
        }
    }
}

fn stress_mix_00(a: u64, b: u64, c: u64) -> u64 {
    let lane = a + b + c;
    if lane % 2u64 == 0u64 {
        return lane / 2u64;
    } else {
        return lane + 19u64;
    }
}

fn stress_mix_01(a: u64, b: u64, c: u64) -> u64 {
    let lane = a * 2u64 + b + c;
    if lane % 3u64 == 0u64 {
        return lane / 3u64;
    } else {
        return lane + 23u64;
    }
}

fn stress_mix_02(a: u64, b: u64, c: u64) -> u64 {
    let lane = a + b * 2u64 + c;
    if lane % 5u64 == 0u64 {
        return lane / 5u64;
    } else {
        return lane + 29u64;
    }
}

fn stress_mix_03(a: u64, b: u64, c: u64) -> u64 {
    let lane = a + b + c * 2u64;
    if lane % 7u64 == 0u64 {
        return lane / 7u64;
    } else {
        return lane + 31u64;
    }
}

fn stress_mix_04(a: u64, b: u64, c: u64) -> u64 {
    let lane = a * 3u64 + b + c;
    if lane % 11u64 == 0u64 {
        return lane / 11u64;
    } else {
        return lane + 37u64;
    }
}

fn stress_mix_05(a: u64, b: u64, c: u64) -> u64 {
    let lane = a + b * 3u64 + c;
    if lane % 13u64 == 0u64 {
        return lane / 13u64;
    } else {
        return lane + 41u64;
    }
}

fn stress_mix_06(a: u64, b: u64, c: u64) -> u64 {
    let lane = a + b + c * 3u64;
    if lane % 17u64 == 0u64 {
        return lane / 17u64;
    } else {
        return lane + 43u64;
    }
}

fn stress_mix_07(a: u64, b: u64, c: u64) -> u64 {
    let lane = a * 4u64 + b + c;
    if lane % 19u64 == 0u64 {
        return lane / 19u64;
    } else {
        return lane + 47u64;
    }
}

program bench_control_flow_matrix.aleo {
    fn evaluate_route(
        pool_a: Pool,
        pool_b: Pool,
        pool_c: Pool,
        params: RouteParams,
        window: TradeWindow,
    ) -> u64 {
        let score_a = route_total(pool_a, params);
        let score_b = route_total(pool_b, params);
        let score_c = route_total(pool_c, params);
        let best = best_of_three(score_a, score_b, score_c);
        let worst = worst_of_three(score_a, score_b, score_c);
        let pressure = window_pressure(window);
        let variance = window_variance(window);
        let adjusted = best + pressure;
        let penalty = worst / 4u64 + variance;
        if adjusted > penalty {
            return adjusted - penalty;
        } else {
            return best / 2u64;
        }
    }

    fn matrix_probe(pool: Pool, params: RouteParams) -> u64 {
        let lanes = [
            lane_score_00(pool, params),
            lane_score_01(pool, params),
            lane_score_02(pool, params),
            lane_score_03(pool, params),
            lane_score_04(pool, params),
            lane_score_05(pool, params),
            lane_score_06(pool, params),
            lane_score_07(pool, params),
            lane_score_08(pool, params),
            lane_score_09(pool, params),
            lane_score_10(pool, params),
            lane_score_11(pool, params),
            lane_score_12(pool, params),
            lane_score_13(pool, params),
            lane_score_14(pool, params),
            lane_score_15(pool, params),
        ];
        let acc = 0u64;
        for i: u8 in 0u8..16u8 {
            if i % 2u8 == 0u8 {
                acc += lanes[i] / (i as u64 + 1u64);
            } else {
                acc += lanes[i] / (i as u64 + 2u64);
            }
        }
        let floor = params.slippage_limit_bps * 10u64;
        let ceiling = params.risk_limit * 5u64;
        if acc < floor {
            return floor;
        }
        if acc > ceiling {
            return ceiling;
        }
        return acc;
    }
}
