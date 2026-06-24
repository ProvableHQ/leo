// Positive coverage for `final fn` declarations with generic const parameters.
// `final fn`s are always inlined into their `final {}` call site
// (`function_inlining/transform.rs:278`), so monomorphization can specialize
// the body for each concrete const argument at the call site. The patterns
// below exercise the cases the language is expected to handle:
//
// 1. A `final fn` with a single u32 const generic, invoked from a `final {}`.
// 2. A `final fn` with an `address` const generic.
// 3. A `final fn` calling another `final fn` with const generics (both inline).
// 4. A `final fn` threading its own const generic into a generic callee.
// 5. Multiple const generic parameters on a single `final fn`.

final fn scaled::[N: u32](x: u32) -> u32 {
    return x + N;
}

final fn for_program::[PROG: address]() -> address {
    return PROG;
}

final fn double_scaled::[N: u32](x: u32) -> u32 {
    return scaled::[N](x) + scaled::[N](x);
}

final fn forwarded::[A: u32](x: u32) -> u32 {
    return scaled::[A](x);
}

final fn shifted::[N: u32, M: u32](x: u32) -> u32 {
    return scaled::[N](x) + M;
}

program test.aleo {
    fn run() -> Final {
        return final {
            assert_eq(scaled::[5u32](10u32), 15u32);
            assert_eq(double_scaled::[3u32](7u32), 20u32);
            assert_eq(forwarded::[2u32](8u32), 10u32);
            assert_eq(shifted::[4u32, 6u32](1u32), 11u32);
            assert_eq(for_program::[test.aleo](), test.aleo);
        };
    }

    @noupgrade
    constructor() {}
}
