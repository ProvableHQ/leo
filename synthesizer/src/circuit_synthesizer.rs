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

use snarkvm_curves::traits::PairingEngine;
use snarkvm_fields::Field;
use snarkvm_r1cs::{ConstraintSystem, Index, LinearCombination, OptionalVec, SynthesisError, Variable};

#[derive(Default)]
pub struct Namespace {
    pub constraint_indices: Vec<usize>,
    pub public_var_indices: Vec<usize>,
    pub private_var_indices: Vec<usize>,
}

pub struct ConstraintSet<E: PairingEngine> {
    pub at: Vec<(E::Fr, Index)>,
    pub bt: Vec<(E::Fr, Index)>,
    pub ct: Vec<(E::Fr, Index)>,
}

impl<E: PairingEngine> Default for ConstraintSet<E> {
    fn default() -> Self {
        ConstraintSet {
            at: Default::default(),
            bt: Default::default(),
            ct: Default::default(),
        }
    }
}

pub struct CircuitSynthesizer<E: PairingEngine> {
    // Constraints
    pub constraints: OptionalVec<ConstraintSet<E>>,

    // Assignments of variables
    pub public_variables: OptionalVec<E::Fr>,
    pub private_variables: OptionalVec<E::Fr>,

    // Technical namespaces used to remove of out-of-scope objects.
    pub namespaces: Vec<Namespace>,
}

impl<E: PairingEngine> Default for CircuitSynthesizer<E> {
    fn default() -> Self {
        Self {
            constraints: Default::default(),
            public_variables: Default::default(),
            private_variables: Default::default(),
            namespaces: Default::default(),
        }
    }
}

impl<E: PairingEngine> ConstraintSystem<E::Fr> for CircuitSynthesizer<E> {
    type Root = Self;

    #[inline]
    fn alloc<F, A, AR>(&mut self, _: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.private_variables.insert(f()?);
        if let Some(ref mut ns) = self.namespaces.last_mut() {
            ns.private_var_indices.push(index);
        }
        Ok(Variable::new_unchecked(Index::Private(index)))
    }

    #[inline]
    fn alloc_input<F, A, AR>(&mut self, _: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.public_variables.insert(f()?);
        if let Some(ref mut ns) = self.namespaces.last_mut() {
            ns.public_var_indices.push(index);
        }
        Ok(Variable::new_unchecked(Index::Public(index)))
    }

    #[inline]
    fn enforce<A, AR, LA, LB, LC>(&mut self, _: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<E::Fr>) -> LinearCombination<E::Fr>,
        LB: FnOnce(LinearCombination<E::Fr>) -> LinearCombination<E::Fr>,
        LC: FnOnce(LinearCombination<E::Fr>) -> LinearCombination<E::Fr>,
    {
        let index = self.constraints.insert(Default::default());

        push_constraints(a(LinearCombination::zero()), &mut self.constraints[index].at);
        push_constraints(b(LinearCombination::zero()), &mut self.constraints[index].bt);
        push_constraints(c(LinearCombination::zero()), &mut self.constraints[index].ct);

        if let Some(ref mut ns) = self.namespaces.last_mut() {
            ns.constraint_indices.push(index);
        }
    }

    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        self.namespaces.push(Namespace::default());
    }

    fn pop_namespace(&mut self) {
        // Todo @ljedrz: Fix constraint system optimizations.
        // if let Some(ns) = self.namespaces.pop() {
        //     for idx in ns.constraint_indices {
        //         self.constraints.remove(idx);
        //     }
        //
        //     for idx in ns.private_var_indices {
        //         self.private_variables.remove(idx);
        //     }
        //
        //     for idx in ns.public_var_indices {
        //         self.public_variables.remove(idx);
        //     }
        // }
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    fn num_public_variables(&self) -> usize {
        self.public_variables.len()
    }

    fn num_private_variables(&self) -> usize {
        self.private_variables.len()
    }

    fn is_in_setup_mode(&self) -> bool {
        false
    }
}

fn push_constraints<F: Field>(l: LinearCombination<F>, constraint: &mut Vec<(F, Index)>) {
    for (var, coeff) in l.as_ref() {
        match var.get_unchecked() {
            Index::Public(i) => constraint.push((*coeff, Index::Public(i))),
            Index::Private(i) => constraint.push((*coeff, Index::Private(i))),
        }
    }
}
