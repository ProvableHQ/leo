// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_gadgets::{arithmetic::*, Int8};

use snarkvm_fields::{One, Zero};
use snarkvm_gadgets::traits::utilities::{
    alloc::AllocGadget,
    arithmetic::{Add, Div, Mul, Neg, Pow},
    boolean::Boolean,
};
use snarkvm_r1cs::{ConstraintSystem, Fr, TestConstraintSystem};

use rand::Rng;
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::i8;

fn check_all_constant_bits(expected: i8, actual: Int8) {
    for (i, b) in actual.bits.iter().enumerate() {
        // shift value by i
        let mask = 1 << i as i8;
        let result = expected & mask;

        match *b {
            Boolean::Is(_) => panic!(),
            Boolean::Not(_) => panic!(),
            Boolean::Constant(b) => {
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

        match *b {
            Boolean::Is(ref b) => {
                let bit = result == mask;
                assert_eq!(b.get_value().unwrap(), bit);
            }
            Boolean::Not(ref b) => {
                let bit = result == mask;
                assert_eq!(!b.get_value().unwrap(), bit);
            }
            Boolean::Constant(_) => unreachable!(),
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

        let expected = match a.checked_add(b) {
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

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

    for _ in 0..100 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen_range(-4..4);
        let b: i8 = rng.gen_range(-4..4);

        let expected = match a.checked_pow(b as u32) {
            Some(valid) => valid,
            None => continue,
        };

        let a_bit = Int8::constant(a);
        let b_bit = Int8::constant(b);

        let r = a_bit.pow(cs.ns(|| "exponentiation"), &b_bit).unwrap();

        assert!(r.value == Some(expected));

        check_all_constant_bits(expected, r);
    }
}

#[test]
fn test_int8_pow() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let mut cs = TestConstraintSystem::<Fr>::new();

        let a: i8 = rng.gen_range(-4..4);
        let b: i8 = rng.gen_range(-4..4);

        let expected = match a.checked_pow(b as u32) {
            Some(valid) => valid,
            None => continue,
        };

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
}
