use risc0_zkvm::guest::env;
use verity_tls::{
    merkle::generate_merkle_tree,
    tlsn_core::{presentation::Presentation, CryptoProvider},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the input
    // let presentation = read_input_1();
    // let presentation = read_input_2()?;
    let private_presentation = read_input_3()?;

    let public_presentation = private_presentation.clone().wipe_private_data()?;

    private_presentation.verify_private_facets(&CryptoProvider::default())?;

    let leaf = serde_json::to_string(&public_presentation)?;

    let merkle_tree = generate_merkle_tree(&vec![leaf]);
    let root = merkle_tree.root().unwrap();
    let root_hex = hex::encode(root);

    env::commit(&root_hex);

    Ok(())
}

#[allow(dead_code)]
fn read_input_1() -> Presentation {
    let presentation = env::read();
    presentation
}

#[allow(dead_code)]
fn read_input_2() -> Result<Presentation, Box<dyn std::error::Error>> {
    let input_bytes: Vec<u8> = env::read();

    let params: String = String::from_utf8(input_bytes)?;
    let presentation: Presentation = serde_json::from_str(params.as_str())?;

    Ok(presentation)
}

#[allow(dead_code)]
fn read_input_3() -> Result<Presentation, Box<dyn std::error::Error>> {
    let len: usize = env::read();
    println!("Guest is reading input_bytes: {}", len);

    let mut input_bytes = vec![0u8; len];
    env::read_slice(&mut input_bytes);

    let presentation = bincode::deserialize(&input_bytes)?;

    Ok(presentation)
}
