/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
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

    let keypair_ed25519 = ed25519_dalek::Keypair::generate(&mut rng);
    let keypair = fluence_keypair::KeyPair::Ed25519(keypair_ed25519.into());
    let keypair = KeyPair::try_from(keypair).expect("cannot happen");

    let peer_id = keypair.public().to_peer_id().to_string();
    (keypair, peer_id)
}

pub fn at(peer_name: &str) -> String {
    derive_dummy_keypair(peer_name).1
}
