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
use super::*;

macro_rules! tuple_append {
    ($id1: ident, $id2:ident, (((((((((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        ($id1.0.append($id2.0),)
    };
    ($id1: ident, $id2:ident, ((((((((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        ($id1.0.append($id2.0), $id1.1.append($id2.1))
    };
    ($id1:ident, $id2:ident, (((((((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        ($id1.0.append($id2.0), $id1.1.append($id2.1), $id1.2.append($id2.2))
    };
    ($id1:ident, $id2:ident, ((((((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
        )
    };
    ($id1:ident, $id2:ident, (((((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
        )
    };
    ($id1:ident, $id2:ident, ((((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
        )
    };
    ($id1:ident, $id2: ident, (((((12u32 - 1u32) - 1u32) - 1u32) - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
            $id1.6.append($id2.6),
        )
    };
    ($id1:ident, $id2: ident, ((((12u32 - 1u32) - 1u32) - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
            $id1.6.append($id2.6),
            $id1.7.append($id2.7),
        )
    };
    ($id1:ident, $id2:ident, (((12u32 - 1u32) - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
            $id1.6.append($id2.6),
            $id1.7.append($id2.7),
            $id1.8.append($id2.8),
        )
    };
    ($id1:ident, $id2:ident, ((12u32 - 1u32) - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
            $id1.6.append($id2.6),
            $id1.7.append($id2.7),
            $id1.8.append($id2.8),
            $id1.9.append($id2.9),
        )
    };
    ($id1:ident, $id2:ident, (12u32 - 1u32)) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
            $id1.6.append($id2.6),
            $id1.7.append($id2.7),
            $id1.8.append($id2.8),
            $id1.9.append($id2.9),
            $id1.10.append($id2.10),
        )
    };
    ($id1:ident, $id2: ident, 12u32) => {
        (
            $id1.0.append($id2.0),
            $id1.1.append($id2.1),
            $id1.2.append($id2.2),
            $id1.3.append($id2.3),
            $id1.4.append($id2.4),
            $id1.5.append($id2.5),
            $id1.6.append($id2.6),
            $id1.7.append($id2.7),
            $id1.8.append($id2.8),
            $id1.9.append($id2.9),
            $id1.10.append($id2.10),
            $id1.11.append($id2.11),
        )
    };
}

macro_rules! tuple_monoid_impls {
    (@count $i:tt, $head:ident, $( $tail:ident, )* ) => {
        impl<$head, $( $tail ),*> Monoid for ($head, $( $tail ),*)
        where
            $head: Monoid,
            $( $tail: Monoid),*
        {

            fn append(self, other: ($head, $( $tail ),*)) -> ($head, $( $tail ),*) {

                tuple_append!(self, other, $i)
            }

            fn append_all(self, others: impl Iterator<Item = ($head, $( $tail),* )>) -> ($head, $( $tail ),*) {
                others.fold(self, |acc, tup| acc.append(tup))
            }

        }
        tuple_monoid_impls!(@count ($i - 1u32), $( $tail, )*);
    };
    (@count $i:tt,) => {};
}

tuple_monoid_impls!(@count 12u32, A, B, C, D, E, F, G, H, I, J, K, L,);
