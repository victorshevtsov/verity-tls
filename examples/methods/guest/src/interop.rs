use blake3::Hash;
use risc0_zkvm::guest::env;
use verity_tls::tlsn_core::presentation::Presentation;

pub(crate) fn read_request() -> Result<Vec<Presentation>, Box<dyn std::error::Error>> {
    read_request_3()
}

#[allow(dead_code)]
fn read_request_1() -> Result<Vec<Presentation>, Box<dyn std::error::Error>> {
    let request = env::read();

    Ok(request)
}

#[allow(dead_code)]
fn read_request_2() -> Result<Vec<Presentation>, Box<dyn std::error::Error>> {
    let input_bytes: Vec<u8> = env::read();

    let params: String = String::from_utf8(input_bytes)?;
    let request: Vec<Presentation> = serde_json::from_str(params.as_str())?;

    Ok(request)
}

#[allow(dead_code)]
fn read_request_3() -> Result<Vec<Presentation>, Box<dyn std::error::Error>> {
    let len: usize = env::read();

    let mut input_bytes = vec![0u8; len];
    env::read_slice(&mut input_bytes);

    let request = bincode::deserialize(&input_bytes)?;

    Ok(request)
}

pub fn commit_response(response: &Hash) -> Result<(), Box<dyn std::error::Error>> {
    commit_response_3(response)
}

#[allow(dead_code)]
fn commit_response_1(response: &Hash) -> Result<(), Box<dyn std::error::Error>> {
    env::commit(response);

    Ok(())
}

#[allow(dead_code)]
fn commit_response_3(response: &Hash) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bincode::serialize(&response)?;

    env::commit_slice(&bytes);

    Ok(())
}
