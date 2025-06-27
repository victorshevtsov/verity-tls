use risc0_zkvm::guest::env;
use verity_tls::{Request, Response};

pub(crate) fn read_request() -> Result<Request, Box<dyn std::error::Error>> {
    read_request_3()
}

#[allow(dead_code)]
fn read_request_1() -> Result<Request, Box<dyn std::error::Error>> {
    let request = env::read();

    Ok(request)
}

#[allow(dead_code)]
fn read_request_2() -> Result<Request, Box<dyn std::error::Error>> {
    let input_bytes: Vec<u8> = env::read();

    let params: String = String::from_utf8(input_bytes)?;
    let request: Request = serde_json::from_str(params.as_str())?;

    Ok(request)
}

#[allow(dead_code)]
fn read_request_3() -> Result<Request, Box<dyn std::error::Error>> {
    let len: usize = env::read();

    let mut input_bytes = vec![0u8; len];
    env::read_slice(&mut input_bytes);

    let request = bincode::deserialize(&input_bytes)?;

    Ok(request)
}

pub fn commit_response(response: &Response) -> Result<(), Box<dyn std::error::Error>> {
    commit_response_3(response)
}

#[allow(dead_code)]
fn commit_response_1(response: &Response) -> Result<(), Box<dyn std::error::Error>> {
    env::commit(response);

    Ok(())
}

#[allow(dead_code)]
fn commit_response_3(response: &Response) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bincode::serialize(&response)?;

    env::commit_slice(&bytes);

    Ok(())
}
