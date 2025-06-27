use risc0_zkvm::guest::env;
use verity_tls::tlsn_core::presentation::Presentation;

pub(crate) fn read_input() -> Result<Presentation, Box<dyn std::error::Error>> {
    read_input_3()
}

#[allow(dead_code)]
fn read_input_1() -> Result<Presentation, Box<dyn std::error::Error>> {
    let presentation = env::read();

    Ok(presentation)
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
