use risc0_zkvm::guest::env;
use verity_tls::tlsn_core::{presentation::Presentation, CryptoProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the input
    let input: u32 = env::read();

    let presentation: Presentation = serde_json::from_str(fixtures::proof::PRESENTATION_1KB)?;

    let presentation_output = presentation.verify(&CryptoProvider::default())?;
    let mut transcript = presentation_output.transcript.ok_or("no transcript")?;

    transcript.set_unauthed(b'X');

    let sent = String::from_utf8(transcript.sent_unsafe().to_vec())?;
    let received = String::from_utf8(transcript.received_unsafe().to_vec())?;

    println!("{}", sent);
    println!("{}", received);

    // write public output to the journal
    env::commit(&input);

    Ok(())
}
