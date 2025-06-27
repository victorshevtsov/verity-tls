use blake3::Hash;
use serde::{Deserialize, Serialize};

use crate::tlsn_core::presentation::Presentation;

pub mod tlsn_core;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Request {
    pub items: Vec<Presentation>,
}

impl Request {
    pub fn push(&mut self, item: Presentation) {
        self.items.push(item);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub items: Vec<Presentation>,
    pub hash: Hash,
}

impl From<Vec<Presentation>> for Response {
    fn from(items: Vec<Presentation>) -> Self {
        let mut hasher = blake3::Hasher::new();

        for item in &items {
            let bytes = bincode::serialize(&item).unwrap();
            hasher.update(&bytes);
        }

        Self {
            items,
            hash: hasher.finalize(),
        }
    }
}
