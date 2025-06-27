use risc0_zkvm::guest::env;
use verity_tls::{merkle::generate_merkle_tree, tlsn_core::CryptoProvider};

mod interop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the input
    let private_presentation = interop::read_input()?;

    let mut public_presentation = private_presentation.clone();
    public_presentation = public_presentation.wipe_private_data()?;
    public_presentation = public_presentation.wipe_public_data()?;
    public_presentation = public_presentation.wipe_precomputed_encodings()?;

    private_presentation.verify_private_facets(&CryptoProvider::default())?;

    let leaf = serde_json::to_string(&public_presentation)?;

    let merkle_tree = generate_merkle_tree(&vec![leaf]);
    let root = merkle_tree.root().unwrap();
    let root_hex = hex::encode(root);

    env::commit(&root_hex);
    env::commit(&serde_json::to_string(&public_presentation)?);
    env::commit(&bincode::serialize(&public_presentation)?);

    Ok(())
}
