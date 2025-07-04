use blake3::Hash;
use risc0_zkvm::{ExecutorEnv, Receipt};
use verity_tls::tlsn_core::presentation::Presentation;

pub(crate) fn write_request(
    request: &Vec<Presentation>,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    write_request_3(request)
}

#[allow(dead_code)]
fn write_request_1(
    request: &Vec<Presentation>,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    let env = ExecutorEnv::builder().write(&request)?.build()?;

    Ok(env)
}

#[allow(dead_code)]
fn write_request_2(
    request: &Vec<Presentation>,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    let input = serde_json::to_string(&request)?;
    let input: &[u8] = input.as_bytes();

    let env = ExecutorEnv::builder().write(&input)?.build()?;

    Ok(env)
}

#[allow(dead_code)]
fn write_request_3(
    request: &Vec<Presentation>,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    let input_bytes = bincode::serialize(&request)?;

    let env = ExecutorEnv::builder()
        .write(&input_bytes.len())?
        .write_slice(&input_bytes)
        .build()?;

    Ok(env)
}

pub(crate) fn read_response(receipt: &Receipt) -> Result<Hash, Box<dyn std::error::Error>> {
    read_response_3(receipt)
}

#[allow(dead_code)]
fn read_response_1(receipt: &Receipt) -> Result<Hash, Box<dyn std::error::Error>> {
    let response = receipt.journal.decode::<Hash>()?;

    Ok(response)
}

#[allow(dead_code)]
fn read_response_3(receipt: &Receipt) -> Result<Hash, Box<dyn std::error::Error>> {
    let response = bincode::deserialize(&receipt.journal.bytes)?;

    Ok(response)
}
