use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

pub fn generate_merkle_leaves(leaves: &Vec<String>) -> Vec<[u8; 32]> {
    leaves
        .iter()
        .map(|leaf| Sha256::hash(leaf.as_bytes()))
        .collect()
}

pub fn generate_merkle_tree(leaves: &Vec<String>) -> MerkleTree<Sha256> {
    let leaves = generate_merkle_leaves(leaves);

    let tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);
    return tree;
}

pub fn validate_merkle_tree(leaves: &Vec<String>, root_hash: &String) -> bool {
    let merkle_tree: MerkleTree<Sha256> = generate_merkle_tree(leaves);

    let derived_root_hash = merkle_tree.root().unwrap();
    let derived_root_hash = hex::encode(derived_root_hash);

    return derived_root_hash.eq(root_hash);
}
