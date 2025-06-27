use blake3::{Hash, Hasher};

use crate::tlsn_core::presentation::Presentation;

pub mod tlsn_core;

pub fn hash_presentations(presentations: &Vec<Presentation>) -> Hash {
    let mut hasher = Hasher::new();

    for presentation in presentations {
        let bytes = bincode::serialize(&presentation).unwrap();
        hasher.update(&bytes);
    }

    hasher.finalize()
}
