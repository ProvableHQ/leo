use leo_gadgets::{arithmetic::*, errors::SignedIntegerError, Int8};

use snarkos_models::{
    curves::{One, Zero},
    gadgets::{
        r1cs::{ConstraintSystem, Fr, TestConstraintSystem},
        utilities::{alloc::AllocGadget, boolean::Boolean},
    },
};

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::i8;

fn check_all_constant_bits(expected: i8, actual: Int8) {
    for (i, b) in actual.bits.iter().enumerate() {
        // shift value by i
        let mask = 1 << i as i8;
        let result = expected & mask;

        match b {
            &Boolean::Is(_) => panic!(),
            &Boolean::Not(_) => panic!(),
            &Boolean::Constant(b) => {
                let bit = result == mask;
                assert_eq!(b, bit);
            }
        }
    }
}

fn check_all_allocated_bits(expected: i8, actual: Int8) {
    for (i, b) in actual.bits.iter().enumerate() {
        // shift value by i
        let mask = 1 << i as i8;
        let result = expected & mask;

        match b {
            &Boolean::Is(ref b) => {
                let bit = result == mask;
                assert_eq!(b.get_value().unwrap(), bit);
            }
            &Boolean::Not(ref b) => {
                let bit = result == mask;
                assert_eq!(!b.get_value().unwrap(), bit);
            }
            &Boolean::Constant(_) => unreachable!(),
        }
    }
}

#[test]
fn test_int8_constant_and_alloc() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();

        let a_const = Int8::constant(a);

        assert!(a_const.value == Some(a));

        check_all_constant_bits(a, a_const);

        let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();

        assert!(cs.is_satisfied());
        assert!(a_bit.value == Some(a));

        check_all_allocated_bits(a, a_bit);
    }
}

#[test]
fn test_int8_add_constants() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

        let expected = match a.checked_add(b) {
            Some(valid) => valid,
            None => continue,
        };

        let r = a_bit.add(cs.ns(|| "addition"), &b_bit).unwrap();

        assert!(r.value == Some(expected));

        check_all_constant_bits(expected, r);
    }
}

#[test]
fn test_int8_add() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        let expected = match a.checked_add(b) {
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
        let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

        let r = a_bit.add(cs.ns(|| "addition"), &b_bit).unwrap();

        assert!(cs.is_satisfied());

        assert!(r.value == Some(expected));

        check_all_allocated_bits(expected, r);

        // Flip a bit_gadget and see if the addition constraint still works
        if cs.get("addition/result bit_gadget 0/boolean").is_zero() {
            cs.set("addition/result bit_gadget 0/boolean", Fr::one());
        } else {
            cs.set("addition/result bit_gadget 0/boolean", Fr::zero());
        }

        assert!(!cs.is_satisfied());
    }
}

#[test]
fn test_int8_sub_constants() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        if b.checked_neg().is_none() {
            // negate with overflows will fail: -128
            continue;
        }
        let expected = match a.checked_sub(b) {
            // subtract with overflow will fail: -0
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

        let r = a_bit.sub(cs.ns(|| "subtraction"), &b_bit).unwrap();

        assert!(r.value == Some(expected));

        check_all_constant_bits(expected, r);
    }
}

#[test]
fn test_int8_sub() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        if b.checked_neg().is_none() {
            // negate with overflows will fail: -128
            continue;
        }
        let expected = match a.checked_sub(b) {
            // subtract with overflow will fail: -0
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
        let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

        let r = a_bit.sub(cs.ns(|| "subtraction"), &b_bit).unwrap();

        assert!(cs.is_satisfied());

        assert!(r.value == Some(expected));

        check_all_allocated_bits(expected, r);

        // Flip a bit_gadget and see if the subtraction constraint still works
        if cs
            .get("subtraction/add_complement/result bit_gadget 0/boolean")
            .is_zero()
        {
            cs.set("subtraction/add_complement/result bit_gadget 0/boolean", Fr::one());
        } else {
            cs.set("subtraction/add_complement/result bit_gadget 0/boolean", Fr::zero());
        }

        assert!(!cs.is_satisfied());
    }
}

#[test]
fn test_int8_mul_constants() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        let expected = match a.checked_mul(b) {
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

        let r = a_bit.mul(cs.ns(|| "multiplication"), &b_bit).unwrap();

        assert!(r.value == Some(expected));

        check_all_constant_bits(expected, r);
    }
}

#[test]
fn test_int8_mul() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        let expected = match a.checked_mul(b) {
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
        let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

        let r = a_bit.mul(cs.ns(|| "multiplication"), &b_bit).unwrap();

        assert!(cs.is_satisfied());

        assert!(r.value == Some(expected));

        check_all_allocated_bits(expected, r);

        // Flip a bit_gadget and see if the multiplication constraint still works
        if cs.get("multiplication/result bit_gadget 0/boolean").is_zero() {
            cs.set("multiplication/result bit_gadget 0/boolean", Fr::one());
        } else {
            cs.set("multiplication/result bit_gadget 0/boolean", Fr::zero());
        }

        assert!(!cs.is_satisfied());
    }
}

