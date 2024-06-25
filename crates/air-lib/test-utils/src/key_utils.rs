/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use air_interpreter_signatures::KeyPair;
use rand_chacha::rand_core::SeedableRng;

///  Derive fake keypair for testing proposes.
///
///  This function should be used in production, but it is yet.
///  It returns a keypair determinisitically derived from seed, and a corresponding peer ID
///  that might be useful in tests.
// Should be moved to test lib when keypair interface PR is merged.
pub fn derive_dummy_keypair(seed: &str) -> (KeyPair, String) {
    use sha2::{Digest as _, Sha256};

    let mut rng = {
        let mut hasher = Sha256::new();
        hasher.update(seed);
        rand_chacha::ChaCha8Rng::from_seed(hasher.finalize().into())
    };

    let keypair_ed25519 = ed25519_dalek::SigningKey::generate(&mut rng);
    let keypair = fluence_keypair::KeyPair::Ed25519(keypair_ed25519.into());
    let keypair = KeyPair::try_from(keypair).expect("cannot happen");

    let peer_id = keypair.public().to_peer_id().unwrap().to_string();
    (keypair, peer_id)
}

pub fn at(peer_name: &str) -> String {
    derive_dummy_keypair(peer_name).1
}
