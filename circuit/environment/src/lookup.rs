// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    BaseConstraint,
    Circuit,
    Constraint,
    Environment,
    LinearCombination,
    Mode,
    Variable,
    CIRCUIT,
    IN_WITNESS,
    LOOKUP_TABLES,
};

use snarkvm_r1cs::LookupTable;

pub trait Lookup: Environment {
    fn add_lookup_table(lookup_table: LookupTable<Self::BaseField>) -> usize;

    fn unary_lookup<A: Into<LinearCombination<Self::BaseField>>>(
        id: usize,
        key: A,
    ) -> (Variable<Self::BaseField>, Variable<Self::BaseField>);

    fn binary_lookup<A: Into<LinearCombination<Self::BaseField>>, B: Into<LinearCombination<Self::BaseField>>>(
        id: usize,
        key_1: A,
        key_2: B,
    ) -> Variable<Self::BaseField>;

    fn index_lookup(
        id: usize,
        index: usize,
    ) -> (Variable<Self::BaseField>, Variable<Self::BaseField>, Variable<Self::BaseField>);

    fn enforce_lookup<Fn, A, B, C>(constraint: Fn)
    where
        Fn: FnOnce() -> (A, B, C),
        A: Into<LinearCombination<Self::BaseField>>,
        B: Into<LinearCombination<Self::BaseField>>,
        C: Into<LinearCombination<Self::BaseField>>;
}

impl Lookup for Circuit {
    fn add_lookup_table(lookup_table: LookupTable<Self::BaseField>) -> usize {
        LOOKUP_TABLES.with(|lookup_tables| {
            let lookup_tables = &mut *(**lookup_tables).borrow_mut();
            lookup_tables.push(lookup_table);
            lookup_tables.len() - 1
        })
    }

    fn unary_lookup<A: Into<LinearCombination<Self::BaseField>>>(
        id: usize,
        key: A,
    ) -> (Variable<Self::BaseField>, Variable<Self::BaseField>) {
        let val = key.into().value();
        let (a, b) = LOOKUP_TABLES.with(|lookup_tables| {
            let lookup_tables = &*(**lookup_tables).borrow();
            let row = lookup_tables[id].table.iter().find(|row| row.0 == val).unwrap();
            (row.1, row.2)
        });

        (Self::new_variable(Mode::Private, a), Self::new_variable(Mode::Private, b))
    }

    fn binary_lookup<A: Into<LinearCombination<Self::BaseField>>, B: Into<LinearCombination<Self::BaseField>>>(
        id: usize,
        key_1: A,
        key_2: B,
    ) -> Variable<Self::BaseField> {
        let val_1 = key_1.into().value();
        let val_2 = key_2.into().value();
        let a = LOOKUP_TABLES.with(|lookup_tables| {
            let lookup_tables = &*(**lookup_tables).borrow();
            let row = lookup_tables[id].table.iter().find(|row| row.0 == val_1 && row.1 == val_2).unwrap();
            row.2
        });

        Self::new_variable(Mode::Private, a)
    }

    fn index_lookup(
        id: usize,
        index: usize,
    ) -> (Variable<Self::BaseField>, Variable<Self::BaseField>, Variable<Self::BaseField>) {
        let (a, b, c) = LOOKUP_TABLES.with(|lookup_tables| {
            let lookup_tables = &*(**lookup_tables).borrow();
            let row = lookup_tables[id].table[index];
            (row.0, row.1, row.2)
        });

        (
            Self::new_variable(Mode::Private, a),
            Self::new_variable(Mode::Private, b),
            Self::new_variable(Mode::Private, c),
        )
    }

    fn enforce_lookup<Fn, A, B, C>(constraint: Fn)
    where
        Fn: FnOnce() -> (A, B, C),
        A: Into<LinearCombination<Self::BaseField>>,
        B: Into<LinearCombination<Self::BaseField>>,
        C: Into<LinearCombination<Self::BaseField>>,
    {
        IN_WITNESS.with(|in_witness| {
            // Ensure we are not in witness mode.
            if !(*(**in_witness).borrow()) {
                CIRCUIT.with(|circuit| {
                    let (a, b, c) = constraint();
                    let (a, b, c) = (a.into(), b.into(), c.into());

                    // Construct the constraint object.
                    // TODO: fix right table index
                    let constraint =
                        Constraint::LookupConstraint(BaseConstraint((**circuit).borrow().scope(), a, b, c), 0);
                    // Append the constraint.
                    (**circuit).borrow_mut().enforce_lookup(constraint)
                });
            }
        })
    }
}