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

use super::*;

impl<N: Network, B: BlockStorage<N>, P: ProgramStorage<N>> Ledger<N, B, P> {
    /// Returns `true` if the given state root exists.
    pub fn contains_state_root(&self, _state_root: &Field<N>) -> bool {
        todo!()
        // state_root == self.latest_state_root()
        //     || self.headers.values().any(|h| Header::previous_state_root(&h) == state_root)
    }

    /// Returns `true` if the given block hash exists.
    pub fn contains_block_hash(&self, block_hash: &N::BlockHash) -> Result<bool> {
        self.blocks.contains_block_hash(block_hash)
    }

    /// Returns `true` if the given block height exists.
    pub fn contains_block_height(&self, height: u32) -> Result<bool> {
        self.blocks.contains_block_height(height)
    }

    /// Returns `true` if the given program ID exists.
    pub fn contains_program_id(&self, program_id: &ProgramID<N>) -> Result<bool> {
        self.transactions.contains_program_id(program_id)
    }

    /// Returns `true` if the given transaction ID exists.
    pub fn contains_transaction_id(&self, transaction_id: &N::TransactionID) -> Result<bool> {
        self.transactions.contains_transaction_id(transaction_id)
    }

    /* Transition */

    /// Returns `true` if the given transition ID exists.
    pub fn contains_transition_id(&self, transition_id: &N::TransitionID) -> Result<bool> {
        self.transitions.contains_transition_id(transition_id)
    }

    /* Input */

    /// Returns `true` if the given input ID exists.
    pub fn contains_input_id(&self, input_id: &Field<N>) -> Result<bool> {
        self.transitions.contains_input_id(input_id)
    }

    /// Returns `true` if the given serial number exists.
    pub fn contains_serial_number(&self, serial_number: &Field<N>) -> Result<bool> {
        self.transitions.contains_serial_number(serial_number)
    }

    /// Returns `true` if the given tag exists.
    pub fn contains_tag(&self, tag: &Field<N>) -> Result<bool> {
        self.transitions.contains_tag(tag)
    }

    /* Output */

    /// Returns `true` if the given output ID exists.
    pub fn contains_output_id(&self, output_id: &Field<N>) -> Result<bool> {
        self.transitions.contains_output_id(output_id)
    }

    /// Returns `true` if the given commitment exists.
    pub fn contains_commitment(&self, commitment: &Field<N>) -> Result<bool> {
        self.transitions.contains_commitment(commitment)
    }

    /// Returns `true` if the given checksum exists.
    pub fn contains_checksum(&self, checksum: &Field<N>) -> bool {
        self.transitions.contains_checksum(checksum)
    }

    /// Returns `true` if the given nonce exists.
    pub fn contains_nonce(&self, nonce: &Group<N>) -> Result<bool> {
        self.transitions.contains_nonce(nonce)
    }

    /* Metadata */

    /// Returns `true` if the given transition public key exists.
    pub fn contains_tpk(&self, tpk: &Group<N>) -> Result<bool> {
        self.transitions.contains_tpk(tpk)
    }

    /// Returns `true` if the given transition commitment exists.
    pub fn contains_tcm(&self, tcm: &Field<N>) -> Result<bool> {
        self.transitions.contains_tcm(tcm)
    }
}