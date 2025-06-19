use verity_tls::tlsn_core::presentation::Presentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let presentation: Presentation = serde_json::from_str(fixtures::proof::PRESENTATION_1KB)?;

    println!("{:?}", presentation);

    Ok(())
}