#[test]
fn test_int8_div_constants() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..1000 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        if a.checked_neg().is_none() {
            return;
        }

        let expected = match a.checked_div(b) {
            Some(valid) => valid,
            None => return,
        };

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

        let r = a_bit.div(cs.ns(|| "division"), &b_bit).unwrap();

        assert!(r.value == Some(expected));

        check_all_constant_bits(expected, r);
    }
}

#[test]
fn test_int8_div() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..100 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        if a.checked_neg().is_none() {
            continue;
        }

        let expected = match a.checked_div(b) {
            Some(valid) => valid,
            None => return,
        };

        let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
        let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

        let r = a_bit.div(cs.ns(|| "division"), &b_bit).unwrap();

        assert!(cs.is_satisfied());

        assert!(r.value == Some(expected));

        check_all_allocated_bits(expected, r);
    }
}

#[test]
fn test_int8_pow_constants() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        // Test small ranges that we know won't overflow
        let a: i8 = rng.gen_range(-4, 4);
        let b: i8 = rng.gen_range(0, 4);

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

        let expected = match a.checked_pow(b as u32) {
            Some(valid) => valid,
            None => continue,
        };

        let r = a_bit.pow(cs.ns(|| "exponentiation"), &b_bit).unwrap();

        assert!(r.value == Some(expected));

        // Make sure we have not allocated any variables
        check_all_constant_bits(expected, r);
    }
}

fn test_int8_pow(a: i8, b: i8, expected: i8) {
    let mut cs = TestConstraintSystem::<Fr>::new();

    let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
    let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

    let r = a_bit.pow(cs.ns(|| "exponentiation"), &b_bit).unwrap();

    assert!(cs.is_satisfied());

    assert!(r.value == Some(expected));

    check_all_allocated_bits(expected, r);

    // Flip a bit_gadget and see if the exponentiation constraint still works
    if cs
        .get("exponentiation/multiply_by_self_0/result bit_gadget 0/boolean")
        .is_zero()
    {
        cs.set(
            "exponentiation/multiply_by_self_0/result bit_gadget 0/boolean",
            Fr::one(),
        );
    } else {
        cs.set(
            "exponentiation/multiply_by_self_0/result bit_gadget 0/boolean",
            Fr::zero(),
        );
    }

    assert!(!cs.is_satisfied());
}

fn expect_overflow(a: i8, b: i8) {
    let mut cs = TestConstraintSystem::<Fr>::new();

    let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
    let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

    match a_bit.pow(cs.ns(|| "exponentiation"), &b_bit) {
        Err(SignedIntegerError::Overflow) => {}
        Err(err) => panic!("expected overflow error, found error {}", err),
        Ok(res) => panic!("expected overflow error, found result {}", res.value.unwrap()),
    }
}

fn expect_underflow(a: i8, b: i8) {
    let mut cs = TestConstraintSystem::<Fr>::new();

    let a_bit = Int8::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
    let b_bit = Int8::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

    match a_bit.pow(cs.ns(|| "exponentiation"), &b_bit) {
        Err(SignedIntegerError::Underflow) => {}
        Err(err) => panic!("expected underflow error, found error {}", err),
        Ok(res) => panic!("expected underflow error, found result {}", res.value.unwrap()),
    }
}

#[test]
fn test_int8_pow_min_edge_cases() {
    let min = -128i8;

    // -128 ** 0 = 1
    test_int8_pow(min, 0, 1);

    // -128 ** 1 = -128
    test_int8_pow(min, 1, min);

    // -128 ** 2 = overflow_error
    expect_overflow(min, 2);

    // -2 ** 7 = -128
    test_int8_pow(-2, 7, min);

    // 0 ** 0 = 1
    test_int8_pow(0, 0, 1);
}

#[test]
fn test_int8_pow_max_edge_cases() {
    let max = 127i8;

    // 127 ** 0 = 1
    test_int8_pow(max, 0, 1);

    // 127 ** 1 = 127
    test_int8_pow(max, 1, max);

    // 127 ** 2 = overflow_error
    expect_overflow(max, 2);

    // 1 ** 127 = 1
    test_int8_pow(1, max, 1);

    // 0 ** 127 = 0
    test_int8_pow(0, max, 0);

    // 2 ** 6 = 64
    test_int8_pow(2, 6, 64);

    // 2 ** 7 = overflow_error
    expect_overflow(2, 7);
}

#[test]
fn test_int8_underflow() {
    // -11 ** 2 = 121
    test_int8_pow(-11, 2, 121);

    // -11 ** 3 = underflow error
    expect_underflow(-11, 3);
}

#[test]
fn test_int8_pow_random_small() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        // Test small ranges that we know won't overflow
        let a: i8 = rng.gen_range(-4, 4);
        let b: i8 = rng.gen_range(0, 4);

        let expected = match a.checked_pow(b as u32) {
            Some(valid) => valid,
            None => continue,
        };

        test_int8_pow(a, b, expected);
    }
}

#[test]
fn test_int8_pow_random_all() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..100 {
        let a: i8 = rng.gen();
        let b: i8 = rng.gen();

        let expected = match a.checked_pow(b as u32) {
            Some(valid) => valid,
            None => continue,
        };

        test_int8_pow(a, b, expected);
    }
}
