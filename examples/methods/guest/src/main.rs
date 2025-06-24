use risc0_zkvm::guest::env;
use verity_tls::tlsn_core::{presentation::Presentation, CryptoProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the input
    // let presentation = read_input_1();
    // let presentation = read_input_2()?;
    let presentation = read_input_3()?;

    let presentation_output = presentation.verify_private_facets(&CryptoProvider::default())?;
    let mut transcript = presentation_output.transcript.ok_or("no transcript")?;

    transcript.set_unauthed(b'X');

    let sent = String::from_utf8(transcript.sent_unsafe().to_vec())?;
    let received = String::from_utf8(transcript.received_unsafe().to_vec())?;

    println!("{}", sent);
    println!("{}", received);

    // write public output to the journal
    env::commit(&sent);
    env::commit(&received);

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
