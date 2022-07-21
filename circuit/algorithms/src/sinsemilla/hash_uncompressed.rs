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

use std::borrow::Cow;

impl<E: Lookup + Environment, const NUM_WINDOWS: u8> HashUncompressed for Sinsemilla<E, NUM_WINDOWS> {
    type Input = Boolean<E>;
    type Output = Group<E>;

    fn hash_uncompressed(&self, input: &[Self::Input]) -> Self::Output {
        // Ensure the input size is within the size bounds.
        let mut input = Cow::Borrowed(input);
        let max_len = console::SINSEMILLA_WINDOW_SIZE * NUM_WINDOWS as usize;
        match input.len() <= max_len {
            // Pad the input if it is under the required parameter size.
            true => input.to_mut().resize(max_len, Boolean::constant(false)),
            // Ensure the input size is within the parameter size.
            false => E::halt(format!("The Sinsemilla hash input cannot exceed {} bits.", max_len)),
        }

        input.chunks(console::SINSEMILLA_WINDOW_SIZE).fold(self.q.clone(), |acc, bits| {
            // Recover the bit window as a native integer value so we can index into the lookup table.
            let i = Field::from_bits_le(bits);
            let (s_x, s_y) = E::unary_lookup(0, i);
            let s = Group::from_xy_coordinates(
                Field::from(LinearCombination::from(s_x)),
                Field::from(LinearCombination::from(s_y)),
            );
            acc.double() + s
        })
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use snarkvm_algorithms::{crh::SinsemillaCRH, CRH};
    use snarkvm_circuits_environment::{assert_count, assert_output_mode, Circuit};
    use snarkvm_curves::AffineCurve;
    use snarkvm_utilities::{test_rng, UniformRand};

    const ITERATIONS: u64 = 10;
    const MESSAGE: &str = "SinsemillaCircuit0";
    const WINDOW_SIZE_MULTIPLIER: usize = 8;

    type Projective = <<Circuit as Environment>::Affine as AffineCurve>::Projective;

    fn check_hash_uncompressed<const NUM_WINDOWS: usize, const WINDOW_SIZE: usize>(mode: Mode) {
        // Initialize the Sinsemilla hash.
        let native = SinsemillaCRH::<Projective, NUM_WINDOWS, WINDOW_SIZE>::setup(MESSAGE);
        let circuit = Sinsemilla::<Circuit, NUM_WINDOWS, WINDOW_SIZE>::setup(MESSAGE);
        // Determine the number of inputs.
        let num_input_bits = NUM_WINDOWS * WINDOW_SIZE;

        for i in 0..ITERATIONS {
            // Sample a random input.
            let input = (0..num_input_bits).map(|_| bool::rand(&mut test_rng())).collect::<Vec<bool>>();
            // Compute the expected hash.
            let expected = native.hash(&input).expect("Failed to hash native input");
            // Prepare the circuit input.
            let circuit_input: Vec<Boolean<_>> = Inject::new(mode, input);

            Circuit::scope(format!("Sinsemilla {mode} {i}"), || {
                // Perform the hash operation.
                let candidate = circuit.hash_uncompressed(&circuit_input);
                println!("{}", Circuit::num_constraints_in_scope());
                println!("{}", Circuit::num_constants_in_scope());
                assert_eq!(Circuit::affine_from_x_coordinate(expected), candidate.eject_value());
            });
        }
    }

    #[test]
    fn test_hash_uncompressed_constant() {
        // Set the number of windows, and modulate the window size.
        check_hash_uncompressed::<10, 52>(Mode::Constant);
    }

    #[test]
    fn test_hash_uncompressed_public() {
        // Set the number of windows, and modulate the window size.
        check_hash_uncompressed::<10, 52>(Mode::Public);
    }

    #[test]
    fn test_hash_uncompressed_private() {
        // Set the number of windows, and modulate the window size.
        check_hash_uncompressed::<10, 52>(Mode::Private);
    }
    */
}