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

use snarkvm_gadgets::traits::utilities::{
    alloc::AllocGadget,
    arithmetic::{Add, Div, Mul, Sub},
    int::{Int128, Int16, Int32, Int64, Int8},
};
use snarkvm_r1cs::{ConstraintSystem, Fr, TestConstraintSystem};

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::i128;

use criterion::{criterion_group, criterion_main, Criterion};

// TODO: move these benchmarks to snarkvm?
macro_rules! create_add_bench {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if a.checked_add(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: a (add)", bench_run_id)), || Ok(a)).unwrap();
                let b_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: b (add)", bench_run_id)), || Ok(b)).unwrap();

                a_bit
                    .add(cs.ns(|| format!("{}: a add b", bench_run_id)), &b_bit)
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_add_bench_const {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if a.checked_add(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit_const = <$bit_type>::constant(a);
                let b_bit_const = <$bit_type>::constant(b);
                a_bit_const
                    .add(
                        cs.ns(|| format!("{}: a add b: const", bench_run_id)),
                        &b_bit_const,
                    )
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_sub_bench {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if b.checked_neg().is_none() || a.checked_sub(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: a (sub)", bench_run_id)), || Ok(a)).unwrap();
                let b_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: b (sub)", bench_run_id)), || Ok(b)).unwrap();

                a_bit
                    .sub(cs.ns(|| format!("{}: a sub b", bench_run_id)), &b_bit)
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_sub_bench_const {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if b.checked_neg().is_none() || a.checked_sub(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit_const = <$bit_type>::constant(a);
                let b_bit_const = <$bit_type>::constant(b);
                a_bit_const
                    .sub(
                        cs.ns(|| format!("{}: a sub b: const", bench_run_id)),
                        &b_bit_const,
                    )
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_mul_bench {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if a.checked_mul(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: a (mul)", bench_run_id)), || Ok(a)).unwrap();
                let b_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: b (mul)", bench_run_id)), || Ok(b)).unwrap();
                a_bit
                    .mul(cs.ns(|| format!("{}: a mul b", bench_run_id)), &b_bit)
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_mul_bench_const {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if a.checked_mul(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit_const = <$bit_type>::constant(a);
                let b_bit_const = <$bit_type>::constant(b);
                a_bit_const
                    .mul(
                        cs.ns(|| format!("{}: a mul b: const", bench_run_id)),
                        &b_bit_const,
                    )
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_div_bench {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if a.checked_neg().is_none() || a.checked_div(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: a (div)", bench_run_id)), || Ok(a)).unwrap();
                let b_bit = <$bit_type>::alloc(cs.ns(|| format!("{}: b (div)", bench_run_id)), || Ok(b)).unwrap();
                a_bit
                    .div(cs.ns(|| format!("{}: a div b", bench_run_id)), &b_bit)
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

macro_rules! create_div_bench_const {
    ($bench_name:ident, $bench_id:expr, $foo_name:ident, $std_type:ty, $bit_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            fn $foo_name(cs: &mut TestConstraintSystem<Fr>, rng: &mut XorShiftRng) {
                let a: $std_type = rng.gen();
                let b: $std_type = rng.gen();

                if a.checked_neg().is_none() || a.checked_div(b).is_none() {
                    return;
                }

                let bench_run_id: u64 = rng.gen();

                let a_bit_const = <$bit_type>::constant(a);
                let b_bit_const = <$bit_type>::constant(b);
                a_bit_const
                    .div(
                        cs.ns(|| format!("{}: a div b: const", bench_run_id)),
                        &b_bit_const,
                    )
                    .unwrap();
            }

            let mut cs = TestConstraintSystem::<Fr>::new();

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            c.bench_function(&format!("integer_arithmetic::{}", $bench_id), |b| {
                b.iter(|| $foo_name(&mut cs, &mut rng))
            });
        }
    };
}

create_add_bench!(bench_i8_add, "i8_add", i8_add, i8, Int8);
create_add_bench!(bench_i16_add, "i16_add", i16_add, i16, Int16);
create_add_bench!(bench_i32_add, "i32_add", i32_add, i32, Int32);
create_add_bench!(bench_i64_add, "i64_add", i64_add, i64, Int64);
create_add_bench!(bench_i128_add, "i128_add", i128_add, i128, Int128);

create_add_bench_const!(bench_i8_add_const, "i8_add_const", i8_add, i8, Int8);
create_add_bench_const!(bench_i16_add_const, "i16_add_const", i16_add, i16, Int16);
create_add_bench_const!(bench_i32_add_const, "i32_add_const", i32_add, i32, Int32);
create_add_bench_const!(bench_i64_add_const, "i64_add_const", i64_add, i64, Int64);
create_add_bench_const!(bench_i128_add_const, "i128_add_const", i128_add, i128, Int128);

create_sub_bench!(bench_i8_sub, "i8_sub", i8_sub, i8, Int8);
create_sub_bench!(bench_i16_sub, "i16_sub", i16_sub, i16, Int16);
create_sub_bench!(bench_i32_sub, "i32_sub", i32_sub, i32, Int32);
create_sub_bench!(bench_i64_sub, "i64_sub", i64_sub, i64, Int64);
create_sub_bench!(bench_i128_sub, "i128_sub", i128_sub, i128, Int128);

create_sub_bench_const!(bench_i8_sub_const, "i8_sub_const", i8_sub, i8, Int8);
create_sub_bench_const!(bench_i16_sub_const, "i16_sub_const", i16_sub, i16, Int16);
create_sub_bench_const!(bench_i32_sub_const, "i32_sub_const", i32_sub, i32, Int32);
create_sub_bench_const!(bench_i64_sub_const, "i64_sub_const", i64_sub, i64, Int64);
create_sub_bench_const!(bench_i128_sub_const, "i128_sub_const", i128_sub, i128, Int128);

create_mul_bench!(bench_i8_mul, "i8_mul", i8_mul, i8, Int8);
create_mul_bench!(bench_i16_mul, "i16_mul", i16_mul, i16, Int16);
create_mul_bench!(bench_i32_mul, "i32_mul", i32_mul, i32, Int32);
create_mul_bench!(bench_i64_mul, "i64_mul", i64_mul, i64, Int64);
create_mul_bench!(bench_i128_mul, "i128_mul", i128_mul, i128, Int128);

create_mul_bench_const!(bench_i8_mul_const, "i8_mul_const", i8_mul, i8, Int8);
create_mul_bench_const!(bench_i16_mul_const, "i16_mul_const", i16_mul, i16, Int16);
create_mul_bench_const!(bench_i32_mul_const, "i32_mul_const", i32_mul, i32, Int32);
create_mul_bench_const!(bench_i64_mul_const, "i64_mul_const", i64_mul, i64, Int64);
create_mul_bench_const!(bench_i128_mul_const, "i128_mul_const", i128_mul, i128, Int128);

create_div_bench!(bench_i8_div, "i8_div", i8_div, i8, Int8);
create_div_bench!(bench_i16_div, "i16_div", i16_div, i16, Int16);
create_div_bench!(bench_i32_div, "i32_div", i32_div, i32, Int32);
// create_div_bench!(bench_i64_div, "i64_div", i64_div, i64, Int64);
// create_div_bench!(bench_i128_div, "i128_div", i128_div, i128, Int128);

create_div_bench_const!(bench_i8_div_const, "i8_div_const", i8_div, i8, Int8);
create_div_bench_const!(bench_i16_div_const, "i16_div_const", i16_div, i16, Int16);
create_div_bench_const!(bench_i32_div_const, "i32_div_const", i32_div, i32, Int32);
// create_div_bench_const!(bench_i64_div_const, "i64_div_const", i64_div, i64, Int64);
// create_div_bench_const!(bench_i128_div_const, "i128_div_const", i128_div, i128, Int128);

criterion_group!(
    name = benches_add;
    config = Criterion::default();
    targets = bench_i8_add,
    bench_i16_add,
    bench_i32_add,
    bench_i64_add,
    bench_i128_add,
);

criterion_group!(
    name = benches_add_const;
    config = Criterion::default();
    targets = bench_i8_add_const,
    bench_i16_add_const,
    bench_i32_add_const,
    bench_i64_add_const,
    bench_i128_add_const,
);

criterion_group!(
    name = benches_sub;
    config = Criterion::default();
    targets = bench_i8_sub,
    bench_i16_sub,
    bench_i32_sub,
    bench_i64_sub,
    bench_i128_sub,
);

criterion_group!(
    name = benches_sub_const;
    config = Criterion::default();
    targets = bench_i8_sub_const,
    bench_i16_sub_const,
    bench_i32_sub_const,
    bench_i64_sub_const,
    bench_i128_sub_const,
);

criterion_group!(
    name = benches_mul;
    config = Criterion::default();
    targets = bench_i8_mul,
    bench_i16_mul,
    bench_i32_mul,
    bench_i64_mul,
    bench_i128_mul,
);

criterion_group!(
    name = benches_mul_const;
    config = Criterion::default();
    targets = bench_i8_mul_const,
    bench_i16_mul_const,
    bench_i32_mul_const,
    bench_i64_mul_const,
    bench_i128_mul_const,
);

criterion_group!(
    name = benches_div;
    config = Criterion::default();
    targets = bench_i8_div,
    bench_i16_div,
    bench_i32_div,
    // bench_i64_div,
    // bench_i128_div,
);

criterion_group!(
    name = benches_div_const;
    config = Criterion::default();
    targets = bench_i8_div_const,
    bench_i16_div_const,
    bench_i32_div_const,
    // bench_i64_div_const,
    // bench_i128_div_const,
);

criterion_main!(
    benches_add,
    benches_add_const,
    benches_sub,
    benches_sub_const,
    benches_mul,
    benches_mul_const,
    benches_div,
    benches_div_const
);
