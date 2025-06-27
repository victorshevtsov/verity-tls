use blake3::Hash;
use verity_tls::{
    hash_presentations,
    tlsn_core::{presentation::Presentation, CryptoProvider},
};

mod interop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = interop::read_request()?;

    let response = process_request(request)?;

    interop::commit_response(&response)?;

    Ok(())
}

fn process_request(presentations: Vec<Presentation>) -> Result<Hash, Box<dyn std::error::Error>> {
    let mut public_presentations = Vec::<Presentation>::new();

    for presentation in presentations.into_iter() {
        public_presentations.push(presentation.clone().wipe_private_data()?);

        presentation.verify_private_facets(&CryptoProvider::default())?;
    }

    Ok(hash_presentations(&public_presentations))
}
