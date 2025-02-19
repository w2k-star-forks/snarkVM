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

mod bytes;
mod serialize;
mod string;
mod to_bits;

use console::{
    network::prelude::*,
    program::{Identifier, ProgramID},
    types::Field,
};

/// The Merkle leaf for a function or transition in the transaction.
#[derive(Clone, PartialEq, Eq)]
pub struct TransactionLeaf<N: Network> {
    /// The variant of the Merkle leaf.
    variant: u8,
    /// The index of the Merkle leaf.
    index: u16,
    /// The program ID.
    program_id: ProgramID<N>,
    /// The function name.
    function_name: Identifier<N>,
    /// The ID.
    id: Field<N>,
}

impl<N: Network> TransactionLeaf<N> {
    /// Initializes a new instance of `TransactionLeaf`.
    pub const fn new(
        variant: u8,
        index: u16,
        program_id: ProgramID<N>,
        function_name: Identifier<N>,
        id: Field<N>,
    ) -> Self {
        Self { variant, index, program_id, function_name, id }
    }

    /// Returns the variant of the Merkle leaf.
    pub const fn variant(&self) -> u8 {
        self.variant
    }

    /// Returns the index of the Merkle leaf.
    pub const fn index(&self) -> u16 {
        self.index
    }

    /// Returns the program ID in the Merkle leaf.
    pub const fn program_id(&self) -> ProgramID<N> {
        self.program_id
    }

    /// Returns the function name in the Merkle leaf.
    pub const fn function_name(&self) -> Identifier<N> {
        self.function_name
    }

    /// Returns the ID in the Merkle leaf.
    pub const fn id(&self) -> Field<N> {
        self.id
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    pub(super) fn sample_leaf(rng: &mut TestRng) -> TransactionLeaf<CurrentNetwork> {
        // Construct a new leaf.
        TransactionLeaf::new(
            rng.gen(),
            rng.gen(),
            FromStr::from_str("hello.aleo").unwrap(),
            FromStr::from_str("runner").unwrap(),
            Uniform::rand(rng),
        )
    }
}
