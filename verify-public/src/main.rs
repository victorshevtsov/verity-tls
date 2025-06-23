use verity_tls::tlsn_core::{presentation::Presentation, CryptoProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let presentation: Presentation =
        serde_json::from_str(fixtures::proof::PRESENTATION_32B_PUBLIC)?;
    let presentation_output = presentation.verify(&CryptoProvider::default())?;

    println!("server_name: {:?}", &presentation_output.server_name);
    println!(
        "connection_info: {:?}",
        &presentation_output.connection_info
    );

    Ok(())
}
